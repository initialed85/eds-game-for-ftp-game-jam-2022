use std::cell::RefCell;
use std::rc::Rc;

use bevy::prelude::{trace, EventReader, EventWriter, NonSend};

use crate::client::websocket::WebSocketClient;
use crate::types::network::{Close, IncomingMessage, Open, OutgoingMessage};

pub fn handle_websocket_client(
    web_socket: NonSend<Rc<RefCell<WebSocketClient>>>,
    mut outgoing_message_event_reader: EventReader<OutgoingMessage>,
    mut open_event_writer: EventWriter<Open>,
    mut incoming_message_event_writer: EventWriter<IncomingMessage>,
    mut close_event_writer: EventWriter<Close>,
) {
    let mut web_socket = web_socket.as_ref().borrow_mut();

    for outgoing_message_event in outgoing_message_event_reader.iter() {
        let session_uuid = outgoing_message_event.session_uuid.unwrap();
        trace!(
            "handle_websocket_client; incoming_message - session_uuid={:?}, message={:?}",
            session_uuid,
            outgoing_message_event.message
        );
        web_socket.send(session_uuid, outgoing_message_event.message.clone())
    }

    let websocket_open_events = web_socket.get_open_events();
    for session_uuid in websocket_open_events.iter() {
        trace!("handle_websocket_client; open - session_uuid={:?}", session_uuid);
        open_event_writer.send(Open {
            session_uuid: session_uuid.clone(),
        });
    }

    let websocket_incoming_message_events = web_socket.get_incoming_message_events();
    for (session_uuid, message) in websocket_incoming_message_events.iter() {
        trace!(
            "handle_websocket_client; incoming_message - session_uuid={:?}, message={:?}",
            session_uuid,
            message
        );
        incoming_message_event_writer.send(IncomingMessage {
            session_uuid: session_uuid.clone(),
            message: message.clone(),
        });
    }

    let websocket_close_events = web_socket.get_close_events();
    for session_uuid in websocket_close_events.iter() {
        trace!("handle_websocket_client; close - session_uuid={:?}", session_uuid);
        close_event_writer.send(Close {
            session_uuid: session_uuid.clone(),
        });
    }
}
