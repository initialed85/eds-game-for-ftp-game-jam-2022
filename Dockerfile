FROM rust:1.65.0-slim-bullseye AS base

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-server-runner

WORKDIR /srv/

FROM base AS builder

COPY Cargo.toml /srv/Cargo.toml
COPY Cargo.lock /srv/Cargo.lock
RUN mkdir -p /srv/src/ && touch /srv/src/main.rs
RUN cargo update

COPY src /srv/src
RUN cargo build --target wasm32-unknown-unknown

FROM base AS runner

WORKDIR /srv/

COPY --from=builder /srv/target /srv/target

ENV WASM_SERVER_RUNNER_ADDRESS=0.0.0.0

EXPOSE 1334

ENTRYPOINT []
CMD ["wasm-server-runner", "target/wasm32-unknown-unknown/debug/eds-game-for-ftp-game-jam-2022.wasm"]
