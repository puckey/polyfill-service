# polyfill-service (self-hosted)

A self-hostable fork of [cdnjs/polyfill-service](https://github.com/cdnjs/polyfill-service) — the polyfill.io replacement Cloudflare runs at `https://cdnjs.cloudflare.com/polyfill`. Upstream is in maintenance mode ([cdnjs/polyfill-service#21](https://github.com/cdnjs/polyfill-service/issues/21)) and only runs on Cloudflare Workers; this fork runs anywhere Docker runs.

Unlike upstream, the bundle is **defined on the server, not in the URL**. A single `polyfill.toml` names one library version and the polyfills to serve; clients just load `/polyfill.min.js` and get what their browser needs. Query parameters are ignored — there is no per-request feature selection, no version switching, no callback injection surface.

What changed relative to upstream:

- The Cloudflare Workers HTTP layer was replaced with [axum](https://github.com/tokio-rs/axum); the D1 database with a local SQLite file built by the included `build-db` binary.
- Bundle configuration moved from URL parameters to `polyfill.toml` (version, features, unknown-UA policy, excludes). The server validates the config against the library metadata at startup, so a typo'd feature name fails the deploy instead of silently serving nothing.
- Polyfill library [5.3.1](https://github.com/mrhenry/polyfill-library) is vendored (upstream stops at 4.8.0), adding the es2025 features from [cdnjs/polyfill-service#15](https://github.com/cdnjs/polyfill-service/issues/15): `Promise.try`, the `Set` methods, and the Iterator helpers.
- Hot paths were fixed (cached regexes, metadata parsed once at startup): a bundle response costs ~1–4 ms of CPU. Responses are compressed (gzip/brotli/zstd). Prometheus metrics at `/metrics`.
- The polyfill bundling logic itself (UA detection, feature resolution, dependency sorting) is unchanged upstream code.

## Configure

Edit `polyfill.toml`:

```toml
version = "5.3.1"

features = [
    "default",              # the library's curated baseline set
    "fetch",
    # "IntersectionObserver",
    # "Array.from|always",  # flags: |always, |gated
]

# What unrecognized user agents (bots) get: "polyfill" (everything,
# feature-gated — safe but big) or "ignore" (empty bundle).
unknown = "polyfill"
```

## Run it

```sh
docker compose up
```

Then point your pages at it:

```html
<script src="https://your-host/polyfill.min.js"></script>
```

`/polyfill.js` serves the readable variant with per-feature license comments.
`/v3/polyfill.min.js` and `/v3/polyfill.js` are aliases so existing
polyfill.io-style embed URLs keep working after a domain swap — their query
parameters are ignored.

To change the bundle, edit `polyfill.toml` and restart the container (it is
mounted, not baked). If you change `version`, rebuild the image with a
matching store: `docker compose build --build-arg POLYFILL_VERSIONS=<version>`.

## Run it without Docker

```sh
cargo build --release
./target/release/build-db --libraries ./polyfill-libraries --db polyfills.db --versions 5.3.1
./target/release/polyfill-service
```

## Configuration reference

| Environment variable | Default | Meaning |
| --- | --- | --- |
| `POLYFILL_CONFIG` | `polyfill.toml` | Path to the bundle definition |
| `POLYFILL_DB` | `polyfills.db` | Path to the SQLite store built by `build-db` |
| `PORT` | `8080` | HTTP listen port |
| `RUST_LOG` | `info` | Log filter (tracing-subscriber syntax) |

The service compresses responses but does no TLS or caching — run it behind
your regular reverse proxy / CDN. Responses carry long-lived `Cache-Control`
headers and `Vary: User-Agent`, so any standard HTTP cache in front of it
will do the heavy lifting.

## Tests

```sh
cargo test                                      # UA table + resolution-loop tests
PORT=7676 ./target/release/polyfill-service &   # with the sample polyfill.toml
cd test && pnpm install && pnpm test            # API + golden-bundle tests
```

With a fixed `polyfill.toml` and store, a bundle is a pure function of the
User-Agent string. `test/integration/goldens.json` pins that function for
30 representative UAs (browsers straddling feature cutoffs, in-app webviews,
bots, garbage): the resolved feature list, the minified bundle's hash, and —
for most entries — that the bundle actually executes in a jsdom window and
installs the expected globals. `library/tests/fixtures/ua_table.json` pins
UA-string → family/version normalization the same way.

If you change the config, the library version, or resolution behavior,
snapshots will fail by design. Explain the diff, then re-bless:

```sh
cd test && pnpm bless
UPDATE_UA_TABLE=1 cargo test -p polyfill-library --test ua_table
```

---

This repository is a fork of the cdnjs polyfill service
(<https://cdnjs.cloudflare.com/polyfill>), which is itself a maintained fork
of the original polyfill.io.
