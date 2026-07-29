//! Per-version polyfill metadata, loaded from the `SQLite` store.
//!
//! The store rows written by `build-db` include each feature's `meta.json`
//! and the version's `aliases.json`. They are loaded and parsed once per
//! version on first use, then served from memory.

use crate::Env;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolyfillConfig {
    pub license: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub browsers: Option<HashMap<String, String>>,
    pub detect_source: Option<String>,
}

pub struct VersionMeta {
    metas: HashMap<String, PolyfillConfig>,
    aliases: HashMap<String, Vec<String>>,
}

impl VersionMeta {
    #[must_use]
    pub fn polyfill_meta(&self, feature_name: &str) -> Option<&PolyfillConfig> {
        if feature_name.is_empty() {
            return None;
        }
        self.metas.get(feature_name)
    }

    #[must_use]
    pub fn config_aliases(&self, alias: &str) -> Option<&Vec<String>> {
        if alias.is_empty() {
            return None;
        }
        self.aliases.get(alias)
    }
}

/// # Errors
///
/// Fails when the store cannot be queried or its metadata rows are missing
/// or unparseable (e.g. a store built by a pre-metadata `build-db`).
///
/// # Panics
///
/// Panics if the metadata cache lock is poisoned.
pub async fn version_meta(
    env: &Arc<Env>,
    version: &str,
) -> Result<Arc<VersionMeta>, crate::BoxError> {
    if let Some(meta) = env.version_meta_cache.read().unwrap().get(version) {
        return Ok(Arc::clone(meta));
    }

    let load_env = Arc::clone(env);
    let load_version = version.to_owned();
    let meta = tokio::task::spawn_blocking(move || load_version_meta(&load_env, &load_version))
        .await
        .map_err(|err| format!("metadata load panicked: {err}"))??;

    let meta = Arc::new(meta);
    env.version_meta_cache
        .write()
        .unwrap()
        .insert(version.to_owned(), Arc::clone(&meta));
    Ok(meta)
}

fn load_version_meta(env: &Env, version: &str) -> Result<VersionMeta, String> {
    let safe_version = version.replace('.', "_");
    let conn = env
        .polyfill_store
        .get()
        .map_err(|err| format!("failed to get store connection: {err}"))?;

    let mut stmt = conn
        .prepare_cached(&format!(
            r"
              SELECT
                  name,
                  cast(value as char) as value
              FROM files_{safe_version}
              WHERE name = '/aliases.json' OR name LIKE '%/meta.json'
        "
        ))
        .map_err(|err| format!("failed to prepare metadata query: {err}"))?;

    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|err| format!("failed to query metadata: {err}"))?;

    let mut metas = HashMap::new();
    let mut aliases = None;
    for row in rows {
        let (name, value) = row.map_err(|err| format!("failed to read metadata row: {err}"))?;
        if name == "/aliases.json" {
            aliases = Some(
                serde_json::from_str(&value)
                    .map_err(|err| format!("failed to parse aliases.json for {version}: {err}"))?,
            );
        } else if let Some(feature) = name
            .strip_prefix('/')
            .and_then(|n| n.strip_suffix("/meta.json"))
        {
            let config = serde_json::from_str(&value)
                .map_err(|err| format!("failed to parse {name} for version {version}: {err}"))?;
            metas.insert(feature.to_owned(), config);
        }
    }

    let aliases = aliases.ok_or_else(|| {
        format!("store has no /aliases.json for version {version} — rebuild it with the current build-db")
    })?;
    if metas.is_empty() {
        return Err(format!(
            "store has no meta.json rows for version {version} — rebuild it with the current build-db"
        ));
    }

    tracing::info!("loaded metadata for version {version} ({} features)", metas.len());
    Ok(VersionMeta { metas, aliases })
}
