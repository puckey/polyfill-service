use crate::routes::resp;
use axum::http::StatusCode;
use axum::response::Response;
use polyfill_library::{
    buffer::Buffer, get_polyfill_string::get_polyfill_string_stream,
    polyfill_parameters::get_polyfill_parameters, Env,
};
use std::sync::Arc;

const SUPPORTED_VERSIONS: &[&str] = &[
    "3.101.0", "3.103.0", "3.104.0", "3.108.0", "3.109.0", "3.110.1", "3.111.0", "3.25.1",
    "3.27.4", "3.34.0", "3.39.0", "3.40.0", "3.41.0", "3.42.0", "3.46.0", "3.48.0", "3.50.2",
    "3.51.0", "3.52.0", "3.52.1", "3.52.2", "3.52.3", "3.53.1", "3.89.4", "3.96.0", "3.98.0",
    "4.8.0", "5.3.1",
];

fn parse_library_version(version: &str) -> String {
    if SUPPORTED_VERSIONS.contains(&version) {
        version.to_owned()
    } else {
        tracing::warn!("unknown version: {version}, using fallback.");
        "3.111.0".to_owned() // fallback to default version
    }
}

pub(crate) async fn polyfill(url: &url::Url, user_agent: Option<&str>, env: Arc<Env>) -> Response {
    let parameters = get_polyfill_parameters(url, user_agent);

    let version = parse_library_version(&parameters.version);

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
