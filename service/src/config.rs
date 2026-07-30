use indexmap::{IndexMap, IndexSet};
use polyfill_library::features_from_query_parameter::features_from_query_parameter;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    /// Polyfill library version to serve (must be in the store).
    version: String,
    /// Polyfills and aliases to serve, e.g. `fetch`, `es2015`,
    /// `IntersectionObserver`. Every entry is gated — wrapped in a runtime
    /// feature detect — by default, so a polyfill can never clobber a
    /// feature the browser already has (some, like Symbol.matchAll on iOS
    /// 18 WKWebView, are readonly and crash on assignment). Flags after a
    /// pipe adjust that: `Array.from|always` (include for every UA),
    /// `fetch|ungated` (drop the runtime detect).
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

    let mut features = features_from_query_parameter(&raw.features.join(","), "");

    // Gating is the default: `|ungated` opts a single entry out, and the
    // (now redundant) `|gated` stays accepted. Unknown flags are config
    // typos — reject them here so they fail the deploy, matching how
    // unknown feature names are handled.
    for (name, flags) in &mut features {
        // features_from_query_parameter splits its (empty) global-flags
        // parameter into [""] and folds that into every set.
        flags.swap_remove("");
        for flag in flags.iter() {
            assert!(
                flag == "always" || flag == "gated" || flag == "ungated",
                "config {path}: feature {name:?} has unknown flag {flag:?} (known: always, gated, ungated)"
            );
        }
        assert!(
            !(flags.contains("gated") && flags.contains("ungated")),
            "config {path}: feature {name:?} is both gated and ungated"
        );
        if !flags.swap_remove("ungated") {
            flags.insert("gated".to_owned());
        }
    }

    ServiceConfig {
        version: raw.version,
        features,
        feature_list: raw.features,
        unknown: raw.unknown,
        excludes: raw.excludes,
    }
}

#[cfg(test)]
mod tests {
    fn write_config(name: &str, contents: &str) -> String {
        let path = std::env::temp_dir().join(format!(
            "polyfill-config-test-{}-{name}.toml",
            std::process::id()
        ));
        std::fs::write(&path, contents).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn minimal_config_parses_with_defaults() {
        let path = write_config("minimal", "version = \"5.3.1\"\nfeatures = [\"fetch\"]\n");
        let config = super::load(&path);
        let _ = std::fs::remove_file(&path);

        assert_eq!(config.version, "5.3.1");
        assert_eq!(config.unknown, "polyfill");
        assert!(config.excludes.is_empty());
        assert!(config.features.contains_key("fetch"));
        // Gating is the default.
        assert!(config.features["fetch"].contains("gated"));
    }

    #[test]
    fn feature_flags_are_parsed() {
        let path = write_config(
            "flags",
            "version = \"5.3.1\"\nfeatures = [\"Array.from|always\"]\n",
        );
        let config = super::load(&path);
        let _ = std::fs::remove_file(&path);

        assert!(config.features["Array.from"].contains("always"));
        // `always` composes with the gated default.
        assert!(config.features["Array.from"].contains("gated"));
    }

    #[test]
    fn ungated_opts_out_of_the_gated_default() {
        let path = write_config(
            "ungated",
            "version = \"5.3.1\"\nfeatures = [\"fetch|ungated\", \"Blob\"]\n",
        );
        let config = super::load(&path);
        let _ = std::fs::remove_file(&path);

        assert!(!config.features["fetch"].contains("gated"));
        // The pseudo-flag is consumed here, not passed to the library.
        assert!(!config.features["fetch"].contains("ungated"));
        assert!(config.features["Blob"].contains("gated"));
    }

    #[test]
    #[should_panic(expected = "unknown flag")]
    fn unknown_flags_are_rejected() {
        let path = write_config(
            "flag-typo",
            "version = \"5.3.1\"\nfeatures = [\"fetch|gatd\"]\n",
        );
        super::load(&path);
    }

    #[test]
    #[should_panic(expected = "both gated and ungated")]
    fn contradictory_flags_are_rejected() {
        let path = write_config(
            "flag-clash",
            "version = \"5.3.1\"\nfeatures = [\"fetch|gated|ungated\"]\n",
        );
        super::load(&path);
    }

    #[test]
    #[should_panic(expected = "`unknown` must be")]
    fn invalid_unknown_policy_is_rejected() {
        let path = write_config(
            "unknown",
            "version = \"5.3.1\"\nfeatures = [\"fetch\"]\nunknown = \"whatever\"\n",
        );
        super::load(&path);
    }

    #[test]
    #[should_panic(expected = "must list at least one")]
    fn empty_features_are_rejected() {
        let path = write_config("empty", "version = \"5.3.1\"\nfeatures = []\n");
        super::load(&path);
    }

    #[test]
    #[should_panic(expected = "invalid config")]
    fn unknown_keys_are_rejected() {
        let path = write_config(
            "typo",
            "version = \"5.3.1\"\nfeatures = [\"fetch\"]\nfeaturez = []\n",
        );
        super::load(&path);
    }

    #[test]
    #[should_panic(expected = "failed to read config")]
    fn missing_file_is_rejected() {
        super::load("/definitely/not/a/real/polyfill.toml");
    }
}
