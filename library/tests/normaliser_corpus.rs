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
    cases: Vec<Case>,
}

#[derive(serde::Deserialize)]
struct Case {
    ua: String,
    normalized: String,
}

/// UAs where this fork intentionally diverges from the original normaliser,
/// with the reason. Keep this list short and explained.
const INTENTIONAL_DIVERGENCES: &[(&str, &str)] = &[
    // 1.6.3's OS-version regex only accepted two components, so patch-level
    // iOS releases ("OS 17_4_1") fell through to the WebKit fallback table.
    // This fork fixed that (commit 5137b7996): the real OS version wins.
    (
        "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Instagram 325.0.3.31.90 (iPhone14,5; iOS 17_4_1; en_US; en; scale=3.00; 1170x2532; 585196630)",
        "iOS patch-version fix: 17.4, not the WebKit-table 11.0",
    ),
    (
        "Mozilla/5.0 (iPhone; CPU iPhone OS 11_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15G77 [FBAN/FBIOS;FBDV/iPhone10,4;FBMD/iPhone;FBSN/iOS;FBSV/11.4.1;FBSS/2;FBCR/A1;FBID/phone;FBLC/de_DE;FBOP/5;FBRV/122166081]",
        "iOS patch-version fix: 11.4, not the WebKit-table 11.0",
    ),
    (
        "Mozilla/5.0 (iPhone; CPU iPhone OS 11_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15G77 [FBAN/FBIOS;FBAV/194.0.0.38.99;FBBV/127868476;FBDV/iPhone7,2;FBMD/iPhone;FBSN/iOS;FBSV/11.4.1;FBSS/2;FBCR/OrangeBotswana;FBID/phone;FBLC/en_GB;FBOP/5;FBRV/128807018]",
        "iOS patch-version fix: 11.4, not the WebKit-table 11.0",
    ),
];

#[test]
fn ft_normaliser_corpus() {
    let fixture: Fixture =
        serde_json::from_str(include_str!("fixtures/ft_normaliser_corpus.json"))
            .expect("invalid corpus fixture");
    assert!(fixture.cases.len() > 1000);

    let mut failures = Vec::new();
    for case in &fixture.cases {
        if let Some((_, reason)) = INTENTIONAL_DIVERGENCES
            .iter()
            .find(|(ua, _)| *ua == case.ua)
        {
            eprintln!("skipping (intentional divergence: {reason}): {}", case.ua);
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
