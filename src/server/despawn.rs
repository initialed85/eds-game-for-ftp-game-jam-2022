use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::Despawn;
use crate::types::network::{Container, OutgoingMessage};

pub fn handle_despawn_event(
    mut despawn_event_reader: EventReader<Despawn>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
) {
    for despawn_event in despawn_event_reader.iter() {
        let message = serialize(Container {
            message_type: "despawn".to_string(),
            join: None,
            spawn: None,
            update: None,
            despawn: Some(despawn_event.clone()),
            leave: None,
        });

        // tell everyone to despawn the entity
        outgoing_message_event_writer.send(OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: message.clone(),
        });
    }
}
