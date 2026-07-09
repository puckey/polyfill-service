# Build the service and the SQLite polyfill store.
#
# POLYFILL_VERSIONS controls which library versions are baked into the store:
#   all (default)          — every version under polyfill-libraries/ (~2.5 GB image)
#   3.111.0,3.25.1         — just the default v3 version and the v2 fallback (~150 MB image)
# Note: requests for a version missing from the store fail at runtime, so only
# trim this list if you know which `version=` parameters your sites use.

FROM rust:1-slim AS build

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY library ./library
COPY service ./service
# The library crate embeds polyfill metadata (aliases.json) at compile time.
COPY polyfill-libraries ./polyfill-libraries

RUN cargo build --release

ARG POLYFILL_VERSIONS=all
RUN ./target/release/build-db \
    --libraries ./polyfill-libraries \
    --db /polyfills.db \
    --versions "$POLYFILL_VERSIONS"

FROM debian:bookworm-slim

RUN useradd --system --no-create-home polyfill

COPY --from=build /app/target/release/polyfill-service /usr/local/bin/polyfill-service
COPY --from=build /polyfills.db /data/polyfills.db

ENV POLYFILL_DB=/data/polyfills.db \
    PORT=8080 \
    RUST_LOG=info

EXPOSE 8080
USER polyfill

CMD ["polyfill-service"]
