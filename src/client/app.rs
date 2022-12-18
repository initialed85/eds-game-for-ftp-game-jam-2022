use bevy::log::trace;
use bevy::prelude::{App, IntoSystemDescriptor};

use crate::base::app::get_base_app;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
};
use crate::client::input::{handle_input_event, handle_input_from_keyboard};
use crate::client::network::handle_websocket_client;
use crate::client::setup::handle_setup;
use crate::client::update::{handle_update_event, handle_update_for_moveable};
use crate::client::websocket::get_websocket_client;

pub fn get_app_for_client() -> App {
    let mut app = get_base_app();

    let web_socket_client = get_websocket_client();

    trace!(
        "client.get_app(); created web_socket_client={:?}",
        web_socket_client
    );

    app.add_startup_system(handle_setup);

    // the client side of the WebSocket
    app.insert_non_send_resource(web_socket_client);

    // network handlers
    app.add_system(
        handle_websocket_client
            .before(base_handle_open_event)
            .before(base_handle_incoming_message_event)
            .before(base_handle_close_event),
    );

    // gane input / update handlers
    app.add_system(handle_input_from_keyboard.before(handle_websocket_client));
    app.add_system(handle_input_event.before(handle_input_from_keyboard));
    app.add_system(handle_update_event.after(handle_input_event));
    app.add_system(handle_update_for_moveable.after(handle_update_event));

    trace!("client.get_app(); returning app={:?}", app);

    return app;
}
