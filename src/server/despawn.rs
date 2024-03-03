use bevy::log::warn;
use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::Despawn;
use crate::types::network::{Container, OutgoingMessage};

pub fn handle_despawn_event(
    mut despawn_event_reader: EventReader<Despawn>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
) {
    for despawn_event in despawn_event_reader.read() {
        if despawn_event.entity_type == "particle" {
            continue;
        }

        let container = Container {
            message_type: "despawn".to_string(),
            join: None,
            spawn: None,
            input: None,
            update: None,
            despawn: Some(despawn_event.clone()),
            leave: None,
            collision: None,
        };

        let serialized_container = serialize(&container);
        if serialized_container.is_err() {
            warn!(
                "failed to serialize {:?} {:?}",
                container,
                serialized_container.err()
            );
            continue;
        }
        // tell everyone to despawn the entity
        outgoing_message_event_writer.send(OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: serialized_container.unwrap(),
        });
    }
}
