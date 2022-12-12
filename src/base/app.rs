use bevy::log::LogPlugin;
use bevy::prelude::{
    default, trace, App, ClearColor, IntoSystemDescriptor, PluginGroup, SystemSet, WindowDescriptor,
    WindowPlugin,
};
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
};
use crate::base::setup::base_handle_setup;
use crate::base::spawn::base_handle_spawn_event;
use crate::behaviour::can_move::base_handle_can_move;
use crate::constants::{BACKGROUND_COLOR, BOUNDS, PIXELS_PER_METER, TIME_STEP, TITLE};
use crate::types::event::{Despawn, Join, Leave, Spawn, Update};
use crate::types::network::{Close, IncomingMessage, Open, OutgoingMessage};

pub fn get_base_app() -> App {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: TITLE.to_string(),
                    width: BOUNDS.x,
                    height: BOUNDS.y,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                ..default()
            })
            .set(LogPlugin {
                filter: "eds_game_for_ftp_game_jam_2022=trace".into(),
                level: bevy::log::Level::WARN,
            }),
    );

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ));

    app.add_system_set(SystemSet::default().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)));

    app.insert_resource(ClearColor(BACKGROUND_COLOR));

    app.add_startup_system(base_handle_setup);

    app.add_event::<Open>();
    app.add_event::<IncomingMessage>();
    app.add_event::<OutgoingMessage>();
    app.add_event::<Close>();
    app.add_event::<Join>();
    app.add_event::<Spawn>();
    app.add_event::<Update>();
    app.add_event::<Despawn>();
    app.add_event::<Leave>();

    app.add_system(base_handle_open_event);
    app.add_system(base_handle_incoming_message_event);
    app.add_system(base_handle_close_event);

    app.add_system(base_handle_can_move.after(base_handle_incoming_message_event));
    app.add_system(base_handle_spawn_event.after(base_handle_incoming_message_event));

    app.add_plugin(RapierDebugRenderPlugin::default());

    trace!("base.get_app(); returning app={:?}", app);

    return app;
}
