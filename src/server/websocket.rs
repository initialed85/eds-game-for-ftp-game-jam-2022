use std::cell::RefCell;
use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;

use bevy::utils::Uuid;
use tungstenite::protocol::frame::coding::CloseCode::Normal;
use tungstenite::protocol::CloseFrame;
use tungstenite::{accept, Message, WebSocket};

use crate::base::helpers::serialize;
use crate::constants::{LISTEN_HOST, LISTEN_PORT};

#[derive(Debug)]
pub struct WebSocketServer {
    tcp_listener: TcpListener,
    web_socket_by_session_uuid: HashMap<Uuid, WebSocket<TcpStream>>,
    open_events: Vec<Uuid>,
    incoming_message_events: Vec<(Uuid, Vec<u8>)>,
    outgoing_message_events: Vec<(Uuid, Message)>,
    close_events: Vec<Uuid>,
}

impl WebSocketServer {
    pub fn new(server_fqdn: String, port: i32) -> WebSocketServer {
        let tcp_listener = TcpListener::bind(format!("{:}:{:}", server_fqdn, port)).unwrap();
        tcp_listener.set_nonblocking(true).unwrap();

        let web_socket_server = WebSocketServer {
            tcp_listener,
            web_socket_by_session_uuid: HashMap::new(),
            open_events: vec![],
            incoming_message_events: vec![],
            outgoing_message_events: vec![],
            close_events: vec![],
        };

        return web_socket_server;
    }

    fn handle_tcp_listener(self: &mut WebSocketServer) {
        for tcp_stream in self.tcp_listener.incoming() {
            if tcp_stream.is_err() {
                let err = tcp_stream.err().unwrap();
                if err.kind() == WouldBlock {
                    return;
                }
                println!("tcp_stream.is_err; err={:?}", err);
                return;
            }

            let tcp_stream = tcp_stream;
            if tcp_stream.is_err() {
                let err = tcp_stream.err().unwrap();
                if err.kind() == WouldBlock {
                    return;
                }
                println!("tcp_stream.is_err; err={:?}", err);
                return;
            }

            let tcp_stream = tcp_stream.unwrap();
            tcp_stream.set_nonblocking(true).unwrap();

            let web_socket = accept(tcp_stream);
            if web_socket.is_err() {
                println!("web_socket.is_err; err={:?}", web_socket.err().unwrap());
                return;
            }

            let session_uuid = Uuid::new_v4();
            let web_socket = web_socket.unwrap();
            self.web_socket_by_session_uuid
                .insert(session_uuid.clone(), web_socket);

            self.open_events.push(session_uuid);
            println!("open_event; session_uuid={:?}", session_uuid);
        }
    }

    fn handle_close_event(self: &mut WebSocketServer, session_uuid: Uuid) {
        let web_socket = self.web_socket_by_session_uuid.get_mut(&session_uuid);
        if web_socket.is_none() {
            return;
        }

        let web_socket = web_socket.unwrap();

        web_socket
            .close(Some(CloseFrame {
                code: Normal,
                reason: Default::default(),
            }))
            .unwrap_or_default();

        self.web_socket_by_session_uuid.remove(&session_uuid).unwrap();

        self.close_events.push(session_uuid);
        println!("close_event; session_uuid={:?}", session_uuid);
    }

    fn handle_incoming_message_event(self: &mut WebSocketServer, session_uuid: Uuid) {
        let web_socket = self.web_socket_by_session_uuid.get_mut(&session_uuid);
        if web_socket.is_none() {
            return;
        }

        let web_socket = web_socket.unwrap();

        loop {
            let message = web_socket.read_message();
            if message.is_err() {
                // println!("tcp_stream.is_err; err={:?}", err);
                return;
            }

            let message = message.unwrap();
            if message.is_close() {
                self.handle_close_event(session_uuid);
                // println!("message.is_close; message={:?}", message);
                return;
            }

            if message.is_empty() || message.is_ping() || message.is_pong() {
                println!("message.is_empty / is_ping / is_pong; message={:?}", message);
                continue;
            }

            let message = message.into_data();

            self.incoming_message_events.push((session_uuid, message.clone()));

            // println!("message; message={:?}", message);
        }
    }

