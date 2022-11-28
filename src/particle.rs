use bevy::asset::Assets;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{default, shape, Color, ColorMaterial, Commands, Component, Mesh, ResMut, Time, Transform};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::{Ccd, Damping, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{Friction, Restitution};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::constants::{FRICTION_COEFFICIENT, MATERIAL_SCALE, PARTICLE_CHANGE_S, PARTICLE_EXPIRY_S, RESTITUTION_COEFFICIENT};

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
        mesh: meshes.add(Mesh::from(shape::Box::new(0.05, 0.05, 0.0))).into(),
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
        size: Vec3::new(0.05, 0.05, 0.0) * MATERIAL_SCALE / 2.0, // TODO
        expire_at: time.elapsed_seconds_f64() + PARTICLE_EXPIRY_S + (thread_rng().gen::<f64>() * 1.0),
        change_at: time.elapsed_seconds_f64() + PARTICLE_CHANGE_S,
    };

    let particle_mesh = get_particle_material_mesh(meshes, materials, transform);

    velocity.linvel += transform.rotation.mul_vec3(Vec3::new(0.0, 250.0, 0.0)).truncate();

    commands
        .spawn((particle_mesh, particle))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::disabled())
        .insert(Friction::coefficient(FRICTION_COEFFICIENT))
        .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
        .insert(velocity)
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
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

    for i in 0..count {
        transform.rotation = Quat::from_rotation_z(f32::to_radians((360.0 / count as f32) * i as f32));
        spawn_particle(transform, velocity, time.clone(), commands, meshes, materials)
    }
}
