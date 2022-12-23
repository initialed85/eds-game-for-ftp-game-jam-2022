use bevy::prelude::Component;
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

use crate::client::error::{QuatEMA, Vec2EMA, Vec3EMA, EMA};
use crate::types::event::Update;

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Moveable {
    pub entity_uuid: Uuid,
    pub unhandled_updates: Vec<Update>,
    pub update_to_handle: Option<Update>,
    pub translation_error: Vec3EMA,
    pub rotation_error: QuatEMA,
    pub linvel_error: Vec2EMA,
    pub angvel_error: EMA,
    pub update_rate_seconds: f64,
    pub last_update_handled_at: f64,
    pub had_rollover: bool,
}
