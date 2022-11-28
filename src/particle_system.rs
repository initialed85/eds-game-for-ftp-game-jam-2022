use bevy::math::{Quat, Vec3};
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Res, Time, Transform};
use bevy_rapier2d::prelude::Velocity;
use rand::{thread_rng, Rng};

use crate::constants::PARTICLE_CHANGE_S;
use crate::particle::Particle;

pub fn handle_particle(mut query: Query<(Entity, &mut Particle, &mut Velocity)>, time: Res<Time>, mut commands: Commands) {
    let now = time.elapsed_seconds_f64();

    for (entity, mut particle, mut velocity) in query.iter_mut() {
        if now >= particle.expire_at {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        if now < particle.change_at {
            continue;
        }

        let mut transform = Transform::default();
        let rnd: f32 = thread_rng().gen();

        transform.rotation = Quat::from_rotation_z(f32::to_radians(rnd * 360.0));
        velocity.linvel += transform.rotation.mul_vec3(Vec3::new(0.0, 50.0, 0.0)).truncate();
        particle.change_at = time.elapsed_seconds_f64() + PARTICLE_CHANGE_S;
    }
}
