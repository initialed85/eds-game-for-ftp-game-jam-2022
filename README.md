# eds-game-for-ftp-game-jam-2022

What is it? Not sure yet- looks like it's gonna be a bit like a multiplayer Asteroids.

## Context

- [FTP](https://www.ftpsolutions.com.au/) is the place I work
- [One of the team members](https://github.com/shane-smt) suggested [a game jam](https://itch.io/jam/ftp-gamejam)
- I've been dying for an excuse to learn Rust

## Approach

- [Rust](https://www.rust-lang.org/) as programming language
- [Bevy](https://bevyengine.org/) as game engine
- [Rapier](https://rapier.rs/) for physics
- [WebAssembly](https://webassembly.org/) as the runtime
- [WebSockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) as the network transport

## Architecture

### Roles

- Server
    - Runs an x86-built app in a Docker container
    - Uses `xvfb` to make Bevy okay with not having a display
    - Essentially runs the simulation (handles all the physics etc)
    - Consumes inputs from Clients at published rate
    - Publishes updates to Clients at ? Hz via WebSocket
- Client
    - Runs a WASM-built app in a web page
    - Publishes inputs to Server at ? Hz via WebSocket
    - Consumes updates from Server at published rate
    - Sets absolute transform, rotation and velocity of players on each update
    - Runs it's own physics simulation in between updates to provide convincing interpolation

### Deployment

- `xvfb` container
    - Just provides a garbage bin to throw rendered frames into
- `server` container
    - Builds the x86 app and runs it, listening for WebSocket traffic on port 8080
- `client` container
    - Builds the WASM app, briefly runs it using `wasm-server-runner` so it can extract the static content
    - Serves up the static content using Nginx
- `proxy` container
    - Provides a single presence for the static content from the client and WebSocket in the server

### TODO

Things I need to do:

- Reduce the network overhead
    - Send smaller messages / no messages when there are no changes
    - Don't use JSON
    - Don't send messages as frequently
- Reduce the CPU overhead on the server
    - I think it's because I'm naively servicing the WebSocket at 60Hz when I could probably let it buffer for a bit longer
- Calculate projectiles on the server only (will mean more network traffic to describe their movement)
- Disable collsions on the client-side as they _may_ disagree with the server right now

Things I want to do:

- Implement gamepad controls
- Implement controls for mobile somehow
- Add some scoring and other standard game stuff

Things I should but probably won't do:

- Refactor the state / event spaghetti
- Work out why the timestep doesn't behave as expected
- Add some abstractions; e.g.:
    - Rather than have a player, just have a "thing" that can have a translation / rotation / velocity and be able to mark it for inclusion
      in WebSocket updates

## Prerequisites (for macOS at least)

I basically the [getting started page](https://bevyengine.org/learn/book/getting-started/setup/) from the Bevy website.

- [Rust](https://www.rust-lang.org/)
- [zld](https://github.com/michaeleisel/zld)

## Quick start

If you have Docker and Docker Compose, you can simply run the following:

```shell
docker-compose up
```

Then open the game by navigating to [https://127.0.0.1:1334](https://127.0.0.1:1334).

## Development

### One-time

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner
cargo update
```

### While iterating

I have a process that's something like this:

- Run the server natively (on my Mac)
- Run the client using `wasm-server-runner`
- Run the `proxy` container on it's own to provide the single presence
- Access [http://127.0.0.1](http://127.0.0.1) to test my changes

So the commands up being something like:

```shell
# in shell 1
cargo build --target x86_64-apple-darwin --bin server --features server && target/x86_64-apple-darwin/debug/server

# in shell 2
export WASM_SERVER_RUNNER_ADDRESS=0.0.0.0; cargo build --target wasm32-unknown-unknown --bin client --features client && wasm-server-runner target/wasm32-unknown-unknown/debug/client.wasm

# in shell 3
cd proxy && ./build.sh && PROXY_PORT=80 ./deploy.sh
```
