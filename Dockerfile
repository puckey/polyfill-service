# Build the service and the SQLite polyfill store.
#
# POLYFILL_VERSIONS controls which library versions are baked into the store:
#   all (default)          — every version under polyfill-libraries/ (~2.5 GB image)
#   5.3.1,3.111.0,3.25.1   — newest library (es2025), the v3 default, and the v2 fallback
# Requests for a version missing from the store are served with the fallback
# version (the v3 default if present, else the newest in the store).

FROM rust:1-slim AS build

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY library ./library
COPY service ./service
RUN cargo build --release

COPY polyfill-libraries ./polyfill-libraries
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
