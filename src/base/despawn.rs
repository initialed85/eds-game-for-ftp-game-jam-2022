use bevy::prelude::{Commands, Entity, EventReader, Query, Res, Time};

use crate::identity::entity::despawn_entity;
use crate::identity::particle::Particle;
use crate::identity::player::Player;
use crate::identity::projectile::Projectile;
use crate::types::event::DespawnEvent;

pub fn base_handle_despawn_event(
    mut despawn_event_reader: EventReader<DespawnEvent>,
    player_query: Query<(Entity, &Player)>,
    projectile_query: Query<(Entity, &Projectile)>,
    particle_query: Query<(Entity, &Particle)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for despawn_event in despawn_event_reader.read() {
        despawn_entity(
            despawn_event.clone(),
            &player_query,
            &projectile_query,
            &particle_query,
            *time,
            &mut commands,
        );
    }
}
