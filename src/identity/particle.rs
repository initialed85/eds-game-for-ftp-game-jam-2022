use bevy::asset::Assets;
use bevy::ecs::system::EntityCommands;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{
    default, Color, ColorMaterial, Commands, Component, Entity, Mesh, Query, Rectangle, Res,
    ResMut, Time, Transform,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::RigidBody::Dynamic;
use bevy_rapier2d::dynamics::{Sleeping, Velocity};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::behaviour::expireable::Expireable;
use crate::constants::{
    DEGREES_MAX, MATERIAL_SCALE, PARTICLE_CHANGE_RATE_SECONDS, PARTICLE_DIMENSION_MULTIPLIER,
    PARTICLE_EXPIRY_SECONDS, PARTICLE_LINEAR_VELOCITY_CHANGE, ZERO,
};
use crate::identity::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Particle {
    pub particle_uuid: Uuid,
    pub changes_at: f64,
}

pub fn spawn_particle(
    particle_uuid: Uuid,
    _game: &Res<Game>,
    _particle_query: &Query<&Particle>,
    color: Color,
    transform: Transform,
    velocity: Velocity,
    time: Time,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    let mut transform = transform;

    let mut size = Vec3::splat(MATERIAL_SCALE);
    size.y *= PARTICLE_DIMENSION_MULTIPLIER;
    size.x *= PARTICLE_DIMENSION_MULTIPLIER;

    transform.scale = size;

    let mesh = meshes
        .add(Mesh::from(Rectangle::new(
            1.0 * PARTICLE_DIMENSION_MULTIPLIER,
            1.0 * PARTICLE_DIMENSION_MULTIPLIER,
        )))
        .into();

    let material = materials.add(ColorMaterial::from(color));

    let material_mesh = MaterialMesh2dBundle {
        mesh,
        transform,
        material,
        ..default()
    };

    let particle = Particle {
        particle_uuid,
        changes_at: time.elapsed_seconds_f64() + PARTICLE_CHANGE_RATE_SECONDS,
    };

    let expireable = Expireable {
        entity_uuid: particle_uuid,
        expires_at: time.elapsed_seconds_f64()
            + PARTICLE_EXPIRY_SECONDS
            + thread_rng().gen::<f64>(),
    };

    let mut parent: EntityCommands = commands.spawn((material_mesh, particle, expireable));

    parent
        .insert(Dynamic)
        .insert(Sleeping::disabled())
        .insert(velocity);
}

pub fn despawn_particle(
    particle_uuid: Uuid,
    particle_query: &Query<(Entity, &Particle)>,
    _time: Time,
    commands: &mut Commands,
) {
    for (entity, particle) in particle_query.iter() {
        if particle.particle_uuid != particle_uuid {
            continue;
        }

        let entity_commands = commands.get_entity(entity);
        if entity_commands.is_none() {
            continue;
        }

        entity_commands.unwrap().despawn();
    }
}

pub fn handle_particle(mut particle_query: Query<(&mut Particle, &mut Velocity)>, time: Res<Time>) {
    for (mut particle, mut velocity) in particle_query.iter_mut() {
        if time.elapsed_seconds_f64() < particle.changes_at {
            continue;
        }

        let mut transform = Transform::default();
        let rnd: f32 = thread_rng().gen();

        transform.rotation = Quat::from_rotation_z(f32::to_radians(rnd * DEGREES_MAX));
        velocity.linvel += transform
            .rotation
            .mul_vec3(Vec3::new(ZERO, PARTICLE_LINEAR_VELOCITY_CHANGE, ZERO))
            .truncate();

        particle.changes_at = time.elapsed_seconds_f64() + PARTICLE_CHANGE_RATE_SECONDS;
    }
}
