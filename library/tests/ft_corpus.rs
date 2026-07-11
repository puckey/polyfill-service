//! Runs the recovered upstream test corpus against our transliterated parser.
//!
//! The fixture is `test/user_agent_strings.yaml` from
//! `@financial-times/useragent_parser@1.6.3` (MIT), recovered from the npm
//! tarball — the GitHub repository was deleted in 2024, so this corpus is
//! the original authors' ground truth for exactly the special cases
//! `useragent.rs` transliterates. A mismatch here (outside the documented
//! exceptions) is a transliteration bug.

use polyfill_library::useragent::useragent;

#[derive(serde::Deserialize)]
struct Fixture {
    cases: Vec<Case>,
}

#[derive(serde::Deserialize)]
struct Case {
    ua: String,
    family: String,
    major: Option<String>,
    minor: Option<String>,
    patch: Option<String>,
}

/// UAs where this fork intentionally diverges from the original parser,
/// with the reason. Keep this list short and explained.
const INTENTIONAL_DIVERGENCES: &[(&str, &str)] = &[
    // 1.6.3's OS-version regex only accepts two components ("OS 11_4 like"),
    // so "OS 11_4_1" fell through to the WebKit fallback table (11.0). This
    // fork fixed that (commit 5137b7996): we return the real 11.4.
    (
        "Mozilla/5.0 (iPhone; CPU iPhone OS 11_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15G77 [FBAN/FBIOS;FBDV/iPhone10,4;FBMD/iPhone;FBSN/iOS;FBSV/11.4.1;FBSS/2;FBCR/A1;FBID/phone;FBLC/de_DE;FBOP/5;FBRV/122166081]",
        "iOS patch-version fix: 11.4, not the WebKit-table 11.0",
    ),
    (
        "Mozilla/5.0 (iPhone; CPU iPhone OS 11_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15G77 [FBAN/FBIOS;FBAV/194.0.0.38.99;FBBV/127868476;FBDV/iPhone7,2;FBMD/iPhone;FBSN/iOS;FBSV/11.4.1;FBSS/2;FBCR/OrangeBotswana;FBID/phone;FBLC/en_GB;FBOP/5;FBRV/128807018]",
        "iOS patch-version fix: 11.4, not the WebKit-table 11.0",
    ),
];

/// The original returned `undefined` for absent version parts; the port
/// returns "0". Both feed `format!("{major}.{minor}.0")` downstream, so
/// null and "0" are equivalent.
fn normalize(part: Option<&str>) -> &str {
    match part {
        None | Some("") => "0",
        Some(value) => value,
    }
}

#[test]
fn ft_useragent_corpus() {
    let fixture: Fixture = serde_json::from_str(include_str!("fixtures/ft_useragent_corpus.json"))
        .expect("invalid corpus fixture");
    assert_eq!(fixture.cases.len(), 72);

    let mut failures = Vec::new();
    for case in &fixture.cases {
        if let Some((_, reason)) = INTENTIONAL_DIVERGENCES
            .iter()
            .find(|(ua, _)| *ua == case.ua)
        {
            eprintln!("skipping (intentional divergence: {reason}): {}", case.ua);
            continue;
        }

        let [family, major, minor, patch] = useragent(&case.ua);
        let expected = (
            case.family.as_str(),
            normalize(case.major.as_deref()),
            normalize(case.minor.as_deref()),
            normalize(case.patch.as_deref()),
        );
        let actual = (
            family.as_str(),
            normalize(Some(&major)),
            normalize(Some(&minor)),
            normalize(Some(&patch)),
        );

        if expected != actual {
            failures.push(format!(
                "  {}\n    expected {}/{}.{}.{}, got {}/{}.{}.{}",
                case.ua,
                expected.0,
                expected.1,
                expected.2,
                expected.3,
                actual.0,
                actual.1,
                actual.2,
                actual.3
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of 72 upstream corpus cases fail:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
