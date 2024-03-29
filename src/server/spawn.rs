use bevy::log::warn;
use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::Spawn;
use crate::types::network::{Container, OutgoingMessage};

pub fn handle_spawn_event(
    mut spawn_event_reader: EventReader<Spawn>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
) {
    for spawn_event in spawn_event_reader.read() {
        let container = Container {
            message_type: "spawn".to_string(),
            join: None,
            spawn: Some(spawn_event.clone()),
            update: None,
            input: None,
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
        // tell everyone to spawn the entity
        outgoing_message_event_writer.send(OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: serialized_container.unwrap(),
        });
    }
}
