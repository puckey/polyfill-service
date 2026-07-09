//! Unit tests for the feature-resolution pipeline, run against a tiny
//! synthetic polyfill store (version 9.9.9) so expectations are exact.
//!
//! Fixture shape:
//!   aliases: alias1 -> [alias2, E], alias2 -> [A], default -> [A, B]
//!   A: depends on B, chrome <50, detect `self.A`
//!   B: depends on D, chrome <50, detect `self.B`
//!   C: no browsers (never targeted by UA), detect `self.C`
//!   D: chrome <50, no dependencies, no detect
//!   E: chrome * (every chrome)

use polyfill_library::buffer::Buffer;
use polyfill_library::get_polyfill_string::get_polyfill_string_stream;
use polyfill_library::polyfill_parameters::PolyfillParameters;
use polyfill_library::Env;
use std::sync::Arc;

const VERSION: &str = "9.9.9";
const OLD_CHROME: &str =
    "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/40.0.0.0 Safari/537.36";
const NEW_CHROME: &str =
    "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.0.0 Safari/537.36";

fn feature_row(name: &str, meta: &str) -> Vec<(String, String)> {
    vec![
        (format!("/{name}/meta.json"), meta.to_owned()),
        (format!("/{name}/raw.js"), format!("RAW_{name};\n")),
        (format!("/{name}/min.js"), format!("MIN_{name};")),
    ]
}

fn build_fixture_store(test_name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "polyfill-fixture-{}-{test_name}.db",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let conn = rusqlite::Connection::open(&path).expect("failed to create fixture store");
    conn.execute_batch(
        "CREATE TABLE files_9_9_9 (name TEXT PRIMARY KEY, value BLOB NOT NULL);",
    )
    .unwrap();

    let mut rows: Vec<(String, String)> = vec![(
        "/aliases.json".to_owned(),
        r#"{"alias1":["alias2","E"],"alias2":["A"],"default":["A","B"]}"#.to_owned(),
    )];
    rows.extend(feature_row(
        "A",
        r#"{"dependencies":["B"],"browsers":{"chrome":"<50"},"detectSource":"self.A"}"#,
    ));
    rows.extend(feature_row(
        "B",
        r#"{"dependencies":["D"],"browsers":{"chrome":"<50"},"detectSource":"self.B"}"#,
    ));
    rows.extend(feature_row("C", r#"{"detectSource":"self.C"}"#));
    rows.extend(feature_row("D", r#"{"browsers":{"chrome":"<50"}}"#));
    rows.extend(feature_row("E", r#"{"browsers":{"chrome":"*"}}"#));

    for (name, value) in rows {
        conn.execute(
            "INSERT INTO files_9_9_9 (name, value) VALUES (?, ?)",
            rusqlite::params![name, value.into_bytes()],
        )
        .unwrap();
    }
    path
}

fn test_env(test_name: &str) -> Arc<Env> {
    let path = build_fixture_store(test_name);
    let manager = r2d2_sqlite::SqliteConnectionManager::file(&path);
    let pool = r2d2::Pool::builder().max_size(2).build(manager).unwrap();
    Arc::new(Env {
        polyfill_store: pool,
        version_meta_cache: std::sync::RwLock::new(std::collections::HashMap::new()),
        store_query_metric: prometheus::IntCounterVec::new(
            prometheus::Opts::new("q", "q"),
            &["status"],
        )
        .unwrap(),
        up_to_date_ua_metric: prometheus::IntCounter::new("u", "u").unwrap(),
        injected_polyfill_metric: prometheus::IntCounter::new("i", "i").unwrap(),
        bytes_out_metric: prometheus::IntCounter::new("b", "b").unwrap(),
    })
}

fn params(features: &str, ua: &str) -> PolyfillParameters {
    PolyfillParameters {
        excludes: vec![],
        features: polyfill_library::features_from_query_parameter::features_from_query_parameter(
            features, "",
        ),
        minify: false,
        callback: None,
        unknown: "polyfill".to_owned(),
        ua_string: ua.to_owned(),
        version: VERSION.to_owned(),
        strict: false,
    }
}

async fn bundle(env: &Arc<Env>, parameters: &PolyfillParameters) -> String {
    let mut output = Buffer::new();
    get_polyfill_string_stream(&mut output, parameters, Arc::clone(env), VERSION)
        .await
        .expect("failed to build bundle");
    output.into_str()
}

#[tokio::test]
async fn transitive_dependencies_are_included_in_order() {
    let env = test_env("deps");
    let out = bundle(&env, &params("A", OLD_CHROME)).await;

    let d = out.find("RAW_D").expect("D (dependency of B) missing");
    let b = out.find("RAW_B").expect("B (dependency of A) missing");
    let a = out.find("RAW_A").expect("A missing");
    assert!(d < b && b < a, "sources not in dependency order: {out}");
}

#[tokio::test]
async fn excludes_beat_always() {
    let env = test_env("excludes");
    let mut parameters = params("A|always", OLD_CHROME);
    parameters.excludes = vec!["A".to_owned()];
    let out = bundle(&env, &parameters).await;

    assert!(!out.contains("RAW_A"), "excluded feature was served: {out}");
}

#[tokio::test]
async fn alias_of_alias_expands() {
    let env = test_env("alias");
    let out = bundle(&env, &params("alias1", OLD_CHROME)).await;

    assert!(out.contains("RAW_A"), "A (via alias1 -> alias2) missing");
    assert!(out.contains("RAW_E"), "E (via alias1) missing");
}

#[tokio::test]
async fn gated_features_are_wrapped_in_detects() {
    let env = test_env("gated");
    let out = bundle(&env, &params("A|gated", OLD_CHROME)).await;

    assert!(
        out.contains("if (!(self.A))"),
        "gated feature not wrapped in its detect: {out}"
    );
}

#[tokio::test]
async fn unknown_ua_gets_gated_bundle_or_nothing() {
    let env = test_env("unknown");

    let served = bundle(&env, &params("A", "SomeBot/1.0")).await;
    assert!(served.contains("RAW_A"), "unknown=polyfill should serve A");
    assert!(
        served.contains("if (!(self.A))"),
        "unknown=polyfill must gate what it serves"
    );

    let mut parameters = params("A", "SomeBot/1.0");
    parameters.unknown = "ignore".to_owned();
    let ignored = bundle(&env, &parameters).await;
    assert!(
        !ignored.contains("RAW_A"),
        "unknown=ignore should serve nothing"
    );
    assert!(ignored.contains("No polyfills needed"));
}

#[tokio::test]
async fn browser_outside_range_gets_nothing() {
    let env = test_env("range");
    let out = bundle(&env, &params("A", NEW_CHROME)).await;

    assert!(!out.contains("RAW_A"), "chrome 60 matched a <50 range");
    assert!(out.contains("No polyfills needed"));
}

#[tokio::test]
async fn always_flag_overrides_browser_targeting() {
    let env = test_env("always");
    let out = bundle(&env, &params("A|always", NEW_CHROME)).await;

    assert!(
        out.contains("RAW_A"),
        "always-flagged feature not served to non-matching browser"
    );
}

#[tokio::test]
async fn default_alias_expands() {
    let env = test_env("default");
    let out = bundle(&env, &params("default", OLD_CHROME)).await;

    assert!(out.contains("RAW_A") && out.contains("RAW_B"), "default alias incomplete");
}
