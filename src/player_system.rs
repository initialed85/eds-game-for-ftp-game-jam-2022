use bevy::prelude::{Input, KeyCode, Query, Res, Transform, Vec3};
use bevy_rapier2d::prelude::Velocity;

use crate::constants::{PLAYER_ANGULAR_VELOCITY_MAX, PLAYER_ANGULAR_VELOCITY_STEP, PLAYER_LINEAR_VELOCITY};
use crate::player::Player;

pub fn handle_player(mut query: Query<(&Player, &Transform, &mut Velocity)>, keyboard_input: Res<Input<KeyCode>>) {
    for (player, transform, mut velocity) in query.iter_mut() {
        let is_left = keyboard_input.pressed(player.left_key);
        let is_right = keyboard_input.pressed(player.right_key);
        let is_forward = keyboard_input.pressed(player.forward_key);
        let is_backward = keyboard_input.pressed(player.backward_key);

        let mut linear_velocity = Vec3::ZERO;
        let mut angular_velocity = velocity.angvel.clone();

        if is_left {
            if angular_velocity <= PLAYER_ANGULAR_VELOCITY_MAX {
                angular_velocity += PLAYER_ANGULAR_VELOCITY_STEP;
            }
        }

        if is_right {
            if angular_velocity >= -PLAYER_ANGULAR_VELOCITY_MAX {
                angular_velocity -= PLAYER_ANGULAR_VELOCITY_STEP;
            }
        }

        if is_forward {
            linear_velocity.y = PLAYER_LINEAR_VELOCITY;
        }

        if is_backward {
            linear_velocity.y = -PLAYER_LINEAR_VELOCITY;
        }

        velocity.linvel += transform.rotation.mul_vec3(linear_velocity).truncate();
        velocity.angvel = angular_velocity;
    }
}
