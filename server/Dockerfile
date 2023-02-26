FROM --platform=linux/amd64 ubuntu:22.04 AS base

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev \
    dbus-x11 libxv1 mesa-utils mesa-utils-extra psmisc procps libvulkan-dev

ENV LANG en_US.UTF-8
RUN echo $LANG UTF-8 > /etc/locale.gen && \
    apt-get install -y locales && \
    update-locale --reset LANG=$LANG

RUN apt-get install -y --no-install-recommends xauth xinit x11-xserver-utils && \
    apt-get install -y xwayland

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh && \
    chmod +x install.sh && \
    ./install.sh -y

ENV PATH=${PATH}:/root/.cargo/bin/

RUN rustup update

RUN apt-get install -y dbus-x11 libxv1 mesa-utils mesa-utils-extra psmisc procps

WORKDIR /srv/

FROM --platform=linux/amd64 base AS builder

COPY Cargo.toml /srv/Cargo.toml
COPY Cargo.lock /srv/Cargo.lock
RUN mkdir -p /srv/src/ && touch /srv/src/main.rs
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/srv/target \
    cargo update

RUN mkdir -p /srv/target

COPY src /srv/src
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/srv/target \
    cargo build --target x86_64-unknown-linux-gnu --bin server --features server --profile release && \
    mv ./target/x86_64-unknown-linux-gnu/release/server ./server

FROM --platform=linux/amd64 base AS runner

WORKDIR /srv/

COPY --from=builder /srv/server /srv/server

EXPOSE 8080

STOPSIGNAL SIGINT

ENTRYPOINT []
CMD ["./server"]
