# TODO

## UA parser: failing-input data sources

The UA parser (`library/src/useragent.rs`, `parse.rs`, `ua.rs`) is a Rust
transliteration of Financial Times code whose repositories were deleted in
2024 (see [mrhenry/polyfill-library#86](https://github.com/mrhenry/polyfill-library/issues/86)).
There is no upstream to pull fixes from — this fork is effectively the
maintained descendant. These are the sources for finding inputs it gets
wrong, roughly in order of value:

- [x] **1. The deleted upstream's own test suites, recovered from npm** (MIT,
  compatible with this repo's MIT license):
  - [x] `@financial-times/useragent_parser@1.6.3` ships
    `test/user_agent_strings.yaml` — 72 hand-maintained override cases.
    Vendored as `library/tests/fixtures/ft_useragent_corpus.json`, run by
    `library/tests/ft_corpus.rs`. Findings: the transliteration had dropped
    the 4th regex capture (patch version) in every branch of useragent.rs
    (fixed; behavior-neutral for bundles since ua.rs normalizes to
    major.minor.0); the yaml shipped in the tarball was stale against its
    own lib for 3 rows, so the fixture's expectations are regenerated from
    the recovered 1.6.3 runtime; validated with zero disagreements against
    live 1.6.3 across the 1,000-UA pzb corpus.
  - [ ] `@financial-times/polyfill-useragent-normaliser@1.10.2` ships unit
    tests with inline UA → normalised family/version cases (ground truth for
    `ua.rs`: family mapping, baselines, wrapper stripping). Extract the cases
    from `test/unit/node.test.js` into a fixture and compare against
    `UA::new`.
- [ ] **2. uap-core `tests/test_ua.yaml`** (Apache-2.0 — keep its notice if
  vendored): ~thousands of cases, actively maintained. Diff against
  `parse.rs` to see where our frozen regex snapshot drifted. Expect noise:
  new bot/app detection is irrelevant to polyfill serving; browser-family
  misclassifications are the signal.
- [ ] **3. Production traffic**: add sampled logging (or a capped
  counter-by-family) for UAs that classify as unknown, so real Radio Garden
  traffic continuously surfaces misclassified browsers. The iOS-webview bug
  would have been immediately visible this way.
- [ ] **4. intoli/user-agents**: daily-updated real-traffic UA strings with
  market-share weights. Run the corpus through `parse_ua`, triage everything
  common that lands on `other/0.0.0`.
- [ ] **5. Differential harness**: script that runs corpora (2, 4, plus the
  pzb gist from issue 86) through our `parse_ua` and a second opinion
  (`ua-parser-js` or the npm-recovered FT originals under node), and emits a
  triage report of disagreements. Each resolved case becomes a new row in
  `library/tests/fixtures/ua_table.json`.

## Service

- [ ] Run `docker compose up` end-to-end on a machine with a Docker daemon
  (the image build has never been exercised; the compile-memory blocker was
  removed when meta.rs was deleted).
- [ ] Decide production `polyfill.toml` for Radio Garden: trim `features`
  to what the frontend needs; likely `unknown = "ignore"` given bot traffic.
- [ ] Consider commenting on
  [mrhenry/polyfill-library#86](https://github.com/mrhenry/polyfill-library/issues/86)
  pointing at the surviving Rust port and the recovered npm test data.
