use bevy::asset::Assets;
use bevy::prelude::{default, shape, Color, ColorMaterial, Commands, Component, Mesh, ResMut, Time, Transform, Vec3};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::Uuid;
use bevy_rapier2d::dynamics::{Ccd, Damping, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties, Friction, Restitution};

use crate::weapon::Weapon;

#[derive(Debug, Component)]
pub struct Projectile {
    pub weapon_uuid: Uuid,
    pub size: Vec3,
    pub created_at: f64,
}

fn get_projectile_material_mesh(
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
) -> MaterialMesh2dBundle<ColorMaterial> {
    return MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 0.0))).into(),
        transform,
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    };
}

pub fn spawn_projectile(
    weapon: &Weapon,
    transform: Transform,
    time: Time,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let projectile = Projectile {
        weapon_uuid: weapon.uuid,
        size: Vec3::new(0.1, 0.1, 0.0), // TODO
        created_at: time.elapsed_seconds_f64(),
    };

    let projectile_mesh = get_projectile_material_mesh(meshes, materials, transform);

    commands
        .spawn((projectile_mesh, projectile))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(0.05, 0.05))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(100.0))
        .insert(Velocity {
            linvel: transform.rotation.mul_vec3(Vec3::new(0.0, 1000.0, 0.0)).truncate(),
            angvel: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        });
}
