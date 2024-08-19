use bevy::log::trace;
use bevy::prelude::{App, FixedUpdate, Startup};

use crate::base::app::{
    get_base_app, AfterNetworkTransition1, AfterNetworkTransition2, AfterNetworkTransition3,
    NetworkTransition,
};
use crate::client::input::{
    handle_input_event, handle_input_from_button, handle_input_from_keyboard, ButtonState,
};
use crate::client::moveable::handle_update_for_moveable;
use crate::client::network::handle_websocket_client;
use crate::client::setup::handle_setup;
use crate::client::update::handle_update_event;
use crate::client::websocket::get_websocket_client;

pub fn get_app_for_client() -> App {
    let mut app = get_base_app();

    let web_socket_client = get_websocket_client();

    trace!(
        "client.get_app(); created web_socket_client={:?}",
        web_socket_client
    );

    app.insert_resource(ButtonState {
        is_bottom_left_pressed: false,
        is_bottom_right_pressed: false,
    });

    app.add_systems(Startup, handle_setup);

    // the client side implementation of the WebSocket
    app.insert_non_send_resource(web_socket_client);

    // handler to wire the network implemention into the network events
    app.add_systems(NetworkTransition, handle_websocket_client);

    // handlers to wire game update event into game state
    app.add_systems(AfterNetworkTransition1, handle_update_event);

    // handler to wire raw input event into game input event
    app.add_systems(AfterNetworkTransition2, handle_input_from_keyboard);
    app.add_systems(AfterNetworkTransition2, handle_input_from_button);

    // handler to wire game input event into network input event
    app.add_systems(AfterNetworkTransition3, handle_input_event);

    // handlers to calculate game state per time step
    app.add_systems(FixedUpdate, handle_update_for_moveable);

    trace!("client.get_app(); returning app={:?}", app);

    app
}
