use bevy::prelude::{EventWriter, Query, Res, Time, Transform};
use bevy_rapier2d::dynamics::Velocity;

use crate::behaviour::moveable::Moveable;
use crate::types::event::{SerializableTransform, SerializableVelocity, Update};

pub fn handle_update_for_moveable(
    time: Res<Time>,
    mut moveable_query: Query<(&mut Moveable, &Transform, &Velocity)>,
    mut update_event_writer: EventWriter<Update>,
) {
    for (mut moveable, transform, velocity) in moveable_query.iter_mut() {
        if time.elapsed_seconds_f64() - moveable.last_update_handled_at < moveable.update_rate_seconds {
            continue;
        }

        let update = Update {
            entity_uuid: moveable.entity_uuid,
            server_time: time.elapsed_seconds_f64(),
            client_time: 0.0,
            transform: Some(SerializableTransform::from_transform(transform.clone())),
            velocity: Some(SerializableVelocity::from_velocity(velocity.clone())),
            handled_at: None,
            includes_rollover: moveable.had_rollover,
        };

        update_event_writer.send(update);

        moveable.last_update_handled_at = time.elapsed_seconds_f64();
        moveable.had_rollover = false;
    }
}
