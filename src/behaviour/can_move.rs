use bevy::prelude::{Component, Query, Transform};
use bevy_rapier2d::prelude::Velocity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct CanMove {}

pub fn base_handle_can_move(can_move_query: Query<(&CanMove, &Transform, &Velocity)>) {
    for (can_move, transform, velocity) in can_move_query.iter() {
        _ = (can_move, transform, velocity);
    }
}
