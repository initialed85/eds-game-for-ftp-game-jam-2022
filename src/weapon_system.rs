use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Input, KeyCode, Mesh, Query, Res, ResMut, Time, Transform, Vec3};
use bevy_rapier2d::dynamics::Velocity;

use crate::constants::MATERIAL_SCALE;
use crate::player::Player;
use crate::weapon::Weapon;

pub fn handle_player_weapon(
    mut query: Query<(&Player, &mut Weapon, &Transform, &Velocity)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let offset = Vec3::new(0.0, MATERIAL_SCALE, 0.0);

    for (player, mut weapon, transform, velocity) in query.iter_mut() {
        if !keyboard_input.pressed(player.fire_key) {
            continue;
        }

        weapon.fire(
            time.clone(),
            transform,
            velocity.clone(),
            offset.clone(),
            commands.borrow_mut(),
            meshes.borrow_mut(),
            materials.borrow_mut(),
        );
    }
}
