use bevy::prelude::{Color, Event, Quat, Transform, Vec2, Vec3};
use bevy_rapier2d::prelude::Velocity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct JoinEvent {
    pub player_uuid: Uuid,
    pub is_for_local_player: bool,
    pub server_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl SerializableTransform {
    pub fn from_transform(transform: Transform) -> SerializableTransform {
        SerializableTransform {
            translation: transform.translation,
            rotation: transform.rotation,
            scale: transform.scale,
        }
    }

    pub fn to_transform(self: &SerializableTransform) -> Transform {
        let _self = self.clone();

        Transform {
            translation: _self.translation,
            rotation: _self.rotation,
            scale: _self.scale,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableVelocity {
    pub linvel: Vec2,
    pub angvel: f32,
}

impl SerializableVelocity {
    pub fn from_velocity(velocity: Velocity) -> SerializableVelocity {
        SerializableVelocity {
            linvel: Vec2::new(velocity.linvel.x, velocity.linvel.y),
            angvel: velocity.angvel,
        }
    }

    pub fn to_velocity(self: &SerializableVelocity) -> Velocity {
        let _self = self.clone();

        Velocity {
            linvel: _self.linvel,
            angvel: _self.angvel,
        }
    }
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct SpawnEvent {
    pub entity_uuid: Uuid,
    // e.g. "player", "projectile" etc
    pub entity_type: String,
    pub transform: Option<SerializableTransform>,
    pub velocity: Option<SerializableVelocity>,
    pub color: Option<Color>,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub player_uuid: Uuid,
    pub is_left: bool,
    pub is_right: bool,
    pub is_forward: bool,
    pub is_backward: bool,
    pub is_firing: bool,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub entity_uuid: Uuid,
    pub server_time: f64,
    pub transform: Option<SerializableTransform>,
    pub velocity: Option<SerializableVelocity>,
    pub includes_rollover: bool,
    pub handled_at: Option<f64>,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct DespawnEvent {
    pub entity_uuid: Uuid,
    // one of "player" or "projectile"
    pub entity_type: String,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct LeaveEvent {
    pub player_uuid: Uuid,
}
