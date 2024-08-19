use crate::base::despawn::base_handle_despawn_event;
use crate::base::join::base_handle_join_event;
use crate::base::leave::base_handle_leave_event;
use crate::base::network::{
    base_handle_close_event, base_handle_incoming_message_event, base_handle_open_event,
};
use crate::base::setup::base_handle_setup;
use crate::base::spawn::base_handle_spawn_event;
use crate::behaviour::collideable::handle_collision_event;
use crate::behaviour::collideable::CollisionEvent;
use crate::behaviour::expireable::handle_expireable;
use crate::behaviour::weaponized::FireEvent;
use crate::constants::{BACKGROUND_COLOR, BASE_TIME_STEP, BOUNDS, PIXELS_PER_METER, TITLE};
use crate::identity::game::Game;
use crate::identity::particle::handle_particle;
use crate::types::event::{
    DespawnEvent, InputEvent, JoinEvent, LeaveEvent, SpawnEvent, UpdateEvent,
};
use crate::types::network::{CloseEvent, IncomingMessageEvent, OpenEvent, OutgoingMessageEvent};
use bevy::app::MainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::log::LogPlugin;
use bevy::math::IVec2;
use bevy::prelude::{
    default, trace, App, ClearColor, Fixed, FixedUpdate, PluginGroup, Schedule, Startup, Time,
    Update, Window, WindowPlugin,
};
use bevy::window::WindowPosition::At;
use bevy::window::{PresentMode, WindowResolution};
use bevy::DefaultPlugins;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use std::collections::HashSet;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct NetworkTransition;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct BaseNetworkTransition;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AfterNetworkTransition1;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AfterNetworkTransition2;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AfterNetworkTransition3;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AfterNetworkTransition4;

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
            .add(bevy_framepace::FramepacePlugin)
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
                PIXELS_PER_METER,
            ))
            .add(LogPlugin {
                filter: "eds_game_for_ftp_game_jam_2022=trace,wgpu_core=warn,bevy_render=warn"
                    .into(),
                level: bevy::log::Level::INFO,
                ..default()
            }),
    );

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

    let network_transition = Schedule::new(NetworkTransition);
    app.add_schedule(network_transition);

    let base_network_transition = Schedule::new(BaseNetworkTransition);
    app.add_schedule(base_network_transition);

    let after_network_transition_1 = Schedule::new(AfterNetworkTransition1);
    app.add_schedule(after_network_transition_1);

    let after_network_transition_2 = Schedule::new(AfterNetworkTransition2);
    app.add_schedule(after_network_transition_2);

    let after_network_transition_3 = Schedule::new(AfterNetworkTransition3);
    app.add_schedule(after_network_transition_3);

    let after_network_transition_4 = Schedule::new(AfterNetworkTransition4);
    app.add_schedule(after_network_transition_4);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_before(Update, AfterNetworkTransition4);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_before(AfterNetworkTransition4, AfterNetworkTransition3);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_before(AfterNetworkTransition3, AfterNetworkTransition2);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_before(AfterNetworkTransition2, AfterNetworkTransition1);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_before(AfterNetworkTransition1, BaseNetworkTransition);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_before(BaseNetworkTransition, NetworkTransition);

    app.add_systems(Startup, base_handle_setup);

    // register network events
    app.add_event::<OpenEvent>();
    app.add_event::<IncomingMessageEvent>();
    app.add_event::<OutgoingMessageEvent>();
    app.add_event::<CloseEvent>();

    // register game events
    app.add_event::<JoinEvent>();
    app.add_event::<SpawnEvent>();
    app.add_event::<InputEvent>();
    app.add_event::<UpdateEvent>();
    app.add_event::<DespawnEvent>();
    app.add_event::<LeaveEvent>();
    app.add_event::<FireEvent>();
    app.add_event::<CollisionEvent>();

    // handlers to wire base network events into server / client game events
    app.add_systems(BaseNetworkTransition, base_handle_open_event);
    app.add_systems(BaseNetworkTransition, base_handle_incoming_message_event);
    app.add_systems(BaseNetworkTransition, base_handle_close_event);

    // handlers to wire game events into game state
    app.add_systems(AfterNetworkTransition1, base_handle_join_event);
    app.add_systems(AfterNetworkTransition1, base_handle_leave_event);
    app.add_systems(AfterNetworkTransition2, base_handle_spawn_event);
    app.add_systems(AfterNetworkTransition2, base_handle_despawn_event);
    app.add_systems(AfterNetworkTransition3, handle_collision_event);

    // handlers to calculate game state per time step
    app.add_systems(FixedUpdate, handle_particle);
    app.add_systems(FixedUpdate, handle_expireable);

    trace!("base.get_app(); returning app={:?}", app);

    app
}
