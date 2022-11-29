use bevy::asset::Assets;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{default, shape, Color, ColorMaterial, Commands, Component, Mesh, ResMut, Time, Transform};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::{Ccd, Damping, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{Friction, Restitution};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::constants::{
    DEGREES_MAX, FRICTION_COEFFICIENT, MATERIAL_SCALE, PARTICLE_CHANGE_S, PARTICLE_DIMENSION, PARTICLE_EXPIRY_S, PARTICLE_SPEED,
    PARTICLE_Z_INDEX, RESTITUTION_COEFFICIENT, ZERO,
};

#[derive(Debug, Component)]
pub struct Particle {
    pub size: Vec3,
    pub expire_at: f64,
    pub change_at: f64,
}

fn get_particle_material_mesh(
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
) -> MaterialMesh2dBundle<ColorMaterial> {
    let color = *vec![Color::YELLOW, Color::ORANGE, Color::ORANGE_RED, Color::RED]
        .choose(&mut thread_rng())
        .unwrap_or_else(|| &Color::YELLOW);

    return MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Box::new(PARTICLE_DIMENSION, PARTICLE_DIMENSION, ZERO)))
            .into(),
        transform,
        material: materials.add(ColorMaterial::from(color)),
        ..default()
    };
}

pub fn spawn_particle(
    transform: Transform,
    mut velocity: Velocity,
    time: Time,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let particle = Particle {
        size: Vec3::new(PARTICLE_DIMENSION, PARTICLE_DIMENSION, ZERO) * MATERIAL_SCALE, // TODO
        expire_at: time.elapsed_seconds_f64() + PARTICLE_EXPIRY_S + thread_rng().gen::<f64>(),
        change_at: time.elapsed_seconds_f64() + PARTICLE_CHANGE_S,
    };

    let particle_mesh = get_particle_material_mesh(meshes, materials, transform);

    velocity.linvel += transform.rotation.mul_vec3(Vec3::new(ZERO, PARTICLE_SPEED, ZERO)).truncate();

    commands
        .spawn((particle_mesh, particle))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::disabled())
        .insert(Friction::coefficient(FRICTION_COEFFICIENT))
        .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
        .insert(velocity)
        .insert(Damping {
            linear_damping: ZERO,
            angular_damping: ZERO,
        });
}

pub fn spawn_particles(
    count: i8,
    mut transform: Transform,
    velocity: Velocity,
    time: Time,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    transform.rotation = Quat::default();
    transform.translation.z = PARTICLE_Z_INDEX;

    for i in 0..count {
        transform.rotation = Quat::from_rotation_z(f32::to_radians((DEGREES_MAX / count as f32) * i as f32));
        spawn_particle(transform, velocity, time.clone(), commands, meshes, materials)
    }
}
