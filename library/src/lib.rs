#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod buffer;
pub mod features_from_query_parameter;
pub mod get_polyfill_string;
pub mod meta_store;
pub mod old_ua;
pub mod parse;
pub mod polyfill_parameters;
pub(crate) mod regex_cache;
pub mod toposort;
pub mod ua;
pub mod useragent;

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
