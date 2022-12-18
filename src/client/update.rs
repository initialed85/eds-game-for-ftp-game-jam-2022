use bevy::log::trace;
use bevy::prelude::{EventReader, Query, Res, Time, Transform};
use bevy_rapier2d::prelude::Velocity;

use crate::behaviour::moveable::Moveable;
use crate::types::event::Update;

pub fn handle_update_event(
    mut update_event_reader: EventReader<Update>,
    mut moveable_query: Query<&mut Moveable>,
) {
    for update in update_event_reader.iter() {
        for mut moveable in moveable_query.iter_mut() {
            if update.entity_uuid != moveable.entity_uuid {
                continue;
            }

            moveable.last_update = Some(update.clone());
        }
    }
}

pub fn handle_update_for_moveable(
    time: Res<Time>,
    mut moveable_query: Query<(&mut Moveable, &mut Transform, &mut Velocity)>,
) {
    for (mut moveable, mut transform, mut velocity) in moveable_query.iter_mut() {
        if moveable.last_update.is_none() {
            continue;
        }

        let mut last_update = moveable.last_update.clone().unwrap();

        // ref.: gafferongames.com
        /* The trick to making this all work is that when a state update comes in you take the current
        simulation position and add the position error to that, and subtract that from the new position,
        giving the new position error offset which gives an identical result to the current (smoothed)
        visual position. */

        // translation and rotation are only handled on receipt of an update
        if last_update.transform.is_some() && last_update.handled_at.is_none() {
            let update_transform = last_update.clone().transform.unwrap();

            if !last_update.includes_rollover {
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
        if last_update.velocity.is_some() {
            let update_velocity = last_update.clone().velocity.unwrap();

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
        }

        last_update.handled_at = Some(time.elapsed_seconds_f64());
        moveable.last_update = Some(last_update.clone());
    }
}
