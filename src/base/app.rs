use std::collections::HashSet;


use bevy::log::LogPlugin;
use bevy::math::IVec2;
use bevy::prelude::*;
use bevy::window::WindowPosition::At;
use bevy::window::{PresentMode, WindowResolution};
use bevy::DefaultPlugins;
use bevy_debug_text_overlay::OverlayPlugin;

use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};

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
use crate::constants::{BACKGROUND_COLOR, BASE_TIME_STEP, BOUNDS, PIXELS_PER_METER, TITLE};
use crate::identity::game::Game;
use crate::identity::particle::handle_particle;
use crate::types::event::{Despawn, Input, Join, Leave, Spawn, Update};
use crate::types::network::{Close, IncomingMessage, Open, OutgoingMessage};

pub fn get_base_app() -> App {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.to_string(),
                    resolution: WindowResolution::new(BOUNDS.x, BOUNDS.y),
                    present_mode: PresentMode::Fifo,
                    position: At(IVec2::new(0, 0)),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                filter: "eds_game_for_ftp_game_jam_2022=trace,wgpu_core=warn,bevy_render=warn"
                    .into(),
                level: bevy::log::Level::INFO,
            }),
    );

    app.add_plugins(OverlayPlugin {
        font_size: 10.0,
        ..default()
    });

    app.add_plugins(bevy_framepace::FramepacePlugin);

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ));

    app.insert_resource(Time::<Fixed>::from_seconds(BASE_TIME_STEP));

    app.insert_resource(ClearColor(BACKGROUND_COLOR));

    app.insert_resource(Game {
        role: "base".to_string(),
        local_player_uuid: None,
        player_uuids: HashSet::new(),
        last_update: 0.0,
        server_time_at_join: 0.0,
        client_time_at_join: 0.0,
    });

    app.add_systems(Startup, base_handle_setup);

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

    // handlers to wire network events into game events
    app.add_systems(Update, base_handle_open_event);
    app.add_systems(Update, base_handle_incoming_message_event);
    app.add_systems(Update, base_handle_close_event);

    // handlers to wire game events into game state
    app.add_systems(
        Update,
        base_handle_join_event.after(base_handle_incoming_message_event),
    );
    app.add_systems(
        Update,
        base_handle_spawn_event.after(base_handle_join_event),
    );
    app.add_systems(
        Update,
        base_handle_leave_event.after(base_handle_spawn_event),
    );
    app.add_systems(
        Update,
        handle_collision_event.after(base_handle_spawn_event),
    );
    app.add_systems(
        Update,
        base_handle_despawn_event.after(base_handle_leave_event),
    );

    // handlers to calculate game state per time step
    app.add_systems(FixedUpdate, handle_particle.after(handle_collision_event));
    app.add_systems(FixedUpdate, handle_expireable.after(handle_collision_event));

    // TODO: debugging related
    // app.add_plugins(RapierDebugRenderPlugin::default());
    // app.add_plugins(WorldInspectorPlugin::new());
    // app.add_plugins(LogDiagnosticsPlugin::default());
    // app.add_plugins(FrameTimeDiagnosticsPlugin);

    trace!("base.get_app(); returning app={:?}", app);

    app
}
