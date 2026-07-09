#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::missing_docs_in_private_items)]
mod pages;
mod polyfill;
mod routes;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use polyfill_library::Env;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub env: Arc<Env>,
    pub registry: prometheus::Registry,
    pub versions: Arc<StoreVersions>,
}

/// Library versions actually present in the polyfill store.
pub struct StoreVersions {
    pub available: Vec<String>,
    /// Served when a request names a version the store does not have.
    pub fallback: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let db_path = std::env::var("POLYFILL_DB").unwrap_or_else(|_| "polyfills.db".to_owned());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let manager = r2d2_sqlite::SqliteConnectionManager::file(&db_path).with_flags(
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    );
    let pool = r2d2::Pool::builder()
        .max_size(16)
        .build(manager)
        .unwrap_or_else(|err| panic!("failed to open polyfill store at {db_path}: {err}"));

    let versions = Arc::new(discover_store_versions(&pool, &db_path));

    let registry = prometheus::Registry::new();
    let store_query_metric = prometheus::IntCounterVec::new(
        prometheus::Opts::new(
            "polyfill_store_queries_total",
            "Polyfill store queries by status",
        ),
        &["status"],
    )
    .unwrap();
    let up_to_date_ua_metric = prometheus::IntCounter::new(
        "polyfill_up_to_date_ua_total",
        "Requests answered with an empty bundle because the browser needs no polyfills",
    )
    .unwrap();
    let injected_polyfill_metric = prometheus::IntCounter::new(
        "polyfill_injected_total",
        "Individual polyfills written into bundles",
    )
    .unwrap();
    let bytes_out_metric =
        prometheus::IntCounter::new("polyfill_bytes_out_total", "Bundle bytes written").unwrap();

    registry
        .register(Box::new(store_query_metric.clone()))
        .unwrap();
    registry
        .register(Box::new(up_to_date_ua_metric.clone()))
        .unwrap();
    registry
        .register(Box::new(injected_polyfill_metric.clone()))
        .unwrap();
    registry
        .register(Box::new(bytes_out_metric.clone()))
        .unwrap();

    let env = Arc::new(Env {
        polyfill_store: pool,
        version_meta_cache: std::sync::RwLock::new(std::collections::HashMap::new()),
        store_query_metric,
        up_to_date_ua_metric,
        injected_polyfill_metric,
        bytes_out_metric,
    });

    let state = AppState {
        env,
        registry,
        versions,
    };

    let app = axum::Router::new()
        .route("/metrics", axum::routing::get(metrics))
        .fallback(routes::handle_request)
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|err| panic!("failed to bind port {port}: {err}"));
    tracing::info!("listening on http://0.0.0.0:{port}");
    axum::serve(listener, app).await.expect("server failed");
}

fn discover_store_versions(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    db_path: &str,
) -> StoreVersions {
    let conn = pool.get().expect("failed to get store connection");
    let mut stmt = conn
        .prepare(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name LIKE 'files_%' ORDER BY name",
        )
        .expect("failed to inspect polyfill store");
    let mut available = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .expect("failed to inspect polyfill store")
        .filter_map(Result::ok)
        .map(|table| table.trim_start_matches("files_").replace('_', "."))
        .collect::<Vec<_>>();

    assert!(
        !available.is_empty(),
        "polyfill store at {} contains no files_* tables — build it with the build-db binary",
        db_path
    );
    available.sort_by_key(|version| numeric_version_key(version));

    // Prefer the upstream default so unversioned requests behave like the
    // production service; otherwise serve the newest version in the store.
    let fallback = if available.iter().any(|v| v == "3.111.0") {
        "3.111.0".to_owned()
    } else {
        available.last().cloned().unwrap()
    };

    tracing::info!(
        "polyfill store {} provides versions: {} (fallback: {})",
        db_path,
        available.join(", "),
        fallback
    );

    StoreVersions {
        available,
        fallback,
    }
}

fn numeric_version_key(version: &str) -> (u64, u64, u64) {
    let mut parts = version
        .split('.')
        .map(|part| part.parse::<u64>().unwrap_or(0));
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

async fn metrics(State(state): State<AppState>) -> Response {
    let encoder = prometheus::TextEncoder::new();
    match encoder.encode_to_string(&state.registry.gather()) {
        Ok(body) => (
            StatusCode::OK,
            [("content-type", prometheus::TEXT_FORMAT)],
            body,
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to encode metrics: {err}"),
        )
            .into_response(),
    }
}
