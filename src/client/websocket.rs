use std::cell::RefCell;
use std::rc::Rc;

use bevy::reflect::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, BinaryType, ErrorEvent, Location, MessageEvent, WebSocket};

#[derive(Debug)]
pub struct WebSocketClient {
    _ws: WebSocket,
    open_events: Vec<Uuid>,
    incoming_message_events: Vec<(Uuid, String)>,
    close_events: Vec<Uuid>,
}

impl WebSocketClient {
    pub fn new(server_uri: &str) -> Rc<RefCell<WebSocketClient>> {
        let ws = WebSocket::new(server_uri).unwrap();

        ws.set_binary_type(BinaryType::Arraybuffer);

        let web_socket_client = WebSocketClient {
            _ws: ws.clone(),
            open_events: vec![],
            incoming_message_events: vec![],
            close_events: vec![],
        };

        let shareable_web_socket_client = Rc::new(RefCell::new(web_socket_client));

        let cloned_web_socket_client_for_onopen = Rc::clone(&shareable_web_socket_client);
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            let mut wsc = cloned_web_socket_client_for_onopen.as_ref().borrow_mut();

            wsc.open_events.push(Uuid::default());
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let cloned_web_socket_client_for_onmessage = Rc::clone(&shareable_web_socket_client);
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |m: MessageEvent| {
            if let Ok(buf) = m.data().dyn_into::<js_sys::ArrayBuffer>() {
                let data = String::from_utf8(js_sys::Uint8Array::new(&buf).to_vec()).unwrap();

                let mut wsc = cloned_web_socket_client_for_onmessage.as_ref().borrow_mut();
                wsc.incoming_message_events.push((Uuid::default(), data));
            }
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let cloned_web_socket_client_for_onerror = Rc::clone(&shareable_web_socket_client);
        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            let mut wsc = cloned_web_socket_client_for_onerror.as_ref().borrow_mut();

            wsc.close_events.push(Uuid::default());
        });

        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        return Rc::clone(&shareable_web_socket_client);
    }

    pub fn send(self: &WebSocketClient, session_uuid: Uuid, data: String) {
        let _ = session_uuid;
        let cloned_ws = self._ws.clone();

        cloned_ws.send_with_u8_array(&data.as_bytes().to_vec()).unwrap();
    }

    pub fn get_open_events(self: &mut WebSocketClient) -> Vec<Uuid> {
        let open_events = self.open_events.to_vec();
        self.open_events.clear();
        return open_events;
    }

    pub fn get_incoming_message_events(self: &mut WebSocketClient) -> Vec<(Uuid, String)> {
        let incoming_message_events = self.incoming_message_events.to_vec();
        self.incoming_message_events.clear();
        return incoming_message_events;
    }

    pub fn get_close_events(self: &mut WebSocketClient) -> Vec<Uuid> {
        let close_events = self.close_events.to_vec();
        self.close_events.clear();
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

    return WebSocketClient::new(server_uri.as_str());
}
