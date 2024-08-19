use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::SpawnEvent;
use crate::types::network::{Container, OutgoingMessageEvent};

pub fn handle_spawn_event(
    mut spawn_event_reader: EventReader<SpawnEvent>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessageEvent>,
) {
    for spawn_event in spawn_event_reader.read() {
        let message = serialize(Container {
            message_type: "spawn".to_string(),
            join: None,
            spawn: Some(spawn_event.clone()),
            update: None,
            input: None,
            despawn: None,
            leave: None,
            collision: None,
        });

        // tell everyone to spawn the entity
        outgoing_message_event_writer.send(OutgoingMessageEvent {
            session_uuid: None,
            not_session_uuid: None,
            message: message.clone(),
        });
    }
}
