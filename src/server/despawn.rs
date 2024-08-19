use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::DespawnEvent;
use crate::types::network::{Container, OutgoingMessageEvent};

pub fn handle_despawn_event(
    mut despawn_event_reader: EventReader<DespawnEvent>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessageEvent>,
) {
    for despawn_event in despawn_event_reader.read() {
        if despawn_event.entity_type == "particle" {
            continue;
        }

        let message = serialize(Container {
            message_type: "despawn".to_string(),
            join: None,
            spawn: None,
            input: None,
            update: None,
            despawn: Some(despawn_event.clone()),
            leave: None,
            collision: None,
        });

        // tell everyone to despawn the entity
        outgoing_message_event_writer.send(OutgoingMessageEvent {
            session_uuid: None,
            not_session_uuid: None,
            message: message.clone(),
        });
    }
}
