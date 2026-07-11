//! Runs the recovered upstream normaliser's outputs against our `ua.rs` port.
//!
//! The fixture holds `UA.normalize()` results from
//! `@financial-times/polyfill-useragent-normaliser@1.10.2` (MIT, recovered
//! from the npm tarball — the GitHub repository was deleted in 2024),
//! executed under node over 1,125 unique real-world user agents. This is
//! ground truth for the normalisation layer: family mapping, baseline
//! demotion to "other", and wrapper-browser stripping. A mismatch outside
//! the documented divergences is a transliteration bug — this corpus is
//! what caught the YaBrowser strip regex carrying a stray JS delimiter.

use polyfill_library::ua::{UA, UserAgent};

#[derive(serde::Deserialize)]
struct Fixture {
    /// UAs where this fork intentionally diverges from the original
    /// normaliser, with the reason. Shared with scripts/ua-triage.ts —
    /// keep the list short and explained.
    intentional_divergences: Vec<Divergence>,
    cases: Vec<Case>,
}

#[derive(serde::Deserialize)]
struct Divergence {
    ua: String,
    reason: String,
}

#[derive(serde::Deserialize)]
struct Case {
    ua: String,
    normalized: String,
}

#[test]
fn ft_normaliser_corpus() {
    let fixture: Fixture =
        serde_json::from_str(include_str!("fixtures/ft_normaliser_corpus.json"))
            .expect("invalid corpus fixture");
    assert!(fixture.cases.len() > 1000);

    let mut failures = Vec::new();
    for case in &fixture.cases {
        if let Some(divergence) = fixture
            .intentional_divergences
            .iter()
            .find(|divergence| divergence.ua == case.ua)
        {
            eprintln!(
                "skipping (intentional divergence: {}): {}",
                divergence.reason, case.ua
            );
            continue;
        }

        let ua = UA::new(&case.ua);
        let actual = format!("{}/{}", ua.get_family(), ua.get_version());
        if actual != case.normalized {
            failures.push(format!(
                "  {}\n    expected {}, got {actual}",
                case.ua, case.normalized
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} normaliser corpus cases fail:\n{}",
        failures.len(),
        fixture.cases.len(),
        failures.join("\n")
    );
}
