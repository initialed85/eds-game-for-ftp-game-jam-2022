use bevy::prelude::{Query, Res, Time, Transform};
use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::dynamics::Velocity;

use crate::behaviour::moveable::Moveable;
use crate::identity::game::Game;

pub fn handle_update_for_moveable(
    time: Res<Time>,
    mut moveable_query: Query<(&mut Moveable, &mut Transform, &mut Velocity)>,
    game: Res<Game>,
) {
    let original_delta = game.server_time_at_join - game.client_time_at_join;
    let synced_time = time.elapsed_seconds_f64() + original_delta;

    for (mut moveable, mut transform, mut velocity) in moveable_query.iter_mut() {
        loop {
            // nothing to handle
            let update = moveable.unhandled_updates.last();
            if update.is_none() {
                break;
            }

            // nothing to handle yet
            let update = update.unwrap();
            if update.server_time > synced_time {
                break;
            }

            // handle / ignore this update (pending if there are newer)
            moveable.update_to_handle = Some(moveable.unhandled_updates.pop().unwrap());
        }

        if moveable.update_to_handle.is_none() {
            continue;
        }

        let mut update = moveable.update_to_handle.clone().unwrap();

        // ref.: gafferongames.com
        /* The trick to making this all work is that when a state update comes in you take the current
        simulation position and add the position error to that, and subtract that from the new position,
        giving the new position error offset which gives an identical result to the current (smoothed)
        visual position. */

        // translation and rotation are only handled on receipt of an update
        if update.transform.is_some() && update.handled_at.is_none() {
            let update_transform = update.clone().transform.unwrap();

            if game.local_player_uuid.is_some() && game.local_player_uuid.unwrap() == update.entity_uuid {
                let latency = update.server_time - synced_time;
                screen_print!("latency={:?}, synced_time={:?}", latency, synced_time);
            }

            if !update.includes_rollover {
                // common path uses EMA for translation smoothing
                let old_translation_error = moveable.translation_error.get_value();
                let new_translation = transform.translation.clone() + old_translation_error;
                transform.translation = new_translation;
                let new_translation_error = update_transform.translation - new_translation;
                moveable
                    .translation_error
                    .add_value(time.clone(), new_translation_error);

                // and also for rotation smoothing
                let old_rotation_error = moveable.rotation_error.get_value();
                let new_rotation = transform.rotation * old_rotation_error;
                transform.rotation = new_rotation;
                let new_rotation_error = update_transform.rotation * new_rotation.inverse();
                moveable
                    .rotation_error
                    .add_value(time.clone(), new_rotation_error);

                if game.local_player_uuid.is_some() && game.local_player_uuid.unwrap() == update.entity_uuid {
                    screen_print!("translation_error={:?}", new_translation_error);
                    screen_print!("translation={:?}", transform.translation);
                    screen_print!("rotation_error={:?}", new_rotation_error);
                    screen_print!("rotation={:?}", transform.rotation);
                }
            } else {
                // rollover path upsets EMA, so just reset the values
                transform.translation = update_transform.translation;
                transform.rotation = update_transform.rotation;

                // and also the EMAs
                moveable.translation_error.reset();
                moveable.rotation_error.reset();
            }
        }

        // linear and angular velocity is handled every frame to keep it smooth
        if update.velocity.is_some() {
            let update_velocity = update.clone().velocity.unwrap();

            let old_linvel_error = moveable.linvel_error.get_value();
            let new_linvel = velocity.linvel.clone() + old_linvel_error;
            velocity.linvel = new_linvel;
            let new_linvel_error = update_velocity.linvel - new_linvel;
            moveable.linvel_error.add_value(time.clone(), new_linvel_error);

            let old_angvel_error = moveable.angvel_error.get_value();
            let new_angvel = velocity.angvel.clone() as f64 + old_angvel_error;
            velocity.angvel = new_angvel as f32;
            let new_angvel_error = update_velocity.angvel - new_angvel as f32;
            moveable
                .angvel_error
                .add_value(time.clone(), new_angvel_error as f64);
            if game.local_player_uuid.is_some() && game.local_player_uuid.unwrap() == update.entity_uuid {
                screen_print!("linvel={:?}", velocity.linvel);
                screen_print!("linvel_error={:?}", new_linvel_error);
                screen_print!("angvel={:?}", velocity.angvel);
                screen_print!("angvel_error={:?}", new_angvel_error);
            }
        }

        update.handled_at = Some(time.elapsed_seconds_f64());
        moveable.update_to_handle = Some(update.clone());
    }
}
