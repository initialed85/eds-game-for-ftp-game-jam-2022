use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug)]
pub struct WebSocketClient {
    _ws: WebSocket,
    is_open: bool,
    messages: Vec<String>,
}

impl WebSocketClient {
    pub fn new(server_uri: &str) -> Rc<RefCell<WebSocketClient>> {
        // TODO: handle error (JsValue?)
        let ws = WebSocket::new(server_uri).unwrap();
        ws.set_binary_type(BinaryType::Arraybuffer);

        let web_socket_client = WebSocketClient {
            _ws: ws.clone(),
            is_open: false,
            messages: vec![],
        };

        let shareable_web_socket_client = Rc::new(RefCell::new(web_socket_client));

        let cloned_web_socket_client_for_onopen = Rc::clone(&shareable_web_socket_client);
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            let mut wsc = cloned_web_socket_client_for_onopen.as_ref().borrow_mut();
            wsc.is_open = true;
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let cloned_web_socket_client_for_onmessage = Rc::clone(&shareable_web_socket_client);
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |m: MessageEvent| {
            if let Ok(buf) = m.data().dyn_into::<js_sys::ArrayBuffer>() {
                // TODO: handle error
                let data = String::from_utf8(js_sys::Uint8Array::new(&buf).to_vec()).unwrap();

                let mut wsc = cloned_web_socket_client_for_onmessage.as_ref().borrow_mut();
                wsc.messages.insert(0, data);
            } else {
                console_log!("onmessage; failed to decode m={:?}", m.data());
            }
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let cloned_web_socket_client_for_onerror = Rc::clone(&shareable_web_socket_client);
        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            console_log!("onerror; e={:?}", e.message());

            let mut wsc = cloned_web_socket_client_for_onerror.as_ref().borrow_mut();
            wsc.is_open = false;
        });

        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        return Rc::clone(&shareable_web_socket_client);
    }

    pub fn send(self: &WebSocketClient, data: String) {
        let cloned_ws = self._ws.clone();

        // TODO: handle error (JsValue?)
        cloned_ws.send_with_u8_array(&data.as_bytes().to_vec()).unwrap();
    }

    pub fn get_messages(self: &mut WebSocketClient) -> Vec<String> {
        let mut messages = vec![];

        loop {
            let result = self.messages.pop();
            if result.is_none() {
                break;
            }

            messages.insert(0, result.unwrap());
        }

        return messages;
    }
}
