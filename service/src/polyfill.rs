use crate::config::ServiceConfig;
use crate::routes::resp;
use crate::UnknownUaTelemetry;
use axum::http::StatusCode;
use axum::response::Response;
use polyfill_library::ua::{UA, UserAgent};
use polyfill_library::{
    buffer::Buffer, get_polyfill_string::get_polyfill_string_stream,
    polyfill_parameters::PolyfillParameters, Env,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;

/// How many unknown-UA requests pass between sampled log lines.
const UNKNOWN_UA_LOG_SAMPLE: u64 = 100;

pub(crate) async fn polyfill(
    user_agent: Option<&str>,
    minify: bool,
    env: Arc<Env>,
    config: &ServiceConfig,
    telemetry: &UnknownUaTelemetry,
) -> Response {
    let ua_string = user_agent.unwrap_or_default();

    // Unknown UAs get the configured `unknown` policy instead of targeted
    // bundles. Counting and sampling them lets production traffic surface
    // misclassified real browsers (the iOS-webview and YaBrowser parser
    // bugs would have shown up here as floods of plausible UAs).
    if UA::new(ua_string).is_unknown().unwrap_or(true) {
        telemetry.metric.inc();
        let seen = telemetry.sample_counter.fetch_add(1, Ordering::Relaxed);
        if seen.is_multiple_of(UNKNOWN_UA_LOG_SAMPLE) {
            tracing::info!(
                "unknown user agent (1/{UNKNOWN_UA_LOG_SAMPLE} sample): {ua_string:?}"
            );
        }
    }

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
    match get_polyfill_string_stream(&mut res_body, &parameters, env, &config.version).await {
        Ok(()) => resp(
            StatusCode::OK,
            &[
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "GET,HEAD,OPTIONS"),
                ("Content-Type", "text/javascript; charset=UTF-8"),
                ("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable"),
                // We need "Vary: User-Agent" in the browser cache because a browser
                // may update itself to a version which needs different polyfills
                // So we need to have it ignore the browser cached bundle when the user-agent changes.
                ("Vary", "User-Agent, Accept-Encoding"),
                ("X-Polyfill-Version", &config.version),
            ],
            res_body.into_str(),
        ),
        Err(err) => {
            tracing::error!("failed to build polyfill bundle: {err}");
            resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                &[],
                "Internal Server Error\n",
            )
        }
    }
}
