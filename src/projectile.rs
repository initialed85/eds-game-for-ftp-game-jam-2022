use bevy::asset::Assets;
use bevy::prelude::{default, shape, Color, ColorMaterial, Commands, Component, Mesh, ResMut, Time, Transform, Vec3};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::Uuid;
use bevy_rapier2d::dynamics::{Ccd, Damping, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, ColliderMassProperties, Friction, Restitution};

use crate::constants::{
    FRICTION_COEFFICIENT, MATERIAL_SCALE, PROJECTILE_DENSITY, PROJECTILE_DIMENSION, PROJECTILE_EXPIRY, PROJECTILE_SPEED,
    PROJECTILE_Z_INDEX, RESTITUTION_COEFFICIENT, ZERO,
};
use crate::weapon::Weapon;

#[derive(Debug, Component)]
pub struct Projectile {
    pub weapon_uuid: Uuid,
    pub size: Vec3,
    pub expire_at: f64,
    pub has_ricocheted: bool,
}

fn get_projectile_material_mesh(
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
) -> MaterialMesh2dBundle<ColorMaterial> {
    return MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Box::new(PROJECTILE_DIMENSION, PROJECTILE_DIMENSION, ZERO)))
            .into(),
        transform,
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    };
}

pub fn spawn_projectile(
    weapon: &Weapon,
    transform: Transform,
    mut velocity: Velocity,
    time: Time,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let projectile = Projectile {
        weapon_uuid: weapon.uuid,
        size: Vec3::new(PROJECTILE_DIMENSION, PROJECTILE_DIMENSION, ZERO) * MATERIAL_SCALE,
        expire_at: time.elapsed_seconds_f64() + PROJECTILE_EXPIRY,
        has_ricocheted: false,
    };

    let projectile_mesh = get_projectile_material_mesh(meshes, materials, transform);

    velocity.linvel += transform
        .rotation
        .mul_vec3(Vec3::new(ZERO, PROJECTILE_SPEED, PROJECTILE_Z_INDEX))
        .truncate();

    commands
        .spawn((projectile_mesh, projectile))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::disabled())
        .insert(Collider::cuboid(PROJECTILE_DIMENSION, PROJECTILE_DIMENSION))
        .insert(Friction::coefficient(FRICTION_COEFFICIENT))
        .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
        .insert(ColliderMassProperties::Density(PROJECTILE_DENSITY))
        .insert(velocity)
        .insert(Damping {
            linear_damping: ZERO,
            angular_damping: ZERO,
        })
        .insert(ActiveEvents::all());
}
