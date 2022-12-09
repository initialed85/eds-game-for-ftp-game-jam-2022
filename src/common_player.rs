use crate::constants::{MATERIAL_SCALE, PLAYER_HEIGHT_MULTIPLIER, PLAYER_WIDTH_MULTIPLIER};
use crate::types::{Player, Weapon};
use crate::weapon::get_weapon;
use bevy::math::Vec3;
use bevy::prelude::{Color, KeyCode, Transform};
use bevy::reflect::Uuid;

pub fn get_player_and_weapon(
    player_uuid: Uuid,
    color: Color,
    name: String,
    forward_key: KeyCode,
    backward_key: KeyCode,
    left_key: KeyCode,
    right_key: KeyCode,
    fire_key: KeyCode,
    is_local: bool,
    transform: Transform,
) -> (Player, Weapon) {
    let weapon = get_weapon();

    let mut size = Vec3::splat(MATERIAL_SCALE);
    size.y *= PLAYER_HEIGHT_MULTIPLIER;
    size.x *= PLAYER_WIDTH_MULTIPLIER;

    let player = Player {
        player_uuid,
        color,
        name,
        forward_key,
        backward_key,
        left_key,
        right_key,
        fire_key,
        size,
        weapon_uuid: weapon.weapon_uuid,
        is_local,
        transform,
        last_update: 0.0,
    };

    return (player, weapon);
}
