use std::cell::RefCell;
use std::rc::Rc;

use bevy::prelude::{EventReader, EventWriter, NonSend};

use crate::types::PlayerMessage;
use crate::websocket_client::WebSocketClient;

pub fn handle_client_read(web_socket_client: NonSend<Rc<RefCell<WebSocketClient>>>, mut player_message_writer: EventWriter<PlayerMessage>) {
    let mut wsc = web_socket_client.as_ref().borrow_mut();

    for message in wsc.get_messages().iter() {
        let result = serde_json::from_str(message.as_str());
        if result.is_err() {
            continue;
        }

        let mut player_message: PlayerMessage = result.unwrap();

        player_message.is_incoming = true;

        // warn!(">>> {:?}", player_message);

        player_message_writer.send(player_message);
    }
}

pub fn handle_client_write(
    mut player_message_reader: EventReader<PlayerMessage>,
    web_socket_client: NonSend<Rc<RefCell<WebSocketClient>>>,
) {
    let wsc = web_socket_client.as_ref().borrow_mut();

    for player_message in player_message_reader.iter() {
        if player_message.is_incoming {
            continue;
        }

        let result = serde_json::to_string(&player_message);

        if result.is_err() {
            continue;
        }

        wsc.send(result.unwrap());

        // warn!("<<< {:?}", player_message);
    }
}
