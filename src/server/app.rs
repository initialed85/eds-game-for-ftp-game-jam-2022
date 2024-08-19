use bevy::log::trace;
use bevy::prelude::{App, FixedUpdate, Startup};

use crate::base::app::{
    get_base_app, AfterNetworkTransition1, AfterNetworkTransition2, AfterNetworkTransition3,
    AfterNetworkTransition4, NetworkTransition,
};
use crate::base::rollover::handle_rollover_for_moveable;
use crate::behaviour::collideable::handle_rapier_collision_event;
use crate::behaviour::expireable::handle_expireable;
use crate::behaviour::weaponized::handle_fire_event;
use crate::server::collision::handle_collision_event;
use crate::server::despawn::handle_despawn_event;
use crate::server::input::{handle_input_event, handle_input_for_player};
use crate::server::join::handle_join_event;
use crate::server::leave::handle_leave_event;
use crate::server::moveable::handle_update_for_moveable;
use crate::server::network::{handle_close_event, handle_open_event, handle_websocket_server};
use crate::server::setup::handle_setup;
use crate::server::spawn::handle_spawn_event;
use crate::server::update::handle_update_event;
use crate::server::websocket::get_websocket_server;

pub fn get_app_for_server() -> App {
    let mut app = get_base_app();

    let web_socket_server = get_websocket_server();

    trace!(
        "client.get_app(); created web_socket_server={:?}",
        web_socket_server
    );

    app.add_systems(Startup, handle_setup);

    // the server side implementation of the WebSocket
    app.insert_non_send_resource(web_socket_server);

    // handler to wire the server network implemention into the base network events
    app.add_systems(NetworkTransition, handle_websocket_server);

    // handlers to wire game events together
    app.add_systems(AfterNetworkTransition1, handle_open_event);
    app.add_systems(AfterNetworkTransition1, handle_close_event);
    app.add_systems(AfterNetworkTransition2, handle_join_event);
    app.add_systems(AfterNetworkTransition2, handle_leave_event);
    app.add_systems(AfterNetworkTransition3, handle_spawn_event);
    app.add_systems(AfterNetworkTransition3, handle_despawn_event);

    // handlers to wire game events into game state
    app.add_systems(AfterNetworkTransition4, handle_input_event);
    app.add_systems(AfterNetworkTransition4, handle_update_event);
    app.add_systems(AfterNetworkTransition4, handle_fire_event);
    app.add_systems(AfterNetworkTransition4, handle_rapier_collision_event);
    app.add_systems(AfterNetworkTransition4, handle_collision_event);

    // handlers to calculate game state per time step
    app.add_systems(FixedUpdate, handle_input_for_player);
    app.add_systems(FixedUpdate, handle_rollover_for_moveable);
    app.add_systems(FixedUpdate, handle_update_for_moveable);
    app.add_systems(FixedUpdate, handle_expireable);

    trace!("client.get_app(); returning app={:?}", app);

    app
}
