//! Resolution against the vendored library's own metadata, for the serve
//! ranges a synthetic fixture cannot vouch for. The store holds only the
//! requested features and their dependency closure, read from
//! `polyfill-libraries/<version>/`.

use polyfill_library::Env;
use polyfill_library::buffer::Buffer;
use polyfill_library::get_polyfill_string::get_polyfill_string_stream;
use polyfill_library::polyfill_parameters::PolyfillParameters;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

const VERSION: &str = "5.3.1";
const FIREFOX_ANDROID_128: &str =
    "Mozilla/5.0 (Android 14; Mobile; rv:128.0) Gecko/128.0 Firefox/128.0";

fn library_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../polyfill-libraries")
        .join(VERSION)
}

fn dependency_closure(roots: &[&str]) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut pending: Vec<String> = roots.iter().map(|&root| root.to_owned()).collect();
    while let Some(name) = pending.pop() {
        if !seen.insert(name.clone()) {
            continue;
        }
        let meta = std::fs::read_to_string(library_dir().join(&name).join("meta.json"))
            .unwrap_or_else(|err| panic!("no meta.json for {name}: {err}"));
        let meta: serde_json::Value = serde_json::from_str(&meta).unwrap();
        if let Some(dependencies) = meta["dependencies"].as_array() {
            pending.extend(
                dependencies
                    .iter()
                    .filter_map(|d| d.as_str().map(str::to_owned)),
            );
        }
    }
    seen
}

fn build_store(test_name: &str, roots: &[&str]) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "polyfill-library-data-{}-{test_name}.db",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let conn = rusqlite::Connection::open(&path).expect("failed to create store");
    let table = format!("files_{}", VERSION.replace('.', "_"));
    conn.execute_batch(&format!(
        "CREATE TABLE {table} (name TEXT PRIMARY KEY, value BLOB NOT NULL);"
    ))
    .unwrap();
    let insert = format!("INSERT INTO {table} (name, value) VALUES (?, ?)");

    conn.execute(&insert, rusqlite::params!["/aliases.json", b"{}".to_vec()])
        .unwrap();
    for feature in dependency_closure(roots) {
        for file_name in ["raw.js", "min.js", "meta.json"] {
            if let Ok(contents) = std::fs::read(library_dir().join(&feature).join(file_name)) {
                conn.execute(
                    &insert,
                    rusqlite::params![format!("/{feature}/{file_name}"), contents],
                )
                .unwrap();
            }
        }
    }
    path
}

/// Deletes the store when the test finishes.
struct StoreGuard(PathBuf);

impl Drop for StoreGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn test_env(test_name: &str, roots: &[&str]) -> (Arc<Env>, StoreGuard) {
    let path = build_store(test_name, roots);
    let manager = r2d2_sqlite::SqliteConnectionManager::file(&path);
    let pool = r2d2::Pool::builder().max_size(2).build(manager).unwrap();
    let guard = StoreGuard(path);
    let env = Arc::new(Env {
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
    });
    (env, guard)
}

async fn bundle(env: &Arc<Env>, features: &str, ua: &str) -> String {
    let parameters = PolyfillParameters {
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
    };
    let mut output = Buffer::new();
    get_polyfill_string_stream(&mut output, &parameters, Arc::clone(env), VERSION)
        .await
        .expect("failed to build bundle");
    output.into_str()
}

/// Firefox for Android ships the `Iterator` constructor in 131, like desktop
/// Firefox. A browser that is served an Iterator helper without it throws in
/// the helper and skips the rest of the bundle.
#[tokio::test]
async fn firefox_android_before_131_gets_the_iterator_constructor_with_its_helpers() {
    let (env, _store) = test_env("iterator-firefox-android", &["Iterator.from"]);
    let out = bundle(&env, "Iterator.from", FIREFOX_ANDROID_128).await;

    let helper = out
        .find("// Iterator.from")
        .unwrap_or_else(|| panic!("Iterator.from not served: {out}"));
    let constructor = out
        .find("// Iterator\n")
        .unwrap_or_else(|| panic!("Iterator constructor not served with its helper"));
    assert!(constructor < helper, "Iterator served after its helper");
}
