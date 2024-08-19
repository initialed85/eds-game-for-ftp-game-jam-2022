use bevy::prelude::{EventReader, EventWriter};

use crate::base::helpers::deserialize;
use crate::behaviour::collideable::CollisionEvent;
use crate::types::event::{
    DespawnEvent, InputEvent, JoinEvent, LeaveEvent, SpawnEvent, UpdateEvent,
};
use crate::types::network::{CloseEvent, Container, IncomingMessageEvent, OpenEvent};

pub fn base_handle_open_event(mut open_event_reader: EventReader<OpenEvent>) {
    for open_event in open_event_reader.read() {
        let _ = open_event;
    }
}

pub fn base_handle_incoming_message_event(
    mut incoming_message_event_reader: EventReader<IncomingMessageEvent>,
    mut join_event_writer: EventWriter<JoinEvent>,
    mut spawn_event_writer: EventWriter<SpawnEvent>,
    mut input_event_writer: EventWriter<InputEvent>,
    mut update_event_writer: EventWriter<UpdateEvent>,
    mut leave_event_writer: EventWriter<LeaveEvent>,
    mut despawn_event_writer: EventWriter<DespawnEvent>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    for incoming_message_event in incoming_message_event_reader.read() {
        let container = deserialize::<Container>(incoming_message_event.message.clone());

        if container.message_type == "join" {
            join_event_writer.send(container.join.unwrap());
        } else if container.message_type == "spawn" {
            spawn_event_writer.send(container.spawn.unwrap());
        } else if container.message_type == "input" {
            input_event_writer.send(container.input.unwrap());
        } else if container.message_type == "update" {
            update_event_writer.send(container.update.unwrap());
        } else if container.message_type == "leave" {
            leave_event_writer.send(container.leave.unwrap());
        } else if container.message_type == "despawn" {
            despawn_event_writer.send(container.despawn.unwrap());
        } else if container.message_type == "collision" {
            collision_event_writer.send(container.collision.unwrap());
        }
    }
}

pub fn base_handle_close_event(mut close_event_reader: EventReader<CloseEvent>) {
    for close_event in close_event_reader.read() {
        let _ = close_event;
    }
}
