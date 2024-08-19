use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::behaviour::collideable::CollisionEvent;
use crate::types::network::{Container, OutgoingMessageEvent};

pub fn handle_collision_event(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessageEvent>,
) {
    for collision_event in collision_event_reader.read() {
        let message = serialize(Container {
            message_type: "collision".to_string(),
            join: None,
            spawn: None,
            update: None,
            input: None,
            despawn: None,
            leave: None,
            collision: Some(collision_event.clone()),
        });

        // tell everyone to about the collision
        outgoing_message_event_writer.send(OutgoingMessageEvent {
            session_uuid: None,
            not_session_uuid: None,
            message: message.clone(),
        });
    }
}
