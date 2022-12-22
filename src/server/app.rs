use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::trace;
use bevy::prelude::{App, IntoSystemDescriptor};
use bevy::time::FixedTimestep;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;

use crate::base::app::get_base_app;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
};
use crate::base::rollover::handle_rollover_for_moveable;
use crate::behaviour::collideable::handle_rapier_collision_event;
use crate::behaviour::expireable::handle_expireable;
use crate::behaviour::weaponized::handle_fire_event;
use crate::constants::TIME_STEP;
use crate::server::collision::handle_collision_event;
use crate::server::despawn::handle_despawn_event;
use crate::server::input::{handle_input_event, handle_input_for_player};
use crate::server::join::handle_join_event;
use crate::server::leave::handle_leave_event;
use crate::server::network::{handle_close_event, handle_open_event, handle_websocket_server};
use crate::server::setup::handle_setup;
use crate::server::spawn::handle_spawn_event;
use crate::server::update::{handle_update_event, handle_update_for_moveable};
use crate::server::websocket::get_websocket_server;

pub fn get_app_for_server() -> App {
    let mut app = get_base_app();

    let web_socket_server = get_websocket_server();

    trace!(
        "client.get_app(); created web_socket_server={:?}",
        web_socket_server
    );

    app.add_startup_system(handle_setup);

    // the server side of the WebSocket
    app.insert_non_send_resource(web_socket_server);

    // network handlers
    app.add_system(
        handle_websocket_server
            .before(base_handle_close_event)
            .before(base_handle_incoming_message_event)
            .before(base_handle_open_event),
    );

    // game lifecycle handlers
    app.add_system(handle_open_event.after(handle_websocket_server));
    app.add_system(handle_join_event.after(handle_open_event));
    app.add_system(handle_spawn_event.after(handle_join_event));
    app.add_system(handle_close_event.after(handle_spawn_event));
    app.add_system(handle_leave_event.after(handle_close_event));
    app.add_system(handle_despawn_event.after(handle_leave_event));

    // game input / update handlers
    app.add_system(
        handle_input_event
            .after(handle_websocket_server)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
    );
    app.add_system(
        handle_input_for_player
            .after(handle_input_event)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
    );
    app.add_system(
        handle_rollover_for_moveable
            .before(handle_update_for_moveable)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
    );
    app.add_system(
        handle_update_for_moveable
            .after(handle_input_for_player)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
    );
    app.add_system(
        handle_update_event
            .after(handle_update_for_moveable)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
    );
    app.add_system(handle_fire_event.after(handle_update_event));
    app.add_system(handle_rapier_collision_event.after(handle_fire_event));
    app.add_system(handle_collision_event.after(handle_rapier_collision_event));
    app.add_system(handle_expireable.after(handle_collision_event));

    let _ = RapierDebugRenderPlugin::default();
    let _ = WorldInspectorPlugin::new();
    let _ = LogDiagnosticsPlugin::default();
    let _ = FrameTimeDiagnosticsPlugin::default();

    // TODO: debugging related
    // app.add_plugin(RapierDebugRenderPlugin::default());
    // app.add_plugin(WorldInspectorPlugin::new());
    // app.add_plugin(LogDiagnosticsPlugin::default());
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    trace!("client.get_app(); returning app={:?}", app);

    return app;
}
