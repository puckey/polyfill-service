use crate::routes::resp;
use crate::StoreVersions;
use axum::http::StatusCode;
use axum::response::Response;
use polyfill_library::{
    buffer::Buffer, get_polyfill_string::get_polyfill_string_stream,
    polyfill_parameters::get_polyfill_parameters, Env,
};
use std::sync::Arc;

fn resolve_library_version(version: &str, versions: &StoreVersions) -> String {
    if versions.available.iter().any(|v| v == version) {
        version.to_owned()
    } else {
        tracing::warn!(
            "requested version {version} is not in the polyfill store, using {}",
            versions.fallback
        );
        versions.fallback.clone()
    }
}

pub(crate) async fn polyfill(
    url: &url::Url,
    user_agent: Option<&str>,
    env: Arc<Env>,
    versions: &StoreVersions,
) -> Response {
    let parameters = get_polyfill_parameters(url, user_agent);

    let version = resolve_library_version(&parameters.version, versions);

    let mut res_body = Buffer::new();
    match get_polyfill_string_stream(&mut res_body, &parameters, env, &version).await {
        Ok(()) => resp(
            StatusCode::OK,
            &[
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "GET,HEAD,OPTIONS"),
                ("X-Compress-Hint", "on"),
                ("Content-Type", "text/javascript; charset=UTF-8"),
                ("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable"),
                // We need "Vary: User-Agent" in the browser cache because a browser
                // may update itself to a version which needs different polyfills
                // So we need to have it ignore the browser cached bundle when the user-agent changes.
                ("Vary", "User-Agent, Accept-Encoding"),
                ("X-Polyfill-Version", &version),
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
