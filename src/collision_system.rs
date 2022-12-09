use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Entity, EventReader, Mesh, Query, Res, ResMut, Time, Transform};
use bevy_rapier2d::prelude::{CollisionEvent, Velocity};

use crate::constants::{
    PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PLAYERS, PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PROJECTILES, PARTICLE_COUNT_FOR_PROJECTILES,
    PROJECTILE_RICOCHET_EXPIRY,
};
use crate::particle::spawn_particles_at_client;
use crate::types::Player;
use crate::types::Projectile;

// TODO: work out how to refactor this, borrow checker defeating me
pub fn handle_collision_at_client(
    mut projectile_query: Query<(Entity, &mut Projectile)>,
    mut player_query: Query<(Entity, &Player)>,
    mut transform_and_velocity_query: Query<(Entity, &Transform, &Velocity)>,
    mut collision_events: EventReader<CollisionEvent>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                let mut projectiles_involved = false;
                let mut only_projectiles = false;

                let mut projectile_item_1 = projectile_query.get_mut(*entity_1);
                if projectile_item_1.is_ok() {
                    projectiles_involved = true;

                    let mut projectile = projectile_item_1.unwrap().1;
                    projectile.expire_at = time.elapsed_seconds_f64() + PROJECTILE_RICOCHET_EXPIRY;
                    projectile.has_ricocheted = true;

                    let transform_and_velocity_item = transform_and_velocity_query.get_mut(*entity_1);

                    if transform_and_velocity_item.is_ok() {
                        let (_, transform, velocity) = transform_and_velocity_item.unwrap();
                        spawn_particles_at_client(
                            PARTICLE_COUNT_FOR_PROJECTILES,
                            transform.clone(),
                            velocity.clone(),
                            time.clone(),
                            commands.borrow_mut(),
                            meshes.borrow_mut(),
                            materials.borrow_mut(),
                        );
                    }
                }

                let mut projectile_item_2 = projectile_query.get_mut(*entity_2);
                if projectile_item_2.is_ok() {
                    only_projectiles = projectiles_involved;
                    projectiles_involved = true;

                    let mut projectile = projectile_item_2.unwrap().1;
                    projectile.expire_at = time.elapsed_seconds_f64() + PROJECTILE_RICOCHET_EXPIRY;
                    projectile.has_ricocheted = true;

                    let transform_and_velocity_item = transform_and_velocity_query.get_mut(*entity_2);

                    if transform_and_velocity_item.is_ok() {
                        let (_, transform, velocity) = transform_and_velocity_item.unwrap();
                        spawn_particles_at_client(
                            PARTICLE_COUNT_FOR_PROJECTILES,
                            transform.clone(),
                            velocity.clone(),
                            time.clone(),
                            commands.borrow_mut(),
                            meshes.borrow_mut(),
                            materials.borrow_mut(),
                        );
                    }
                }

                let mut particle_count = PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PLAYERS;
                if projectiles_involved {
                    particle_count = PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PROJECTILES;
                }

                let player_item_1 = player_query.get_mut(*entity_1);
                if player_item_1.is_ok() {
                    let transform_and_velocity_item = transform_and_velocity_query.get_mut(*entity_1);
                    if transform_and_velocity_item.is_ok() {
                        let (_, transform, velocity) = transform_and_velocity_item.unwrap();
                        spawn_particles_at_client(
                            particle_count,
                            *transform,
                            *velocity,
                            time.clone(),
                            commands.borrow_mut(),
                            meshes.borrow_mut(),
                            materials.borrow_mut(),
                        );
                    }
                }

                let player_item_2 = player_query.get_mut(*entity_2);
                if player_item_2.is_ok() {
                    let transform_and_velocity_item = transform_and_velocity_query.get_mut(*entity_2);
                    if transform_and_velocity_item.is_ok() {
                        let (_, transform, velocity) = transform_and_velocity_item.unwrap();
                        spawn_particles_at_client(
                            particle_count,
                            *transform,
                            *velocity,
                            time.clone(),
                            commands.borrow_mut(),
                            meshes.borrow_mut(),
                            materials.borrow_mut(),
                        );
                    }
                }

                if only_projectiles {
                    projectile_item_1 = projectile_query.get_mut(*entity_1);
                    if projectile_item_1.is_ok() {
                        let mut projectile = projectile_item_1.unwrap().1;
                        projectile.expire_at = time.elapsed_seconds_f64();
                    }

                    projectile_item_2 = projectile_query.get_mut(*entity_2);
                    if projectile_item_2.is_ok() {
                        let mut projectile = projectile_item_2.unwrap().1;
                        projectile.expire_at = time.elapsed_seconds_f64();
                    }
                }
            }
            CollisionEvent::Stopped(_entity_1, _entity_2, _flags) => {
                // noop
            }
        }
    }
}

pub fn handle_collision_at_server(
    mut projectile_query: Query<(Entity, &mut Projectile)>,
    mut player_query: Query<(Entity, &Player)>,
    mut collision_events: EventReader<CollisionEvent>,
    time: Res<Time>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                let mut projectiles_involved = false;
                let mut only_projectiles = false;

                let mut projectile_item_1 = projectile_query.get_mut(*entity_1);
                if projectile_item_1.is_ok() {
                    projectiles_involved = true;

                    let mut projectile = projectile_item_1.unwrap().1;
                    projectile.expire_at = time.elapsed_seconds_f64() + PROJECTILE_RICOCHET_EXPIRY;
                    projectile.has_ricocheted = true;
                }

                let mut projectile_item_2 = projectile_query.get_mut(*entity_2);
                if projectile_item_2.is_ok() {
                    only_projectiles = projectiles_involved;
                    projectiles_involved = true;

                    let mut projectile = projectile_item_2.unwrap().1;
                    projectile.expire_at = time.elapsed_seconds_f64() + PROJECTILE_RICOCHET_EXPIRY;
                    projectile.has_ricocheted = true;
                }

                let mut particle_count = PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PLAYERS;
                if projectiles_involved {
                    particle_count = PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PROJECTILES;
                }

                let player_item_1 = player_query.get_mut(*entity_1);
                if player_item_1.is_ok() {}

                let player_item_2 = player_query.get_mut(*entity_2);
                if player_item_2.is_ok() {}

                if only_projectiles {
                    projectile_item_1 = projectile_query.get_mut(*entity_1);
                    if projectile_item_1.is_ok() {
                        let mut projectile = projectile_item_1.unwrap().1;
                        projectile.expire_at = time.elapsed_seconds_f64();
                    }

                    projectile_item_2 = projectile_query.get_mut(*entity_2);
                    if projectile_item_2.is_ok() {
                        let mut projectile = projectile_item_2.unwrap().1;
                        projectile.expire_at = time.elapsed_seconds_f64();
                    }
                }
            }
            CollisionEvent::Stopped(_entity_1, _entity_2, _flags) => {
                // noop
            }
        }
    }
}
