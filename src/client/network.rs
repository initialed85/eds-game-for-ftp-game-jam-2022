use std::cell::RefCell;
use std::rc::Rc;

use bevy::log::trace;
use bevy::prelude::{EventReader, EventWriter, NonSend};
use uuid::Uuid;

use crate::client::websocket::WebSocketClient;
use crate::types::network::{CloseEvent, IncomingMessageEvent, OpenEvent, OutgoingMessageEvent};

pub fn handle_websocket_client(
    web_socket: NonSend<Rc<RefCell<WebSocketClient>>>,
    mut outgoing_message_event_reader: EventReader<OutgoingMessageEvent>,
    mut open_event_writer: EventWriter<OpenEvent>,
    mut incoming_message_event_writer: EventWriter<IncomingMessageEvent>,
    mut close_event_writer: EventWriter<CloseEvent>,
) {
    let mut web_socket = web_socket.as_ref().borrow_mut();

    let session_uuid = Uuid::default();

    for outgoing_message_event in outgoing_message_event_reader.read() {
        // trace!(
        //     "handle_websocket_client; outgoing_message - session_uuid={:?}, message={:?}",
        //     session_uuid,
        //     outgoing_message_event.message
        // );
        web_socket.send(session_uuid, outgoing_message_event.message.clone());
    }

    let websocket_open_events = web_socket.get_open_events();
    for session_uuid in websocket_open_events.iter() {
        trace!(
            "handle_websocket_client; open - session_uuid={:?}",
            session_uuid
        );
        open_event_writer.send(OpenEvent {
            session_uuid: *session_uuid,
        });
    }

    let websocket_incoming_message_events = web_socket.get_incoming_message_events();
    for (session_uuid, message) in websocket_incoming_message_events.iter() {
        // trace!(
        //     "handle_websocket_client; incoming_message - session_uuid={:?}, message={:?}",
        //     session_uuid,
        //     message
        // );
        incoming_message_event_writer.send(IncomingMessageEvent {
            session_uuid: *session_uuid,
            message: message.clone(),
        });
    }

    let websocket_close_events = web_socket.get_close_events();
    for session_uuid in websocket_close_events.iter() {
        trace!(
            "handle_websocket_client; close - session_uuid={:?}",
            session_uuid
        );
        close_event_writer.send(CloseEvent {
            session_uuid: *session_uuid,
        });
    }
}
