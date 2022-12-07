FROM rust:1.65.0-slim-bullseye AS base

RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/srv/target \
    rustup target add wasm32-unknown-unknown
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/srv/target \
    cargo install wasm-server-runner

WORKDIR /srv/

FROM base AS builder

COPY Cargo.toml /srv/Cargo.toml
COPY Cargo.lock /srv/Cargo.lock
RUN mkdir -p /srv/src/ && touch /srv/src/main.rs
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/srv/target \
    cargo update

COPY src /srv/src
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/srv/target \
    cargo build --target wasm32-unknown-unknown --bin client --features client && \
    mv ./target/wasm32-unknown-unknown/debug/client.wasm ./client.wasm

FROM base AS extractor

RUN apt-get update && apt-get install -y curl wget brotli

WORKDIR /srv/

COPY --from=builder /srv/client.wasm /srv/client.wasm

COPY client/extract_static_content.sh /srv/extract_static_content.sh
COPY client/index.html /srv/index.html

RUN /srv/extract_static_content.sh

FROM nginx:stable

COPY --from=extractor /srv/index.html /usr/share/nginx/html/index.html
COPY --from=extractor /srv/api /usr/share/nginx/html/api

COPY client/default.conf /etc/nginx/conf.d/default.conf
