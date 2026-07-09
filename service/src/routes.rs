use crate::pages::home;
use crate::polyfill::polyfill;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Method, StatusCode};
use axum::response::Response;

const APPLICATION_JSON: &str = "application/json";

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

macro_rules! library_json {
    ( $file:expr ) => {{
        return resp(
            StatusCode::OK,
            &[
                ("content-type", APPLICATION_JSON),
                ("x-compress-hint", "on"),
                ("surrogate-key", "website"),
                (
                    "Cache-Control",
                    "max-age=86400, stale-while-revalidate=86400, stale-if-error=86400",
                ),
            ],
            include_str!($file),
        );
    }};
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
    };
    let path = req.uri().path().to_owned();
    match path.as_str() {
        "/" => {
            return resp(
                StatusCode::OK,
                &[
                    ("content-type", "text/html; charset=utf-8"),
                    ("x-compress-hint", "on"),
                    // Enables the cross-site scripting filter built into most modern web browsers.
                    ("X-XSS-Protection", "1; mode=block"),
                    // Prevents MIME-sniffing a response away from the declared content type.
                    ("X-Content-Type-Options", "nosniff"),
                    // The Referrer-Policy header governs which referrer information, sent in the Referer header, should be included with requests made.
                    // Send a full URL when performing a same-origin request, but only send the origin of the document for other cases.
                    ("Referrer-Policy", "origin-when-cross-origin"),
                    // Ensure the site is only served over HTTPS and reduce the chances of someone performing a MITM attack.
                    (
                        "Strict-Transport-Security",
                        "max-age=31536000; includeSubdomains; preload",
                    ),
                    (
                        "Cache-Control",
                        "max-age=60, stale-while-revalidate=60, stale-if-error=86400",
                    ),
                ],
                home(),
            );
        }
        "/img/logo.svg" => {
            return resp(
                StatusCode::OK,
                &[
                    ("content-type", "image/svg+xml"),
                    ("x-compress-hint", "on"),
                    ("surrogate-key", "website"),
                ],
                include_str!("logo.svg"),
            );
        }
        "/robots.txt" => {
            return resp(StatusCode::OK, &[], "User-agent: *\nDisallow:");
        }

        "/v3/json/library-3.101.0.json" => {
            library_json!("json/library-3.101.0.json")
        }
        "/v3/json/library-3.103.0.json" => {
            library_json!("json/library-3.103.0.json")
        }
        "/v3/json/library-3.104.0.json" => {
            library_json!("json/library-3.104.0.json")
        }
        "/v3/json/library-3.108.0.json" => {
            library_json!("json/library-3.108.0.json")
        }
        "/v3/json/library-3.109.0.json" => {
            library_json!("json/library-3.109.0.json")
        }
        "/v3/json/library-3.110.1.json" => {
            library_json!("json/library-3.110.1.json")
        }
        "/v3/json/library-3.111.0.json" => {
            library_json!("json/library-3.111.0.json")
        }
        "/v3/json/library-3.27.4.json" => {
            library_json!("json/library-3.27.4.json")
        }
        "/v3/json/library-3.34.0.json" => {
            library_json!("json/library-3.34.0.json")
        }
        "/v3/json/library-3.39.0.json" => {
            library_json!("json/library-3.39.0.json")
        }
        "/v3/json/library-3.40.0.json" => {
            library_json!("json/library-3.40.0.json")
        }
        "/v3/json/library-3.41.0.json" => {
            library_json!("json/library-3.41.0.json")
        }
        "/v3/json/library-3.42.0.json" => {
            library_json!("json/library-3.42.0.json")
        }
        "/v3/json/library-3.46.0.json" => {
            library_json!("json/library-3.46.0.json")
        }
        "/v3/json/library-3.48.0.json" => {
            library_json!("json/library-3.48.0.json")
        }
        "/v3/json/library-3.50.2.json" => {
            library_json!("json/library-3.50.2.json")
        }
        "/v3/json/library-3.51.0.json" => {
            library_json!("json/library-3.51.0.json")
        }
        "/v3/json/library-3.52.0.json" => {
            library_json!("json/library-3.52.0.json")
        }
        "/v3/json/library-3.52.1.json" => {
            library_json!("json/library-3.52.1.json")
        }
        "/v3/json/library-3.52.2.json" => {
            library_json!("json/library-3.52.2.json")
        }
        "/v3/json/library-3.52.3.json" => {
            library_json!("json/library-3.52.3.json")
        }
        "/v3/json/library-3.53.1.json" => {
            library_json!("json/library-3.53.1.json")
        }
        "/v3/json/library-3.89.4.json" => {
            library_json!("json/library-3.89.4.json")
        }
        "/v3/json/library-3.96.0.json" => {
            library_json!("json/library-3.96.0.json")
        }
        "/v3/json/library-3.98.0.json" => {
            library_json!("json/library-3.98.0.json")
        }

        // FIXME: should be v4
        "/v3/json/library-4.8.0.json" => {
            library_json!("json/library-4.8.0.json")
        }
        "/v3/json/library-5.3.1.json" => {
            library_json!("json/library-5.3.1.json")
        }

        _ => {
            let user_agent = req
                .headers()
                .get(header::USER_AGENT)
                .and_then(|value| value.to_str().ok())
                .map(std::borrow::ToOwned::to_owned);

            let path_and_query = req
                .uri()
                .path_and_query()
                .map_or_else(|| path.clone(), |pq| pq.as_str().to_owned());
            let url = match url::Url::parse(&format!("http://polyfill.invalid{path_and_query}")) {
                Ok(url) => url,
                Err(err) => {
                    return resp(
                        StatusCode::BAD_REQUEST,
                        &[],
                        format!("failed to parse request URL: {err}"),
                    );
                }
            };

            if path == "/v2/polyfill.js" || path == "/v2/polyfill.min.js" {
                let mut url = url;
                url.set_path(&(String::from("/v3") + &path[3..]));
                url.query_pairs_mut().append_pair("version", "3.25.1");

                let has_unknown = url.query_pairs().any(|(key, _)| key == "unknown");
                if !has_unknown {
                    url.query_pairs_mut().append_pair("unknown", "ignore");
                }

                polyfill(&url, user_agent.as_deref(), state.env, &state.versions).await
            }
            // FIXME: add v4
            else if path == "/v3/polyfill.min.js" || path == "/v3/polyfill.js" {
                polyfill(&url, user_agent.as_deref(), state.env, &state.versions).await
            } else {
                resp(
                    StatusCode::NOT_FOUND,
                    &[("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable")],
                    format!("{path}: Not Found"),
                )
            }
        }
    }
}
