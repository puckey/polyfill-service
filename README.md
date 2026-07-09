# polyfill-service (self-hosted)

A self-hostable fork of [cdnjs/polyfill-service](https://github.com/cdnjs/polyfill-service) — the polyfill.io replacement Cloudflare runs at `https://cdnjs.cloudflare.com/polyfill`. Upstream is in maintenance mode ([cdnjs/polyfill-service#21](https://github.com/cdnjs/polyfill-service/issues/21)) and only runs on Cloudflare Workers; this fork replaces the Workers runtime with a plain HTTP server so the service runs anywhere Docker runs.

What changed relative to upstream:

- The Cloudflare Workers HTTP layer (`worker` crate) was replaced with [axum](https://github.com/tokio-rs/axum); the service is now a normal native binary.
- The D1 database holding the polyfill sources was replaced with a local SQLite file, built from `polyfill-libraries/` by the included `build-db` binary (same schema, same queries).
- Prometheus metrics are exposed at `/metrics`.
- All the polyfill bundling logic (UA detection, feature resolution, dependency sorting) is unchanged upstream code.

## Run it

```sh
docker compose up
```

Then use it exactly like the hosted service:

```
http://localhost:8080/v3/polyfill.min.js
http://localhost:8080/v3/polyfill.min.js?features=fetch,Promise
http://localhost:8080/v2/polyfill.min.js        (legacy v2 API)
```

The first build takes a while: it compiles the Rust workspace and packs every
polyfill library version into a SQLite store (~2.5 GB). Give the Docker
builder at least 4 GB of memory — the generated polyfill metadata is a very
large crate to compile (with colima: `colima start --memory 8`). If you only use the
default library version, build a slim image instead:

```sh
docker build --build-arg POLYFILL_VERSIONS=3.111.0,3.25.1 -t polyfill-service .
docker run -p 8080:8080 polyfill-service
```

`3.25.1` is required for the `/v2` endpoints; `3.111.0` is the default for `/v3`.
Requests with an explicit `version=` parameter only work for versions baked
into the store.

## Run it without Docker

```sh
cargo build --release
./target/release/build-db --libraries ./polyfill-libraries --db polyfills.db --versions 3.111.0,3.25.1
POLYFILL_DB=polyfills.db PORT=8080 ./target/release/polyfill-service
```

## Configuration

| Environment variable | Default | Meaning |
| --- | --- | --- |
| `POLYFILL_DB` | `polyfills.db` | Path to the SQLite store built by `build-db` |
| `PORT` | `8080` | HTTP listen port |
| `RUST_LOG` | `info` | Log filter (tracing-subscriber syntax) |

The service itself does no TLS, compression, or caching — run it behind your
regular reverse proxy / CDN and let that layer handle those. Responses carry
long-lived `Cache-Control` headers and `Vary: User-Agent`, so any standard
HTTP cache in front of it will do the heavy lifting.

## Tests

The upstream integration suite runs against a live server on port 7676:

```sh
PORT=7676 POLYFILL_DB=polyfills.db ./target/release/polyfill-service &
cd test && npm install && npx mocha integration/**/*.test.js --timeout 60000
```

---

Upstream README: this repository is a fork of the cdnjs polyfill service
(<https://cdnjs.cloudflare.com/polyfill>), which is itself a maintained fork of
the original polyfill.io. See the announcements from Cloudflare:
<https://blog.cloudflare.com/polyfill-io-now-available-on-cdnjs-reduce-your-supply-chain-risk>.
