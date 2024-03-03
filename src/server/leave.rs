use bevy::log::warn;
use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::{Despawn, Leave};
use crate::types::network::{Container, OutgoingMessage};

pub fn handle_leave_event(
    mut leave_event_reader: EventReader<Leave>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
    mut despawn_event_writer: EventWriter<Despawn>,
) {
    for leave_event in leave_event_reader.read() {
        despawn_event_writer.send(Despawn {
            entity_uuid: leave_event.player_uuid,
            entity_type: "player".to_string(),
        });

        let container = Container {
            message_type: "leave".to_string(),
            join: None,
            spawn: None,
            input: None,
            update: None,
            despawn: None,
            leave: Some(leave_event.clone()),
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
        // tell everyone the player has left
        outgoing_message_event_writer.send(OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: serialized_container.unwrap(),
        });
    }
}
