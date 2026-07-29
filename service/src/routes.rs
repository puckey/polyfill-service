use crate::pages::home;
use crate::polyfill::polyfill;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Method, StatusCode};
use axum::response::Response;

pub(crate) fn resp<B: Into<Body>>(
    status: StatusCode,
    headers: &[(&str, &str)],
    body: B,
) -> Response {
    let mut builder = Response::builder().status(status);
    for (name, value) in headers {
        builder = builder.header(*name, *value);
    }
    builder.body(body.into()).expect("failed to build response")
}

pub async fn handle_request(State(state): State<AppState>, req: axum::extract::Request) -> Response {
    match *req.method() {
        Method::OPTIONS => {
            return resp(
                StatusCode::OK,
                &[
                    ("allow", "OPTIONS, GET, HEAD"),
                    ("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable"),
                ],
                "",
            );
        }
        Method::GET | Method::HEAD => {}
        _ => {
            return resp(
                StatusCode::METHOD_NOT_ALLOWED,
                &[("allow", "GET, HEAD")],
                "This method is not allowed\n",
            );
        }
    }

    let path = req.uri().path();
    // The bundle is defined entirely by the server's polyfill.toml — any
    // query parameters are ignored.
    match path {
        "/" => resp(
            StatusCode::OK,
            &[
                ("content-type", "text/html; charset=utf-8"),
                ("X-Content-Type-Options", "nosniff"),
                ("Referrer-Policy", "origin-when-cross-origin"),
                (
                    "Cache-Control",
                    "max-age=60, stale-while-revalidate=60, stale-if-error=86400",
                ),
            ],
            home(&state.config.version, &state.config.feature_list),
        ),
        "/robots.txt" => resp(StatusCode::OK, &[], "User-agent: *\nDisallow:"),
        "/polyfill.js" | "/v3/polyfill.js" => {
            let user_agent = user_agent(&req);
            polyfill(user_agent.as_deref(), false, state.env, &state.config, &state.unknown_ua).await
        }
        "/polyfill.min.js" | "/v3/polyfill.min.js" => {
            let user_agent = user_agent(&req);
            polyfill(user_agent.as_deref(), true, state.env, &state.config, &state.unknown_ua).await
        }
        _ => resp(
            StatusCode::NOT_FOUND,
            &[("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable")],
            format!("{path}: Not Found"),
        ),
    }
}

fn user_agent(req: &axum::extract::Request) -> Option<String> {
    req.headers()
        .get(header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .map(std::borrow::ToOwned::to_owned)
}