    fn handle_incoming_message_events(self: &mut WebSocketServer) {
        let mut session_uuids = vec![];

        for (session_uuid, _) in self.web_socket_by_session_uuid.iter_mut() {
            session_uuids.push(session_uuid.clone());
        }

        for session_uuid in session_uuids.into_iter() {
            self.handle_incoming_message_event(session_uuid.clone());
        }
    }

    fn handle_outgoing_message_event(self: &mut WebSocketServer, session_uuid: &Uuid, message: &Message) {
        let web_socket = self.web_socket_by_session_uuid.get_mut(&session_uuid);
        if web_socket.is_none() {
            return;
        }

        let web_socket = web_socket.unwrap();

        web_socket.write_message(message.clone()).unwrap_or_default();

        // println!(
        //     "<<< handle_outgoing_message_event; session_uuid={:?}, message={:?}",
        //     session_uuid,
        //     message.to_string()
        // );
    }

    fn handle_outgoing_message_events(self: &mut WebSocketServer) {
        let outgoing_message_events = self.outgoing_message_events.to_vec();
        self.outgoing_message_events.clear();

        let mut raw_messages_by_session_uuid: HashMap<Uuid, Vec<Vec<u8>>> = HashMap::new();

        // TODO: fix going from Message back to Vec<u8>
        for (session_uuid, message) in outgoing_message_events.iter() {
            let session_uuid = session_uuid.clone();

            if !raw_messages_by_session_uuid.contains_key(&session_uuid) {
                raw_messages_by_session_uuid.insert(session_uuid, vec![]);
            }

            raw_messages_by_session_uuid
                .get_mut(&session_uuid)
                .unwrap()
                .push(message.clone().into_data());
        }

        // TODO: fix double serialization
        for (session_uuid, raw_messages) in raw_messages_by_session_uuid.iter() {
            let batched_raw_messages = serialize(raw_messages);
            let message = Message::from(batched_raw_messages);
            self.handle_outgoing_message_event(session_uuid, &message);
        }
    }

    fn handle_web_sockets(self: &mut WebSocketServer) {
        self.handle_incoming_message_events();
        self.handle_outgoing_message_events();
    }

    pub fn handle(self: &mut WebSocketServer) {
        self.handle_tcp_listener();
        self.handle_web_sockets();
    }

    pub fn get_session_uuids(self: &mut WebSocketServer) -> Vec<Uuid> {
        let mut session_uuids = vec![];

        for (session_uuid, _) in self.web_socket_by_session_uuid.iter_mut() {
            session_uuids.push(session_uuid.clone());
        }

        return session_uuids;
    }

    pub fn get_open_events(self: &mut WebSocketServer) -> Vec<Uuid> {
        let open_events = self.open_events.to_vec();
        self.open_events.clear();
        return open_events;
    }

    pub fn get_incoming_message_events(self: &mut WebSocketServer) -> Vec<(Uuid, Vec<u8>)> {
        let incoming_message_events = self.incoming_message_events.to_vec();
        self.incoming_message_events.clear();
        return incoming_message_events;
    }

    pub fn get_close_events(self: &mut WebSocketServer) -> Vec<Uuid> {
        let close_events = self.close_events.to_vec();
        self.close_events.clear();
        return close_events;
    }

    pub fn send(self: &mut WebSocketServer, session_uuid: Uuid, message: Vec<u8>) {
        // println!(">>> send; session_uuid={:?}, message={:?}", session_uuid, message);

        let message = Message::from(message);

        self.outgoing_message_events
            .push((session_uuid.clone(), message.clone()));

        // println!("<<< send; session_uuid={:?}, message={:?}", session_uuid, message);
    }

    pub fn broadcast(self: &mut WebSocketServer, message: Vec<u8>) {
        // println!(">>> broadcast; message={:?}", message);

        let message = Message::from(message);

        let mut session_uuids = vec![];

        for session_uuid in self.web_socket_by_session_uuid.keys() {
            session_uuids.push(session_uuid.clone());
        }

        for session_uuid in session_uuids.iter() {
            self.outgoing_message_events
                .push((session_uuid.clone(), message.clone()));
        }

        // println!("<<< broadcast; message={:?}", message);
    }
}

pub fn get_websocket_server() -> Rc<RefCell<WebSocketServer>> {
    return Rc::new(RefCell::new(WebSocketServer::new(
        LISTEN_HOST.to_string(),
        LISTEN_PORT,
    )));
}
