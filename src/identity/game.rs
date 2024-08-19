use std::collections::HashSet;

use bevy::prelude::Resource;
use uuid::Uuid;

#[derive(Debug, Clone, Resource)]
pub struct Game {
    pub role: String,
    pub local_player_uuid: Option<Uuid>,
    pub player_uuids: HashSet<Uuid>,
    pub last_update: f64,
    pub server_time_at_join: f64,
    pub client_time_at_join: f64,
}
