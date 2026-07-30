//! In-memory bundle cache.
//!
//! A bundle is a pure function of (config, UA bucket, minify), and the
//! config is immutable for the lifetime of the process — so entries never
//! expire, they are only evicted by the byte cap. The UA bucket is the
//! parser's own normalization (family + version), which bounds the key
//! space to the few hundred (family, version) pairs seen in real traffic;
//! every unclassified UA shares the single "unknown" entry.
//!
//! Variants are compressed once at insert time and served by memcpy;
//! per-request compression of large bundles — not building them — is
//! where the CPU otherwise goes.

use indexmap::IndexMap;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

/// Total byte budget across all entries and their compressed variants.
/// The worst-case single entry (the full gated unknown-UA bundle) is
/// ~2 MB across variants; typical tailored entries are a few KB.
pub const CACHE_MAX_BYTES: usize = 64 * 1024 * 1024;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct CacheKey {
    /// Normalized UA family, or "unknown" for unclassifiable UAs.
    pub family: String,
    /// Normalized `major.minor.patch` version; empty for "unknown".
    pub version: String,
    pub minify: bool,
}

pub struct CachedBundle {
    pub identity: Vec<u8>,
    pub gzip: Vec<u8>,
    pub brotli: Vec<u8>,
    pub zstd: Vec<u8>,
}

impl CachedBundle {
    pub fn new(identity: Vec<u8>) -> Self {
        let gzip = {
            let mut enc = flate2::write::GzEncoder::new(
                Vec::with_capacity(identity.len() / 2),
                flate2::Compression::best(),
            );
            enc.write_all(&identity).expect("gzip write to Vec");
            enc.finish().expect("gzip finish to Vec")
        };
        let brotli = {
            let mut out = Vec::with_capacity(identity.len() / 2);
            let params = brotli::enc::BrotliEncoderParams {
                quality: 10,
                ..Default::default()
            };
            brotli::BrotliCompress(&mut identity.as_slice(), &mut out, &params)
                .expect("brotli compress to Vec");
            out
        };
        let zstd = zstd::stream::encode_all(identity.as_slice(), 19).expect("zstd encode to Vec");
        Self {
            identity,
            gzip,
            brotli,
            zstd,
        }
    }

    fn weight(&self) -> usize {
        self.identity.len() + self.gzip.len() + self.brotli.len() + self.zstd.len()
    }

    /// Picks the best variant for an `Accept-Encoding` header. Preference
    /// mirrors the compression middleware: brotli, then zstd, then gzip.
    pub fn pick(&self, accept_encoding: Option<&str>) -> (Option<&'static str>, &[u8]) {
        let accepts = |name: &str| {
            accept_encoding.is_some_and(|header| {
                header.split(',').any(|token| {
                    let mut parts = token.trim().split(';');
                    let coding = parts.next().unwrap_or_default().trim();
                    let refused =
                        parts.any(|p| p.trim().replace(' ', "").eq_ignore_ascii_case("q=0"));
                    coding.eq_ignore_ascii_case(name) && !refused
                })
            })
        };
        if accepts("br") {
            (Some("br"), &self.brotli)
        } else if accepts("zstd") {
            (Some("zstd"), &self.zstd)
        } else if accepts("gzip") {
            (Some("gzip"), &self.gzip)
        } else {
            (None, &self.identity)
        }
    }
}

pub struct BundleCache {
    /// Insertion/access-ordered map used as an LRU: hits re-insert at the
    /// back, eviction pops from the front.
    entries: Mutex<(IndexMap<CacheKey, Arc<CachedBundle>>, usize)>,
    max_bytes: usize,
    pub hits: prometheus::IntCounter,
    pub misses: prometheus::IntCounter,
}

impl BundleCache {
    pub fn new(
        max_bytes: usize,
        hits: prometheus::IntCounter,
        misses: prometheus::IntCounter,
    ) -> Self {
        Self {
            entries: Mutex::new((IndexMap::new(), 0)),
            max_bytes,
            hits,
            misses,
        }
    }

    pub fn get(&self, key: &CacheKey) -> Option<Arc<CachedBundle>> {
        let mut guard = self.entries.lock().expect("bundle cache lock");
        let (entries, _) = &mut *guard;
        // Move the hit to the back so eviction order stays least-recent-first.
        let bundle = entries.shift_remove(key)?;
        entries.insert(key.clone(), Arc::clone(&bundle));
        Some(bundle)
    }

    pub fn insert(&self, key: CacheKey, bundle: Arc<CachedBundle>) {
        let mut guard = self.entries.lock().expect("bundle cache lock");
        let (entries, total) = &mut *guard;
        if let Some(previous) = entries.shift_remove(&key) {
            *total -= previous.weight();
        }
        *total += bundle.weight();
        entries.insert(key, bundle);
        while *total > self.max_bytes && entries.len() > 1 {
            let Some((_, evicted)) = entries.shift_remove_index(0) else {
                break;
            };
            *total -= evicted.weight();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counter(name: &str) -> prometheus::IntCounter {
        prometheus::IntCounter::new(name.to_owned(), name.to_owned()).unwrap()
    }

    fn cache(max_bytes: usize) -> BundleCache {
        BundleCache::new(max_bytes, counter("h"), counter("m"))
    }

    fn key(family: &str) -> CacheKey {
        CacheKey {
            family: family.to_owned(),
            version: "1.0.0".to_owned(),
            minify: true,
        }
    }

    #[test]
    fn variants_round_trip() {
        let body = b"if (!(window.fetch)) { /* polyfill */ }".repeat(100);
        let bundle = CachedBundle::new(body.clone());

        let mut gunzipped = Vec::new();
        let mut dec = flate2::read::GzDecoder::new(bundle.gzip.as_slice());
        std::io::Read::read_to_end(&mut dec, &mut gunzipped).unwrap();
        assert_eq!(gunzipped, body);

        let unzstd = zstd::stream::decode_all(bundle.zstd.as_slice()).unwrap();
        assert_eq!(unzstd, body);

        let mut unbrotli = Vec::new();
        brotli::BrotliDecompress(&mut bundle.brotli.as_slice(), &mut unbrotli).unwrap();
        assert_eq!(unbrotli, body);
    }

    #[test]
    fn negotiation_prefers_brotli_and_honors_refusals() {
        let bundle = CachedBundle::new(b"x".repeat(64));
        assert_eq!(bundle.pick(Some("gzip, deflate, br, zstd")).0, Some("br"));
        assert_eq!(bundle.pick(Some("gzip, zstd")).0, Some("zstd"));
        assert_eq!(bundle.pick(Some("gzip")).0, Some("gzip"));
        assert_eq!(bundle.pick(Some("br;q=0, gzip")).0, Some("gzip"));
        assert_eq!(bundle.pick(Some("identity")).0, None);
        assert_eq!(bundle.pick(None).0, None);
    }

    #[test]
    fn lru_evicts_least_recently_used_when_over_budget() {
        let entry = || Arc::new(CachedBundle::new(b"x".repeat(512)));
        let weight = entry().weight();
        // Room for two entries, not three.
        let cache = cache(weight * 2 + weight / 2);
        cache.insert(key("a"), entry());
        cache.insert(key("b"), entry());
        // Touch "a" so "b" is the eviction candidate.
        assert!(cache.get(&key("a")).is_some());
        cache.insert(key("c"), entry());
        assert!(cache.get(&key("b")).is_none());
        assert!(cache.get(&key("a")).is_some());
        assert!(cache.get(&key("c")).is_some());
    }
}
