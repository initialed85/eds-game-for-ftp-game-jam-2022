use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::serialize;
use crate::types::event::{DespawnEvent, LeaveEvent};
use crate::types::network::{Container, OutgoingMessageEvent};

pub fn handle_leave_event(
    mut leave_event_reader: EventReader<LeaveEvent>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessageEvent>,
    mut despawn_event_writer: EventWriter<DespawnEvent>,
) {
    for leave_event in leave_event_reader.read() {
        despawn_event_writer.send(DespawnEvent {
            entity_uuid: leave_event.player_uuid,
            entity_type: "player".to_string(),
        });

        // tell everyone the player has left
        outgoing_message_event_writer.send(OutgoingMessageEvent {
            session_uuid: None,
            not_session_uuid: None,
            message: serialize(Container {
                message_type: "leave".to_string(),
                join: None,
                spawn: None,
                input: None,
                update: None,
                despawn: None,
                leave: Some(leave_event.clone()),
                collision: None,
            }),
        });
    }
}
