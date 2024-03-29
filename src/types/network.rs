use bevy::prelude::Event;
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

use crate::behaviour::collideable::Collision;
use crate::types::event::{Despawn, Input, Join, Leave, Spawn, Update};

//
// For the WebSocket layer
//

#[derive(Debug, Clone, Event)]
pub struct Open {
    pub session_uuid: Uuid,
}

#[derive(Debug, Clone, Event)]
pub struct IncomingMessage {
    pub session_uuid: Uuid,
    pub message: Vec<u8>,
}

#[derive(Debug, Clone, Event)]
pub struct OutgoingMessage {
    pub session_uuid: Option<Uuid>,
    pub not_session_uuid: Option<Uuid>,
    pub message: Vec<u8>, // a serialized Container
}

#[derive(Debug, Clone, Event)]
pub struct Close {
    pub session_uuid: Uuid,
}

//
// For serializing the Game layer into the WebSocket layer
//

#[derive(Debug, Clone, Event, Serialize, Deserialize)]
pub struct Container {
    pub message_type: String,
    // one of "join", "spawn", "input", "update", "despawn", "leave", "collision"
    pub join: Option<Join>,
    pub spawn: Option<Spawn>,
    pub input: Option<Input>,
    pub update: Option<Update>,
    pub despawn: Option<Despawn>,
    pub leave: Option<Leave>,
    pub collision: Option<Collision>,
}
