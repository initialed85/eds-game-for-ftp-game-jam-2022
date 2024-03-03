
use bevy::log::trace;
use bevy::prelude::*;



use crate::base::app::get_base_app;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
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

    // handler to wire the network implemention into the network events
    app.add_systems(
        Update,
        handle_websocket_server
            .before(base_handle_close_event)
            .before(base_handle_incoming_message_event)
            .before(base_handle_open_event),
    );

    // handlers to wire game events together
    app.add_systems(Update, handle_open_event.after(handle_websocket_server));
    app.add_systems(Update, handle_join_event.after(handle_open_event));
    app.add_systems(Update, handle_spawn_event.after(handle_join_event));
    app.add_systems(Update, handle_close_event.after(handle_spawn_event));
    app.add_systems(Update, handle_leave_event.after(handle_close_event));
    app.add_systems(Update, handle_despawn_event.after(handle_leave_event));

    // handlers to wire game events into game state
    app.add_systems(Update, handle_input_event.after(handle_websocket_server));
    app.add_systems(Update, handle_update_event.after(handle_input_event));
    app.add_systems(Update, handle_fire_event.after(handle_update_event));
    app.add_systems(
        Update,
        handle_rapier_collision_event.after(handle_fire_event),
    );
    app.add_systems(
        Update,
        handle_collision_event.after(handle_rapier_collision_event),
    );

    // handlers to calculate game state per time step
    app.add_systems(
        FixedUpdate,
        handle_input_for_player.after(handle_input_event),
    );
    app.add_systems(
        FixedUpdate,
        handle_rollover_for_moveable.after(handle_input_for_player),
    );
    app.add_systems(
        FixedUpdate,
        handle_update_for_moveable.after(handle_rollover_for_moveable),
    );
    app.add_systems(FixedUpdate, handle_expireable.after(handle_collision_event));

    // TODO: debugging related
    // app.add_plugins(RapierDebugRenderPlugin::default());
    // app.add_plugins(WorldInspectorPlugin::new());
    // app.add_plugins(LogDiagnosticsPlugin::default());
    // app.add_plugins(FrameTimeDiagnosticsPlugin);

    trace!("client.get_app(); returning app={:?}", app);

    app
}
