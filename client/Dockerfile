FROM rust:1.65.0-slim-bullseye AS base

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-server-runner

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
    cargo build --target wasm32-unknown-unknown --bin client --features client --profile release && \
    mv ./target/wasm32-unknown-unknown/release/client.wasm ./client.wasm

FROM base AS extractor

RUN apt-get update && apt-get install -y curl wget brotli

WORKDIR /srv/

COPY --from=builder /srv/client.wasm /srv/client.wasm

COPY client/extract_static_content.sh /srv/extract_static_content.sh
COPY client/index.html /srv/index.html

RUN /srv/extract_static_content.sh

FROM --platform=linux/amd64 nginx:stable

COPY --from=extractor /srv/index.html /usr/share/nginx/html/index.html
COPY --from=extractor /srv/api /usr/share/nginx/html/api

COPY client/default.conf /etc/nginx/conf.d/default.conf

EXPOSE 1334
