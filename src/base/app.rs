use std::collections::HashSet;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::math::Vec2;
use bevy::prelude::{
    default, trace, App, ClearColor, IntoSystemDescriptor, PluginGroup, SystemSet, WindowDescriptor,
    WindowPlugin,
};
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
use bevy::window::WindowPosition::At;
use bevy::DefaultPlugins;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use crate::base::despawn::base_handle_despawn_event;
use crate::base::join::base_handle_join_event;
use crate::base::leave::base_handle_leave_event;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
};
use crate::base::setup::base_handle_setup;
use crate::base::spawn::base_handle_spawn_event;
use crate::behaviour::collideable::handle_collision_event;
use crate::behaviour::collideable::Collision;
use crate::behaviour::expireable::handle_expireable;
use crate::behaviour::weaponized::Fire;
use crate::constants::{BACKGROUND_COLOR, BOUNDS, PIXELS_PER_METER, TIME_STEP, TITLE};
use crate::identity::game::Game;
use crate::identity::particle::handle_particle;
use crate::types::event::{Despawn, Input, Join, Leave, Spawn, Update};
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
                    position: At(Vec2::new(0.0, 50.0)),
                    ..default()
                },
                ..default()
            })
            .set(LogPlugin {
                filter: "eds_game_for_ftp_game_jam_2022=trace,wgpu_core=warn,bevy_render=warn".into(),
                level: bevy::log::Level::INFO,
            }),
    );

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ));

    app.add_system_set(SystemSet::default().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)));

    app.insert_resource(ClearColor(BACKGROUND_COLOR));

    app.insert_resource(Game {
        role: "base".to_string(),
        local_player_uuid: None,
        player_uuids: HashSet::new(),
        last_update: 0.0,
        server_time_at_join: 0.0,
        client_time_at_join: 0.0,
    });

    app.add_startup_system(base_handle_setup);

    // register network events
    app.add_event::<Open>();
    app.add_event::<IncomingMessage>();
    app.add_event::<OutgoingMessage>();
    app.add_event::<Close>();

    // register game events
    app.add_event::<Join>();
    app.add_event::<Spawn>();
    app.add_event::<Input>();
    app.add_event::<Update>();
    app.add_event::<Despawn>();
    app.add_event::<Leave>();
    app.add_event::<Fire>();
    app.add_event::<Collision>();

    // network handlers
    app.add_system(base_handle_open_event);
    app.add_system(base_handle_incoming_message_event);
    app.add_system(base_handle_close_event);

    // game lifecycle handlers
    app.add_system(base_handle_join_event.after(base_handle_incoming_message_event));
    app.add_system(base_handle_spawn_event.after(base_handle_join_event));
    app.add_system(base_handle_leave_event.after(base_handle_spawn_event));
    app.add_system(handle_collision_event.after(base_handle_spawn_event));
    app.add_system(handle_particle.after(handle_collision_event));
    app.add_system(handle_expireable.after(handle_collision_event));
    app.add_system(base_handle_despawn_event.after(base_handle_leave_event));

    let _ = RapierDebugRenderPlugin::default();
    let _ = WorldInspectorPlugin::new();
    let _ = LogDiagnosticsPlugin::default();
    let _ = FrameTimeDiagnosticsPlugin::default();

    // TODO: debugging related
    // app.add_plugin(RapierDebugRenderPlugin::default());
    // app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(LogDiagnosticsPlugin::default());
    app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    trace!("base.get_app(); returning app={:?}", app);

    return app;
}
