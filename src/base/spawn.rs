use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, EventReader, Mesh, Query, Res, ResMut, Time};

use crate::identity::entity::spawn_entity;
use crate::identity::game::Game;
use crate::identity::particle::Particle;
use crate::identity::player::Player;
use crate::identity::projectile::Projectile;
use crate::types::event::Spawn;

pub fn base_handle_spawn_event(
    mut spawn_event_reader: EventReader<Spawn>,
    game: Res<Game>,
    player_query: Query<&Player>,
    projectile_query: Query<&Projectile>,
    particle_query: Query<&Particle>,
    time: Res<Time>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_event_reader.iter() {
        spawn_entity(
            spawn_event.clone(),
            &game,
            &player_query,
            &projectile_query,
            &particle_query,
            time.clone(),
            &mut meshes,
            &mut materials,
            &mut commands,
        );
    }
}
