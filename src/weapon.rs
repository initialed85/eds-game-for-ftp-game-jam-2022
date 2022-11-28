use std::borrow::Borrow;

use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Commands, Component, Mesh, ResMut, Time, Transform};
use bevy::utils::Uuid;

use crate::constants::WEAPON_FIRE_INTERVAL_S;
use crate::projectile::spawn_projectile;

#[derive(Debug, Component)]
pub struct Weapon {
    pub uuid: Uuid,
    pub last_fired: f64,
}

pub fn get_weapon() -> Weapon {
    return Weapon {
        uuid: Uuid::new_v4(),
        last_fired: 0.0,
    };
}

impl Weapon {
    pub fn fire(
        &mut self,
        time: Time,
        transform: &Transform,
        offset: Vec3,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let now = time.elapsed_seconds_f64();

        if now < self.last_fired + WEAPON_FIRE_INTERVAL_S {
            return;
        }

        let rotated_offset = transform.rotation.mul_vec3(offset);

        let mut projectile_transform = transform.clone();
        projectile_transform.translation += rotated_offset;

        spawn_projectile(self.borrow(), projectile_transform, time, commands, meshes, materials);

        self.last_fired = now;
    }
}
