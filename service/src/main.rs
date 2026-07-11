#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::missing_docs_in_private_items)]
mod config;
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
    pub config: Arc<config::ServiceConfig>,
    pub unknown_ua: Arc<UnknownUaTelemetry>,
}

/// Tracks requests from user agents the parser cannot classify.
pub struct UnknownUaTelemetry {
    pub metric: prometheus::IntCounter,
    pub sample_counter: std::sync::atomic::AtomicU64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config_path = std::env::var("POLYFILL_CONFIG").unwrap_or_else(|_| "polyfill.toml".to_owned());
    let config = Arc::new(config::load(&config_path));

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

    ensure_version_in_store(&pool, &db_path, &config.version);

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
    let unknown_ua_metric = prometheus::IntCounter::new(
        "polyfill_unknown_ua_total",
        "Bundle requests from user agents the parser could not classify",
    )
    .unwrap();

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
    registry
        .register(Box::new(unknown_ua_metric.clone()))
        .unwrap();

    let env = Arc::new(Env {
        polyfill_store: pool,
        version_meta_cache: std::sync::RwLock::new(std::collections::HashMap::new()),
        store_query_metric,
        up_to_date_ua_metric,
        injected_polyfill_metric,
        bytes_out_metric,
    });

    validate_features(&env, &config).await;

    tracing::info!(
        "serving polyfill-library {} with features: {}",
        config.version,
        config.feature_list.join(", ")
    );

    let state = AppState {
        env,
        registry,
        config,
        unknown_ua: Arc::new(UnknownUaTelemetry {
            metric: unknown_ua_metric,
            sample_counter: std::sync::atomic::AtomicU64::new(0),
        }),
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

fn ensure_version_in_store(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    db_path: &str,
    version: &str,
) {
    let conn = pool.get().expect("failed to get store connection");
    let mut stmt = conn
        .prepare(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name LIKE 'files_%' ORDER BY name",
        )
        .expect("failed to inspect polyfill store");
    let available = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .expect("failed to inspect polyfill store")
        .filter_map(Result::ok)
        .map(|table| table.trim_start_matches("files_").replace('_', "."))
        .collect::<Vec<_>>();

    assert!(
        available.iter().any(|v| v == version),
        "configured version {} is not in the polyfill store at {} (available: {}) — rebuild it with build-db",
        version,
        db_path,
        if available.is_empty() {
            "none".to_owned()
        } else {
            available.join(", ")
        }
    );
}

/// Fail fast on typos: every configured feature must be a polyfill or an
/// alias known to the configured library version.
async fn validate_features(env: &Arc<Env>, config: &config::ServiceConfig) {
    let meta = polyfill_library::meta_store::version_meta(env, &config.version)
        .await
        .unwrap_or_else(|err| panic!("failed to load metadata: {err}"));

    let unknown = config
        .features
        .keys()
        .filter(|name| {
            meta.polyfill_meta(name).is_none() && meta.config_aliases(name).is_none()
        })
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        unknown.is_empty(),
        "configured features not known to polyfill-library {}: {}",
        config.version,
        unknown.join(", ")
    );

    for exclude in &config.excludes {
        if meta.polyfill_meta(exclude).is_none() {
            tracing::warn!("configured exclude {exclude} is not a known polyfill");
        }
    }
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
