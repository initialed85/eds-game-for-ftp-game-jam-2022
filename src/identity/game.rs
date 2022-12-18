use std::collections::HashSet;

use bevy::prelude::Resource;
use bevy::utils::Uuid;

#[derive(Debug, Clone, Resource)]
pub struct Game {
    pub role: String,
    pub local_player_uuid: Option<Uuid>,
    pub player_uuids: HashSet<Uuid>,
    pub last_update: f64,
}
