use crate::UnknownUaTelemetry;
use crate::bundle_cache::{BundleCache, CacheKey, CachedBundle};
use crate::config::ServiceConfig;
use axum::http::StatusCode;
use axum::response::Response;
use polyfill_library::ua::{UA, UserAgent};
use polyfill_library::{
    Env, buffer::Buffer, get_polyfill_string::get_polyfill_string_stream,
    polyfill_parameters::PolyfillParameters,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;

/// How many unknown-UA requests pass between sampled log lines.
const UNKNOWN_UA_LOG_SAMPLE: u64 = 100;

/// The cache key for user agents the parser cannot classify. They all
/// receive the configured `unknown` policy, so they share one bundle.
fn unknown_key(minify: bool) -> CacheKey {
    CacheKey {
        family: "unknown".to_owned(),
        version: String::new(),
        minify,
    }
}

pub(crate) async fn polyfill(
    user_agent: Option<&str>,
    accept_encoding: Option<&str>,
    minify: bool,
    env: Arc<Env>,
    config: &ServiceConfig,
    cache: &BundleCache,
    telemetry: &UnknownUaTelemetry,
) -> Response {
    let ua_string = user_agent.unwrap_or_default();

    // Unknown UAs get the configured `unknown` policy instead of targeted
    // bundles. Counting and sampling them lets production traffic surface
    // misclassified real browsers (the iOS-webview and YaBrowser parser
    // bugs would have shown up here as floods of plausible UAs).
    let ua = UA::new(ua_string);
    let unknown = ua.is_unknown().unwrap_or(true);
    if unknown {
        telemetry.metric.inc();
        let seen = telemetry.sample_counter.fetch_add(1, Ordering::Relaxed);
        if seen.is_multiple_of(UNKNOWN_UA_LOG_SAMPLE) {
            tracing::info!("unknown user agent (1/{UNKNOWN_UA_LOG_SAMPLE} sample): {ua_string:?}");
        }
    }

    // Bundles are pure functions of (config, UA bucket, minify) and the
    // config is fixed for the process lifetime, so the normalized UA is a
    // complete cache key.
    let key = if unknown {
        unknown_key(minify)
    } else {
        CacheKey {
            family: ua.get_family(),
            version: ua.get_version().to_owned(),
            minify,
        }
    };

    if let Some(bundle) = cache.get(&key) {
        cache.hits.inc();
        return serve(&bundle, accept_encoding, config, &env);
    }
    cache.misses.inc();

    match build_bundle(ua_string, minify, Arc::clone(&env), config).await {
        Ok(bundle) => {
            let bundle = Arc::new(bundle);
            cache.insert(key, Arc::clone(&bundle));
            serve(&bundle, accept_encoding, config, &env)
        }
        Err(err) => {
            tracing::error!("failed to build polyfill bundle: {err}");
            crate::routes::resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                &[],
                "Internal Server Error\n",
            )
        }
    }
}

/// Builds the bundle for a UA string and compresses its variants. Used by
/// the request path on cache miss and by startup warming. Errors are
/// stringified at the boundary: the library's `BoxError` is not `Send`,
/// which would make the handler future non-`Send`.
pub(crate) async fn build_bundle(
    ua_string: &str,
    minify: bool,
    env: Arc<Env>,
    config: &ServiceConfig,
) -> Result<CachedBundle, String> {
    let parameters = PolyfillParameters {
        excludes: config.excludes.clone(),
        features: config.features.clone(),
        minify,
        callback: None,
        unknown: config.unknown.clone(),
        ua_string: ua_string.to_owned(),
        version: config.version.clone(),
        strict: false,
    };

    let mut res_body = Buffer::new();
    get_polyfill_string_stream(&mut res_body, &parameters, env, &config.version)
        .await
        .map_err(|err| err.to_string())?;
    let mut bytes = Vec::new();
    res_body
        .read_to_end(&mut bytes)
        .map_err(|err| err.to_string())?;
    Ok(CachedBundle::new(bytes))
}

/// Warms the shared unknown-UA entry so the most expensive bundle is never
/// built on a request. Called once at startup.
pub(crate) async fn warm_unknown_bundle(
    env: Arc<Env>,
    config: &ServiceConfig,
    cache: &BundleCache,
) {
    for minify in [true, false] {
        match build_bundle("", minify, Arc::clone(&env), config).await {
            Ok(bundle) => cache.insert(unknown_key(minify), Arc::new(bundle)),
            Err(err) => panic!("failed to build the unknown-UA bundle: {err}"),
        }
    }
}

fn serve(
    bundle: &CachedBundle,
    accept_encoding: Option<&str>,
    config: &ServiceConfig,
    env: &Env,
) -> Response {
    let (encoding, body) = bundle.pick(accept_encoding);
    env.bytes_out_metric.inc_by(body.len() as u64);
    let mut headers: Vec<(&str, &str)> = vec![
        ("Access-Control-Allow-Origin", "*"),
        ("Access-Control-Allow-Methods", "GET,HEAD,OPTIONS"),
        ("Content-Type", "text/javascript; charset=UTF-8"),
        (
            "Cache-Control",
            "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable",
        ),
        // We need "Vary: User-Agent" in the browser cache because a browser
        // may update itself to a version which needs different polyfills
        // So we need to have it ignore the browser cached bundle when the user-agent changes.
        ("Vary", "User-Agent, Accept-Encoding"),
        ("X-Polyfill-Version", &config.version),
    ];
    if let Some(encoding) = encoding {
        headers.push(("Content-Encoding", encoding));
    }
    crate::routes::resp(StatusCode::OK, &headers, body.to_vec())
}
