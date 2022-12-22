use bevy::prelude::{Component, Entity, EventWriter, Query, Res, Time};
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

use crate::constants::TIME_STEP;
use crate::identity::particle::Particle;
use crate::identity::player::Player;
use crate::identity::projectile::Projectile;
use crate::types::event::Despawn;

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Expireable {
    pub entity_uuid: Uuid,
    pub expires_at: f64,
}

pub fn handle_expireable(
    time: Res<Time>,
    mut expireable_query: Query<(
        Entity,
        &Expireable,
        Option<&Player>,
        Option<&Projectile>,
        Option<&Particle>,
    )>,
    mut despawn_event_writer: EventWriter<Despawn>,
) {
    for (entity, expireable, player, projectile, particle) in expireable_query.iter_mut() {
        if time.elapsed_seconds_f64() < expireable.expires_at {
            continue;
        }

        let entity_type;

        if player.is_some() {
            entity_type = "player"
        } else if projectile.is_some() {
            entity_type = "projectile"
        } else if particle.is_some() {
            entity_type = "particle"
        } else {
            panic!(
                "failed to identify entity_type for entity={:?}, expireable={:?}",
                entity, expireable
            );
        }

        despawn_event_writer.send(Despawn {
            entity_uuid: expireable.entity_uuid,
            entity_type: entity_type.to_string(),
        });
    }
}
