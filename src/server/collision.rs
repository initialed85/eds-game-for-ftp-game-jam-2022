use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::behaviour::collideable::Collision;
use crate::types::network::{Container, OutgoingMessage};

pub fn handle_collision_event(
    mut collision_event_reader: EventReader<Collision>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
) {
    for collision_event in collision_event_reader.iter() {
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
        outgoing_message_event_writer.send(OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: message.clone(),
        });
    }
}
