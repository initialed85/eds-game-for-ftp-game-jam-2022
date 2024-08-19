use bevy::prelude::Event;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::behaviour::collideable::CollisionEvent;
use crate::types::event::{
    DespawnEvent, InputEvent, JoinEvent, LeaveEvent, SpawnEvent, UpdateEvent,
};

//
// For the WebSocket layer
//

#[derive(Event, Debug, Clone)]
pub struct OpenEvent {
    pub session_uuid: Uuid,
}

#[derive(Event, Debug, Clone)]
pub struct IncomingMessageEvent {
    pub session_uuid: Uuid,
    pub message: Vec<u8>,
}

#[derive(Event, Debug, Clone)]
pub struct OutgoingMessageEvent {
    pub session_uuid: Option<Uuid>,
    pub not_session_uuid: Option<Uuid>,
    pub message: Vec<u8>, // a serialized Container
}

#[derive(Event, Debug, Clone)]
pub struct CloseEvent {
    pub session_uuid: Uuid,
}

//
// For serializing the Game layer into the WebSocket layer
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub message_type: String,
    // one of "join", "spawn", "input", "update", "despawn", "leave", "collision"
    pub join: Option<JoinEvent>,
    pub spawn: Option<SpawnEvent>,
    pub input: Option<InputEvent>,
    pub update: Option<UpdateEvent>,
    pub despawn: Option<DespawnEvent>,
    pub leave: Option<LeaveEvent>,
    pub collision: Option<CollisionEvent>,
}
