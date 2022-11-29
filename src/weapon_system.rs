use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Input, KeyCode, Mesh, Query, Res, ResMut, Time, Transform, Vec3};
use bevy_rapier2d::dynamics::Velocity;

use crate::constants::{PROJECTILE_SPAWN_OFFSET, PROJECTILE_Z_INDEX, ZERO};
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
    let offset = Vec3::new(ZERO, PROJECTILE_SPAWN_OFFSET, PROJECTILE_Z_INDEX);

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
