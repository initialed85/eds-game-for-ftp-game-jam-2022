use bevy::asset::Assets;
use bevy::prelude::{default, shape, Color, ColorMaterial, Commands, Mesh, ResMut, Time, Transform, Vec3};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::{Ccd, Damping, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, ColliderMassProperties, Friction, Restitution};

use crate::constants::{
    FRICTION_COEFFICIENT, MATERIAL_SCALE, PROJECTILE_DENSITY, PROJECTILE_DIMENSION, PROJECTILE_EXPIRY, PROJECTILE_SPEED,
    PROJECTILE_Z_INDEX, RESTITUTION_COEFFICIENT, ZERO,
};
use crate::types::Projectile;
use crate::types::Weapon;

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

pub fn spawn_projectile_at_client(
    weapon: &Weapon,
    transform: Transform,
    mut velocity: Velocity,
    time: Time,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let projectile = Projectile {
        weapon_uuid: weapon.weapon_uuid,
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

pub fn spawn_projectile_at_server(weapon: &Weapon, transform: Transform, mut velocity: Velocity, time: Time, commands: &mut Commands) {
    let projectile = Projectile {
        weapon_uuid: weapon.weapon_uuid,
        size: Vec3::new(PROJECTILE_DIMENSION, PROJECTILE_DIMENSION, ZERO) * MATERIAL_SCALE,
        expire_at: time.elapsed_seconds_f64() + PROJECTILE_EXPIRY,
        has_ricocheted: false,
    };

    velocity.linvel += transform
        .rotation
        .mul_vec3(Vec3::new(ZERO, PROJECTILE_SPEED, PROJECTILE_Z_INDEX))
        .truncate();

    commands
        .spawn(projectile)
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
