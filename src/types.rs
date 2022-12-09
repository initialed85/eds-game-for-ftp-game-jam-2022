use bevy::math::Vec3;
use bevy::prelude::{Color, Component, KeyCode, Transform};
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

//
// entities etc
//

#[derive(Debug, Component)]
pub struct Player {
    pub player_uuid: Uuid,
    pub color: Color,
    pub name: String,
    pub forward_key: KeyCode,
    pub backward_key: KeyCode,
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub fire_key: KeyCode,
    pub size: Vec3,
    pub weapon_uuid: Uuid,
    pub is_local: bool,
    pub transform: Transform,
    pub last_update: f64,
}

#[derive(Debug, Component)]
pub struct Projectile {
    pub weapon_uuid: Uuid,
    pub size: Vec3,
    pub expire_at: f64,
    pub has_ricocheted: bool,
}

#[derive(Debug, Component)]
pub struct Particle {
    pub size: Vec3,
    pub expire_at: f64,
    pub change_at: f64,
}

#[derive(Debug)]
pub struct FireWeapon {
    pub weapon_uuid: Uuid,
}

#[derive(Debug, Component)]
pub struct Weapon {
    pub weapon_uuid: Uuid,
    pub last_fired: f64,
}

//
// there's just one flavour of message on the wire
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMessage {
    pub player_uuid: Uuid,
    pub event: String, // "spawn", "despawn", "update"
    pub color: Color,
    pub is_incoming: bool,
    pub is_for_this_player: bool,
    pub translation_x: f32,
    pub translation_y: f32,
    pub translation_z: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub rotation_w: f32,
    pub linvel_x: f32,
    pub linvel_y: f32,
    pub angvel: f32,
    pub has_input: bool,
    pub is_left: bool,
    pub is_right: bool,
    pub is_forward: bool,
    pub is_backward: bool,
    pub is_firing: bool,
}
