use bevy::prelude::{EventReader, EventWriter, Query, Transform, Vec3};
use bevy_rapier2d::prelude::Velocity;

use crate::behaviour::weaponized::{Fire, Weaponized};
use crate::constants::{
    PLAYER_ANGULAR_VELOCITY_MAX, PLAYER_ANGULAR_VELOCITY_STEP, PLAYER_LINEAR_VELOCITY_MAX,
};
use crate::identity::player::Player;
use crate::types::event::Input;

pub fn handle_input_event(mut input_event_reader: EventReader<Input>, mut player_query: Query<&mut Player>) {
    for input in input_event_reader.iter() {
        for mut player in player_query.iter_mut() {
            if player.player_uuid != input.player_uuid {
                continue;
            }

            player.unhandled_inputs.insert(0, input.clone());
        }
    }
}

pub fn handle_input_for_player(
    mut player_query: Query<(&mut Player, &Transform, &mut Velocity, &Weaponized)>,
    mut fire_event_writer: EventWriter<Fire>,
) {
    for (mut player, transform, mut velocity, weaponized) in player_query.iter_mut() {
        let mut inputs = player.unhandled_inputs.clone();
        player.unhandled_inputs.clear();

        if inputs.len() == 0 {
            if player.last_input.is_some() {
                inputs.insert(0, player.clone().last_input.unwrap());
            }
        } else {
            player.last_input = Some(inputs.last().unwrap().clone());
        }

        for last_input in inputs.iter() {
            if last_input.is_left {
                if velocity.angvel <= PLAYER_ANGULAR_VELOCITY_MAX {
                    velocity.angvel += PLAYER_ANGULAR_VELOCITY_STEP;
                }
            }

            if last_input.is_right {
                if velocity.angvel >= -PLAYER_ANGULAR_VELOCITY_MAX {
                    velocity.angvel -= PLAYER_ANGULAR_VELOCITY_STEP;
                }
            }

            if last_input.is_forward {
                velocity.linvel += transform
                    .rotation
                    .mul_vec3(Vec3::new(0.0, PLAYER_LINEAR_VELOCITY_MAX, 0.0))
                    .truncate();
            }

            if last_input.is_backward {
                velocity.linvel += transform
                    .rotation
                    .mul_vec3(Vec3::new(0.0, -PLAYER_LINEAR_VELOCITY_MAX, 0.0))
                    .truncate();
            }

            if last_input.is_firing {
                weaponized.fire(&mut fire_event_writer);
            }
        }
    }
}
