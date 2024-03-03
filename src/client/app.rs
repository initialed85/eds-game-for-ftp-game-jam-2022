
use bevy::log::trace;
use bevy::prelude::*;



use crate::base::app::get_base_app;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
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
    app.add_systems(
        Update,
        handle_websocket_client
            .before(base_handle_open_event)
            .before(base_handle_incoming_message_event)
            .before(base_handle_close_event),
    );

    // handlers to wire game update event into game state
    app.add_systems(Update, handle_update_event.after(handle_websocket_client));

    // handler to wire raw input event into game input event
    app.add_systems(
        Update,
        handle_input_from_keyboard.after(handle_update_event),
    );
    app.add_systems(Update, handle_input_from_button.after(handle_update_event));

    // handler to wire game input event into network input event
    app.add_systems(Update, handle_input_event.after(handle_input_from_keyboard));

    // handlers to calculate game state per time step
    app.add_systems(
        FixedUpdate,
        handle_update_for_moveable.after(handle_update_event),
    );

    // TODO: debugging related
    // app.add_plugins(RapierDebugRenderPlugin::default());
    // app.add_plugins(WorldInspectorPlugin::new());
    // app.add_plugins(LogDiagnosticsPlugin::default());
    // app.add_plugins(FrameTimeDiagnosticsPlugin);

    trace!("client.get_app(); returning app={:?}", app);

    app
}
