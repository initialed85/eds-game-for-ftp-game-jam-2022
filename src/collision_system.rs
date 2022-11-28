// TODO WIP
// use bevy::hierarchy::DespawnRecursiveExt;
// use bevy::prelude::{Commands, Entity, EventReader, Query};
use bevy::prelude::{Entity, EventReader, Query};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::player::Player;
use crate::projectile::Projectile;
// TODO WIP
// use crate::weapon::Weapon;

pub fn handle_collision(
    mut projectile_query: Query<(Entity, &Projectile)>,
    mut player_query: Query<(Entity, &Player)>,
    mut collision_events: EventReader<CollisionEvent>,
    // TODO WIP

    // mut weapon_query: Query<(Entity, &Weapon)>,
    // mut commands: Commands,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                let entity_1_projectile = projectile_query.get_mut(entity_1.clone());
                if entity_1_projectile.is_ok() {
                    println!("entity_1={:?}", entity_1_projectile.unwrap());
                    // TODO WIP
                    // commands.entity(*entity_1).despawn_recursive();
                }

                let entity_1_player = player_query.get_mut(entity_1.clone());
                if entity_1_player.is_ok() {
                    println!("entity_1={:?}", entity_1_player.unwrap());
                }

                let entity_2_projectile = projectile_query.get_mut(entity_2.clone());
                if entity_2_projectile.is_ok() {
                    println!("entity_2={:?}", entity_2_projectile.unwrap());
                    // commands.entity(*entity_2).despawn_recursive();
                }

                let entity_2_player = player_query.get_mut(entity_2.clone());
                if entity_2_player.is_ok() {
                    // TODO WIP
                    println!("entity_2={:?}", entity_2_player.unwrap());
                }
            }
            CollisionEvent::Stopped(_entity_1, _entity_2, _flags) => {
                // noop
            }
        }
    }
}
