use bevy::prelude::{EventReader, EventWriter, Mesh, Res, ResMut, Time};

use crate::base::helpers::serialize;
use crate::types::event::Spawn;
use crate::types::network::{Container, OutgoingMessage};

pub fn handle_spawn_event(
    mut spawn_event_reader: EventReader<Spawn>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
) {
    for spawn_event in spawn_event_reader.iter() {
        let message = serialize(Container {
            message_type: "spawn".to_string(),
            join: None,
            spawn: Some(spawn_event.clone()),
            update: None,
            despawn: None,
            leave: None,
        });

        // tell everyone to spawn the entity
        outgoing_message_event_writer.send(OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: message.clone(),
        });
    }
}
