use bevy::math::{Quat, Vec3};
use bevy::prelude::{Color, Component, EventReader, EventWriter, Query, Transform};
use bevy::utils::Uuid;
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::prelude::Velocity;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::constants::{DEGREES_MAX, PARTICLE_LINEAR_VELOCITY, ZERO};
use crate::identity::player::Player;
use crate::identity::projectile::Projectile;
use crate::types::event::{SerializableTransform, SerializableVelocity, Spawn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collider {
    entity_uuid: Uuid,
    entity_type: String,
    transform: Option<SerializableTransform>,
    velocity: Option<SerializableVelocity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collision {
    pub collider_a: Collider,
    pub collider_b: Collider,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Collideable {
    pub entity_uuid: Uuid,
}

pub fn handle_rapier_collision_event(
    mut rapier_collision_event_reader: EventReader<CollisionEvent>,
    query: Query<(
        &Collideable,
        Option<&Transform>,
        Option<&Velocity>,
        Option<&Player>,
        Option<&Projectile>,
    )>,
    mut collision_event_writer: EventWriter<Collision>,
) {
    for rapier_collision_event in rapier_collision_event_reader.iter() {
        let mut entity_a = None;
        let mut entity_b = None;

        let _ = match rapier_collision_event {
            CollisionEvent::Started(_entity_a, _entity_b, _) => {
                entity_a = Some(_entity_a);
                entity_b = Some(_entity_b);
            }
            CollisionEvent::Stopped(_, _, _) => {}
        };

        if entity_a.is_none() || entity_b.is_none() {
            continue;
        }

        let entity_a = entity_a.unwrap();
        let entity_b = entity_b.unwrap();

        let result_a = query.get(entity_a.clone());
        let result_b = query.get(entity_b.clone());

        if result_a.is_err() || result_b.is_err() {
            continue;
        }

        let (collideable_a, _transform_a, _velocity_a, player_a, projectile_a) = result_a.unwrap();
        let (collideable_b, _transform_b, _velocity_b, player_b, projectile_b) = result_b.unwrap();

        let mut transform_a = None;
        if _transform_a.is_some() {
            transform_a = Some(SerializableTransform::from_transform(
                _transform_a.unwrap().clone(),
            ));
        }

        let mut velocity_a = None;
        if _velocity_a.is_some() {
            velocity_a = Some(SerializableVelocity::from_velocity(_velocity_a.unwrap().clone()));
        }

        let mut transform_b = None;
        if _transform_b.is_some() {
            transform_b = Some(SerializableTransform::from_transform(
                _transform_b.unwrap().clone(),
            ));
        }

        let mut velocity_b = None;
        if _velocity_b.is_some() {
            velocity_b = Some(SerializableVelocity::from_velocity(_velocity_b.unwrap().clone()));
        }

        let entity_type_a;
        if player_a.is_some() {
            entity_type_a = "player"
        } else if projectile_a.is_some() {
            entity_type_a = "projectile"
        } else {
            panic!(
                "failed to infer entity_type_a from player_a={:?} and projectile_a={:?}",
                player_a, projectile_a
            );
        }

        let entity_type_b;
        if player_b.is_some() {
            entity_type_b = "player"
        } else if projectile_b.is_some() {
            entity_type_b = "projectile"
        } else {
            panic!(
                "failed to infer entity_type_b from player_b={:?} and projectile_b={:?}",
                player_b, projectile_b
            );
        }

        collision_event_writer.send(Collision {
            collider_a: Collider {
                entity_uuid: collideable_a.entity_uuid,
                entity_type: entity_type_a.to_string(),
                transform: transform_a,
                velocity: velocity_a,
            },
            collider_b: Collider {
                entity_uuid: collideable_b.entity_uuid,
                entity_type: entity_type_b.to_string(),
                transform: transform_b,
                velocity: velocity_b,
            },
        })
    }
}

pub fn handle_collision_event(
    mut collision_event_reader: EventReader<Collision>,
    mut spawn_event_writer: EventWriter<Spawn>,
) {
    for collision in collision_event_reader.iter() {
        let mut transform_a = collision.clone().collider_a.transform.unwrap().to_transform();

        transform_a.rotation = Quat::default();

        for i in 0..8 {
            transform_a.rotation =
                Quat::from_rotation_z(f32::to_radians((DEGREES_MAX / 8 as f32) * i as f32));

            let mut velocity_a = Velocity::zero();

            velocity_a.linvel += transform_a
                .rotation
                .mul_vec3(Vec3::new(ZERO, PARTICLE_LINEAR_VELOCITY, ZERO))
                .truncate();

            let mut color = Color::CYAN;

            if collision.collider_a.entity_type == "player" || collision.collider_b.entity_type == "player" {
                color = *vec![Color::YELLOW, Color::ORANGE, Color::ORANGE_RED, Color::RED]
                    .choose(&mut thread_rng())
                    .unwrap_or_else(|| &Color::YELLOW);
            }

            spawn_event_writer.send(Spawn {
                entity_uuid: Uuid::new_v4(),
                entity_type: "particle".to_string(),
                transform: Some(SerializableTransform::from_transform(transform_a)),
                velocity: Some(SerializableVelocity::from_velocity(velocity_a)),
                color: Some(color),
            });
        }
    }
}
