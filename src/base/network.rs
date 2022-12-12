use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::deserialize;
use crate::types::event::{Join, Spawn};
use crate::types::network::{Close, Container, IncomingMessage, Open};

pub fn base_handle_open_event(mut open_event_reader: EventReader<Open>) {
    for open_event in open_event_reader.iter() {
        let _ = open_event;
    }
}

pub fn base_handle_incoming_message_event(
    mut incoming_message_event_reader: EventReader<IncomingMessage>,
    mut join_event_writer: EventWriter<Join>,
    mut spawn_event_writer: EventWriter<Spawn>,
) {
    for incoming_message_event in incoming_message_event_reader.iter() {
        let container = deserialize::<Container>(incoming_message_event.message.clone());

        if container.message_type == "join" {
            join_event_writer.send(container.join.unwrap());
        } else if container.message_type == "spawn" {
            spawn_event_writer.send(container.spawn.unwrap());
        }
    }
}

pub fn base_handle_close_event(mut close_event_reader: EventReader<Close>) {
    for close_event in close_event_reader.iter() {
        let _ = close_event;
    }
}
