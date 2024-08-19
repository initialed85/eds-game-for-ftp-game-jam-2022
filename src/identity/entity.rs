use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Component, Entity, Mesh, Query, Res, ResMut, Time};
use serde::{Deserialize, Serialize};

use crate::identity::game::Game;
use crate::identity::particle::{despawn_particle, spawn_particle, Particle};
use crate::identity::player::{despawn_player, spawn_player, Player};
use crate::identity::projectile::{despawn_projectile, spawn_projectile, Projectile};
use crate::types::event::{DespawnEvent, SpawnEvent};

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Local {}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Remote {}

pub fn spawn_entity(
    spawn: SpawnEvent,
    game: &Res<Game>,
    player_query: &Query<&Player>,
    projectile_query: &Query<&Projectile>,
    particle_query: &Query<&Particle>,
    time: Time,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    if spawn.entity_type == "player" {
        spawn_player(
            spawn.entity_uuid,
            game,
            player_query,
            spawn.color.unwrap(),
            spawn.transform.unwrap().to_transform(),
            spawn.velocity.unwrap().to_velocity(),
            time,
            meshes,
            materials,
            commands,
        );
    } else if spawn.entity_type == "projectile" {
        spawn_projectile(
            spawn.entity_uuid,
            game,
            projectile_query,
            spawn.color.unwrap(),
            spawn.transform.unwrap().to_transform(),
            spawn.velocity.unwrap().to_velocity(),
            time,
            meshes,
            materials,
            commands,
        );
    } else if spawn.entity_type == "particle" {
        spawn_particle(
            spawn.entity_uuid,
            game,
            particle_query,
            spawn.color.unwrap(),
            spawn.transform.unwrap().to_transform(),
            spawn.velocity.unwrap().to_velocity(),
            time,
            meshes,
            materials,
            commands,
        );
    } else {
        panic!(
            "unsupported spawn.entity_type={:?} for spawn={:?}",
            spawn.entity_type, spawn
        );
    }
}

pub fn despawn_entity(
    despawn: DespawnEvent,
    player_query: &Query<(Entity, &Player)>,
    projectile_query: &Query<(Entity, &Projectile)>,
    particle_query: &Query<(Entity, &Particle)>,
    time: Time,
    commands: &mut Commands,
) {
    if despawn.entity_type == "player" {
        despawn_player(despawn.entity_uuid, player_query, time, commands)
    } else if despawn.entity_type == "projectile" {
        despawn_projectile(despawn.entity_uuid, projectile_query, time, commands)
    } else if despawn.entity_type == "particle" {
        despawn_particle(despawn.entity_uuid, particle_query, time, commands)
    } else {
        panic!(
            "unsupported despawn.entity_type={:?} for despawn={:?}",
            despawn.entity_type, despawn
        );
    }
}
