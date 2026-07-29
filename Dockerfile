# Build the service and the SQLite polyfill store.
#
# The bundle the service serves is defined by polyfill.toml (baked in at
# /data/polyfill.toml; mount your own over it or set POLYFILL_CONFIG).
#
# POLYFILL_VERSIONS controls which library versions are packed into the
# store. It must include the version named in polyfill.toml. The default
# packs only the sample config's version; use "all" for every version
# under polyfill-libraries/ (~2.5 GB image).

FROM rust:1-slim AS build

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY library ./library
COPY service ./service
RUN cargo build --release

COPY polyfill-libraries ./polyfill-libraries
ARG POLYFILL_VERSIONS=5.3.1
RUN ./target/release/build-db \
    --libraries ./polyfill-libraries \
    --db /polyfills.db \
    --versions "$POLYFILL_VERSIONS"

FROM debian:bookworm-slim

RUN useradd --system --no-create-home polyfill

COPY --from=build /app/target/release/polyfill-service /usr/local/bin/polyfill-service
COPY --from=build /polyfills.db /data/polyfills.db
COPY polyfill.toml /data/polyfill.toml

ENV POLYFILL_DB=/data/polyfills.db \
    POLYFILL_CONFIG=/data/polyfill.toml \
    PORT=8080 \
    RUST_LOG=info

EXPOSE 8080
USER polyfill

CMD ["polyfill-service"]
