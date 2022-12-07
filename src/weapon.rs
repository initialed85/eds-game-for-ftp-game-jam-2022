use std::borrow::Borrow;

use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Commands, Mesh, ResMut, Time, Transform};
use bevy::utils::Uuid;
use bevy_rapier2d::dynamics::Velocity;

use crate::constants::{MATERIAL_SCALE, WEAPON_FIRE_INTERVAL};
use crate::projectile::{spawn_projectile_at_client, spawn_projectile_at_server};
use crate::types::Weapon;

pub fn get_weapon() -> Weapon {
    return Weapon {
        weapon_uuid: Uuid::new_v4(),
        last_fired: f64::default(),
    };
}

impl Weapon {
    pub fn fire_at_client(
        &mut self,
        time: Time,
        transform: &Transform,
        velocity: Velocity,
        offset: Vec3,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let now = time.elapsed_seconds_f64();

        if now < self.last_fired + WEAPON_FIRE_INTERVAL {
            return;
        }

        let rotated_offset = transform.rotation.mul_vec3(offset);

        let mut projectile_transform = transform.clone();
        projectile_transform.scale = Vec3::splat(MATERIAL_SCALE);
        projectile_transform.translation += rotated_offset;

        spawn_projectile_at_client(self.borrow(), projectile_transform, velocity, time, commands, meshes, materials);

        self.last_fired = now;
    }

    pub fn fire_at_server(&mut self, time: Time, transform: &Transform, velocity: Velocity, offset: Vec3, commands: &mut Commands) {
        let now = time.elapsed_seconds_f64();

        if now < self.last_fired + WEAPON_FIRE_INTERVAL {
            return;
        }

        let rotated_offset = transform.rotation.mul_vec3(offset);

        let mut projectile_transform = transform.clone();
        projectile_transform.scale = Vec3::splat(MATERIAL_SCALE);
        projectile_transform.translation += rotated_offset;

        spawn_projectile_at_server(self.borrow(), projectile_transform, velocity, time, commands);

        self.last_fired = now;
    }
}
