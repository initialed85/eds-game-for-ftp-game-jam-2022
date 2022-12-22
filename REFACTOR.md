# Refactor of eds-game-for-ftp-game-jam-2022

## Goals

- Make it easier to move responsibilities in an experimental way between the Server and the Client
- Make it easier to increase or decrease the rate of updates for player input and network updates
- Minimise the impact of low rates of network updates
- Minimise build duration and maximise runtime performance

## Approaches

- Spend a bit of time planning out some abstractions
    - Try to generalise for reuse, making it easy to do things like
        - Update the position of something
        - Produce / consume network updates about something
- Logically separate things into different modules
- Partition components by behaviour more so than identity
    - Instead of Player being moveable, have something like Moveable; then we could have some flags to control things like:
        - `update_role`; `server` or `client`
- Use events to glue everything
    - Instead of a catalyst thing writing directly to a thing, propagate an event
- Use a general event interface and general event handling to get code reuse
- Lean on Rust Traits to have `from_*` functions to extract things meeting common interfaces from other things

## Model

### Identity-related

```rust
pub struct Projectile {}

pub struct Weapon {}

pub struct Player {
    pub is_this_player: bool,
}
```

### Behaviour-related

```rust
pub struct Expirable {
    pub created_at: f64,
    pub expires_at: f64,
}

pub struct Moveable {}
```

### Event-related

```rust
pub struct 
```
