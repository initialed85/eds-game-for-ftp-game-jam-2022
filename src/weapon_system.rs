use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, EventReader, Mesh, Query, Res, ResMut, Time, Transform, Vec3};
use bevy_rapier2d::dynamics::Velocity;

use crate::constants::{PROJECTILE_SPAWN_OFFSET, PROJECTILE_Z_INDEX, ZERO};
use crate::types::Weapon;
use crate::types::{FireWeapon, Player};

pub fn handle_player_weapon_at_client(
    mut fire_weapon_reader: EventReader<FireWeapon>,
    mut query: Query<(&mut Player, &mut Weapon, &Transform, &Velocity)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let offset = Vec3::new(ZERO, PROJECTILE_SPAWN_OFFSET, PROJECTILE_Z_INDEX);

    for fire_weapon_event in fire_weapon_reader.iter() {
        for (_player, mut weapon, transform, velocity) in query.iter_mut() {
            if weapon.weapon_uuid != fire_weapon_event.weapon_uuid {
                continue;
            }

            weapon.fire_at_client(
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
}

pub fn handle_player_weapon_at_server(
    mut fire_weapon_reader: EventReader<FireWeapon>,
    mut query: Query<(&mut Player, &mut Weapon, &Transform, &Velocity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let offset = Vec3::new(ZERO, PROJECTILE_SPAWN_OFFSET, PROJECTILE_Z_INDEX);

    for fire_weapon_event in fire_weapon_reader.iter() {
        for (_player, mut weapon, transform, velocity) in query.iter_mut() {
            if weapon.weapon_uuid != fire_weapon_event.weapon_uuid {
                continue;
            }

            weapon.fire_at_server(time.clone(), transform, velocity.clone(), offset.clone(), commands.borrow_mut());
        }
    }
}
