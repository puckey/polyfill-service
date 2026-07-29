use indexmap::{IndexMap, IndexSet};
use serde::Serialize;

#[allow(dead_code)]
#[derive(Clone, Default, Serialize)]
pub struct PolyfillParameters {
    pub excludes: Vec<String>,
    pub features: IndexMap<String, IndexSet<String>>,
    pub minify: bool,
    pub callback: Option<String>,
    pub unknown: String,
    pub ua_string: String,
    pub version: String,
    pub strict: bool,
}
