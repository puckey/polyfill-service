use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Returns a lazily compiled, permanently cached regex for a static pattern.
///
/// The UA parsing code evaluates hundreds of regex literals per request;
/// compiling them once and reusing the compiled form is the difference
/// between ~50ms and ~1ms per bundle request.
pub(crate) fn cached_regex(pattern: &'static str) -> &'static regex::Regex {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, &'static regex::Regex>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();
    map.entry(pattern).or_insert_with(|| {
        Box::leak(Box::new(
            regex::Regex::new(pattern).expect("invalid cached regex pattern"),
        ))
    })
}
