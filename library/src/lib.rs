#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
// Duplicate transitive dependency versions are outside our control.
#![allow(clippy::multiple_crate_versions)]

// These modules are transliterated from the upstream FT/JakeChampion
// JavaScript. Their behavior is pinned by the UA table fixture and the
// golden-bundle suite; stylistic rewrites risk changing bundles for some
// user agents with no functional gain, so pedantic/nursery lints are
// silenced for them (clippy::all still applies).
#[allow(clippy::pedantic, clippy::nursery)]
pub mod buffer;
#[allow(clippy::pedantic, clippy::nursery)]
pub mod features_from_query_parameter;
#[allow(clippy::pedantic, clippy::nursery)]
pub mod get_polyfill_string;
#[allow(clippy::pedantic, clippy::nursery)]
pub mod old_ua;
#[allow(clippy::pedantic, clippy::nursery)]
pub mod parse;
#[allow(clippy::pedantic, clippy::nursery)]
pub mod polyfill_parameters;
#[allow(clippy::pedantic, clippy::nursery)]
pub mod toposort;
#[allow(clippy::pedantic, clippy::nursery)]
pub mod ua;
#[allow(
    clippy::pedantic,
    clippy::nursery,
    // Shapes inherent to the mechanical JS transliteration:
    clippy::if_same_then_else,
    clippy::unnecessary_map_or,
    clippy::ptr_arg
)]
pub mod useragent;

pub mod meta_store;
pub(crate) mod regex_cache;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(crate) type BoxError = Box<dyn std::error::Error>;

pub struct Env {
    pub polyfill_store: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    pub version_meta_cache: RwLock<HashMap<String, Arc<meta_store::VersionMeta>>>,
    pub store_query_metric: prometheus::IntCounterVec,
    pub up_to_date_ua_metric: prometheus::IntCounter,
    pub injected_polyfill_metric: prometheus::IntCounter,
    pub bytes_out_metric: prometheus::IntCounter,
}
