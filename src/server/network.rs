use std::cell::RefCell;
use std::rc::Rc;

use bevy::log::trace;
use bevy::prelude::{EventReader, EventWriter, NonSend, Query};

use crate::identity::player::Player;
use crate::server::websocket::WebSocketServer;
use crate::types::event::{Join, Leave};
use crate::types::network::{Close, IncomingMessage, Open, OutgoingMessage};

pub fn handle_websocket_server(
    web_socket: NonSend<Rc<RefCell<WebSocketServer>>>,
    mut outgoing_message_event_reader: EventReader<OutgoingMessage>,
    mut open_event_writer: EventWriter<Open>,
    mut incoming_message_event_writer: EventWriter<IncomingMessage>,
    mut close_event_writer: EventWriter<Close>,
) {
    let mut web_socket = web_socket.borrow_mut();
    web_socket.handle();

    for outgoing_message_event in outgoing_message_event_reader.iter() {
        if outgoing_message_event.session_uuid.is_some() {
            let session_uuid = outgoing_message_event.session_uuid.unwrap();
            // trace!(
            //     "handle_websocket_server; outgoing_message - session_uuid={:?}, message={:?}",
            //     session_uuid,
            //     outgoing_message_event.message
            // );
            web_socket.send(session_uuid.clone(), outgoing_message_event.message.clone())
        } else if outgoing_message_event.session_uuid.is_none() {
            if outgoing_message_event.not_session_uuid.is_none() {
                // trace!(
                //     "handle_websocket_server; outgoing_message - session_uuid=(broadcast), message={:?}",
                //     outgoing_message_event.message
                // );
                web_socket.broadcast(outgoing_message_event.message.clone());
            } else {
                let not_session_uuid = outgoing_message_event.not_session_uuid.unwrap();

                // trace!(
                //     "handle_websocket_server; outgoing_message - not_session_uuid={:?}, message={:?}",
                //     not_session_uuid,
                //     outgoing_message_event.message
                // );

                let session_uuids = web_socket.get_session_uuids();

                for session_uuid in session_uuids.iter() {
                    let session_uuid = session_uuid.clone();

                    if session_uuid == not_session_uuid {
                        continue;
                    }

                    web_socket.send(session_uuid, outgoing_message_event.message.clone());
                }
            }
        }
    }

    let websocket_open_events = web_socket.get_open_events();
    for session_uuid in websocket_open_events.iter() {
        trace!("handle_websocket_server; open - session_uuid={:?}", session_uuid);
        open_event_writer.send(Open {
            session_uuid: session_uuid.clone(),
        });
    }

    let websocket_incoming_message_events = web_socket.get_incoming_message_events();
    for (session_uuid, message) in websocket_incoming_message_events.iter() {
        // trace!(
        //     "handle_websocket_server; incoming_message - session_uuid={:?}, message={:?}",
        //     session_uuid,
        //     message
        // );
        incoming_message_event_writer.send(IncomingMessage {
            session_uuid: session_uuid.clone(),
            message: message.clone(),
        });
    }

    let websocket_close_events = web_socket.get_close_events();
    for session_uuid in websocket_close_events.iter() {
        trace!("handle_websocket_server; close - session_uuid={:?}", session_uuid);
        close_event_writer.send(Close {
            session_uuid: session_uuid.clone(),
        });
    }
}

pub fn handle_open_event(
    mut open_event_reader: EventReader<Open>,
    player_query: Query<&Player>,
    mut join_event_writer: EventWriter<Join>,
) {
    for open_event in open_event_reader.iter() {
        let mut other_player_uuids = vec![];

        for other_player in player_query.iter() {
            other_player_uuids.push(other_player.player_uuid);
        }

        join_event_writer.send(Join {
            player_uuid: open_event.session_uuid,
            is_for_local_player: true,
        });
    }
}

pub fn handle_close_event(
    mut close_event_reader: EventReader<Close>,
    mut leave_event_writer: EventWriter<Leave>,
) {
    for close_event in close_event_reader.iter() {
        leave_event_writer.send(Leave {
            player_uuid: close_event.session_uuid,
        });
    }
}
