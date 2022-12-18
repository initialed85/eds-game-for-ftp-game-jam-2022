use std::cell::RefCell;
use std::rc::Rc;

use bevy::reflect::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, BinaryType, ErrorEvent, Location, MessageEvent, WebSocket};

use crate::base::helpers::deserialize;

#[derive(Debug)]
pub struct WebSocketClient {
    _ws: WebSocket,
    open_events: Rc<RefCell<Vec<Uuid>>>,
    incoming_message_events: Rc<RefCell<Vec<(Uuid, Vec<u8>)>>>,
    close_events: Rc<RefCell<Vec<Uuid>>>,
}

impl WebSocketClient {
    pub fn new(server_uri: &str) -> WebSocketClient {
        let ws = WebSocket::new(server_uri).unwrap();

        ws.set_binary_type(BinaryType::Arraybuffer);

        let open_events = Rc::new(RefCell::new(vec![]));
        let incoming_message_events = Rc::new(RefCell::new(vec![]));
        let close_events = Rc::new(RefCell::new(vec![]));

        let web_socket_client = WebSocketClient {
            _ws: ws.clone(),
            open_events: Rc::clone(&open_events),
            incoming_message_events: Rc::clone(&incoming_message_events),
            close_events: Rc::clone(&close_events),
        };

        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            let mut open_events = open_events.as_ref().borrow_mut();

            open_events.push(Uuid::default());
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |m: MessageEvent| {
            if let Ok(buf) = m.data().dyn_into::<js_sys::ArrayBuffer>() {
                let data = js_sys::Uint8Array::new(&buf).to_vec();
                let mut incoming_message_events = incoming_message_events.as_ref().borrow_mut();
                incoming_message_events.push((Uuid::default(), data));
            }
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |_e: ErrorEvent| {
            let mut close_events = close_events.as_ref().borrow_mut();
            close_events.push(Uuid::default());
        });

        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        return web_socket_client;
    }

    pub fn send(self: &WebSocketClient, session_uuid: Uuid, data: Vec<u8>) {
        let _ = session_uuid;
        self._ws.send_with_u8_array(&data).unwrap();
    }

    pub fn get_open_events(self: &mut WebSocketClient) -> Vec<Uuid> {
        let open_events = self.open_events.as_ref().borrow_mut().to_vec();
        self.open_events.as_ref().borrow_mut().clear();
        return open_events;
    }

    pub fn get_incoming_message_events(self: &mut WebSocketClient) -> Vec<(Uuid, Vec<u8>)> {
        let incoming_message_events = self.incoming_message_events.as_ref().borrow_mut().to_vec();
        self.incoming_message_events.as_ref().borrow_mut().clear();

        let mut session_uuid_and_raw_message = vec![];

        for (session_uuid, batched_raw_messages) in incoming_message_events.iter() {
            let raw_messages = deserialize::<Vec<Vec<u8>>>(batched_raw_messages.clone());

            for raw_message in raw_messages.iter() {
                session_uuid_and_raw_message.push((session_uuid.clone(), raw_message.clone()));
            }
        }

        return session_uuid_and_raw_message;
    }

    pub fn get_close_events(self: &mut WebSocketClient) -> Vec<Uuid> {
        let close_events = self.close_events.as_ref().borrow_mut().to_vec();
        self.close_events.as_ref().borrow_mut().clear();
        return close_events;
    }
}

pub fn get_websocket_client() -> Rc<RefCell<WebSocketClient>> {
    let location: Location = window().unwrap().location();

    let protocol: &str = &location.protocol().unwrap();
    let host: &str = &location.host().unwrap();

    let mut ws_protocol = "ws:";
    if protocol.contains("https:") {
        ws_protocol = "wss:";
    }

    let server_uri = format!("{:}//{:}/ws", ws_protocol, host);

    return Rc::new(RefCell::new(WebSocketClient::new(server_uri.as_str())));
}
