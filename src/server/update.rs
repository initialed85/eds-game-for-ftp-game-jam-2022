use bevy::log::warn;
use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::Update;
use crate::types::network::{Container, OutgoingMessage};

pub fn handle_update_event(
    mut update_event_reader: EventReader<Update>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
) {
    for update in update_event_reader.read() {
        let container = Container {
            message_type: "update".to_string(),
            join: None,
            spawn: None,
            input: None,
            update: Some(update.clone()),
            despawn: None,
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

        outgoing_message_event_writer.send(OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: serialized_container.unwrap(),
        })
    }
}
