use bevy::log::trace;
use bevy::prelude::{App, IntoSystemDescriptor};

use crate::base::app::get_base_app;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
};
use crate::client::network::handle_websocket_client;
use crate::client::websocket::get_websocket_client;

pub fn get_app_for_client() -> App {
    let mut app = get_base_app();

    let web_socket_client = get_websocket_client();

    trace!(
        "client.get_app(); created web_socket_client={:?}",
        web_socket_client
    );

    app.insert_non_send_resource(web_socket_client);

    app.add_system(
        handle_websocket_client
            .before(base_handle_open_event)
            .before(base_handle_incoming_message_event)
            .before(base_handle_close_event),
    );

    trace!("client.get_app(); returning app={:?}", app);

    return app;
}
