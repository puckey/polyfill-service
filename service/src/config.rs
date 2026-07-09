use indexmap::{IndexMap, IndexSet};
use polyfill_library::features_from_query_parameter::features_from_query_parameter;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    /// Polyfill library version to serve (must be in the store).
    version: String,
    /// Polyfills and aliases to serve, e.g. "fetch", "es2015",
    /// "IntersectionObserver". An entry may carry flags after a pipe:
    /// "Array.from|always" (always include), "fetch|gated" (wrap in a
    /// runtime feature detect even for browsers that match).
    features: Vec<String>,
    /// What unrecognized user agents (bots, exotic browsers) receive:
    /// "polyfill" = every configured feature behind runtime detects,
    /// "ignore" = an empty bundle. Defaults to "polyfill".
    #[serde(default = "default_unknown")]
    unknown: String,
    /// Polyfills to exclude even when a configured alias pulls them in.
    #[serde(default)]
    excludes: Vec<String>,
}

fn default_unknown() -> String {
    "polyfill".to_owned()
}

pub struct ServiceConfig {
    pub version: String,
    pub features: IndexMap<String, IndexSet<String>>,
    pub feature_list: Vec<String>,
    pub unknown: String,
    pub excludes: Vec<String>,
}

pub fn load(path: &str) -> ServiceConfig {
    let contents = std::fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read config file {path}: {err} (set POLYFILL_CONFIG to its location)")
    });
    let raw: RawConfig =
        toml::from_str(&contents).unwrap_or_else(|err| panic!("invalid config {path}: {err}"));

    assert!(
        !raw.features.is_empty(),
        "config {path}: `features` must list at least one polyfill or alias"
    );
    assert!(
        raw.unknown == "polyfill" || raw.unknown == "ignore",
        "config {path}: `unknown` must be \"polyfill\" or \"ignore\", got {:?}",
        raw.unknown
    );

    let features = features_from_query_parameter(&raw.features.join(","), "");

    ServiceConfig {
        version: raw.version,
        features,
        feature_list: raw.features,
        unknown: raw.unknown,
        excludes: raw.excludes,
    }
}
