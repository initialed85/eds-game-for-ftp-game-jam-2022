use std::env::var;

use bevy::prelude::{default, App, ClearColor, PluginGroup, SystemSet, WindowDescriptor, WindowPlugin};
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use crate::collision_system::handle_collision;
use crate::constants::{BACKGROUND_COLOR, BOUNDS, PIXELS_PER_METER, TIME_STEP, TITLE};
use crate::map_rollover_system::{handle_player_map_rollover, handle_projectile_map_rollover};
use crate::particle_system::handle_particle;
use crate::player::{spawn_player_1, spawn_player_2};
use crate::player_system::handle_player;
use crate::projectile_system::handle_projectile;
use crate::setup::setup;
use crate::weapon_system::handle_player_weapon;

mod collision_system;
mod constants;
mod map_rollover_system;
mod particle;
mod particle_system;
mod player;
mod player_system;
mod projectile;
mod projectile_system;
mod setup;
mod weapon;
mod weapon_system;

fn main() {
    let debug_render = var("DEBUG_RENDER").unwrap_or_else(|_| "0".to_string()) == "1";

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: TITLE.to_string(),
            width: BOUNDS.x,
            height: BOUNDS.y,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }))
    // TODO: for framerate logging etc
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
    .add_system_set(SystemSet::default().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
    .add_startup_system(setup)
    .add_startup_system(spawn_player_1)
    .add_startup_system(spawn_player_2)
    // TODO: not sure if I like the effect
    // .add_system(handle_particle_map_rollover)
    .add_system(handle_particle)
    .add_system(handle_collision)
    .add_system(handle_player_map_rollover)
    .add_system(handle_player)
    .add_system(handle_projectile_map_rollover)
    .add_system(handle_projectile)
    .add_system(handle_player_weapon)
    .insert_resource(ClearColor(BACKGROUND_COLOR));

    if debug_render {
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}
