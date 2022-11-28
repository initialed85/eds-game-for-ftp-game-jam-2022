use std::env::var;

use bevy::prelude::{App, SystemSet};
use bevy::time::FixedTimestep;
use bevy::DefaultPlugins;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use crate::collision_system::handle_collision;
use crate::constants::TIME_STEP;
use crate::map_rollover_system::{handle_player_map_rollover, handle_projectile_map_rollover};
use crate::player::{spawn_player_1, spawn_player_2};
use crate::player_movement_system::handle_player_movement;
use crate::projectile_system::handle_projectile;
use crate::weapon_system::handle_player_weapon;

mod collision_system;
mod constants;
mod map_rollover_system;
mod player;
mod player_movement_system;
mod projectile;
mod projectile_system;
mod setup;
mod weapon;
mod weapon_system;

fn main() {
    let debug_render = var("DEBUG_RENDER").unwrap_or_else(|_| "0".to_string()) == "1";

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_system_set(SystemSet::default().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        .add_startup_system(setup::setup)
        .add_startup_system(spawn_player_1)
        .add_startup_system(spawn_player_2)
        .add_system(handle_player_map_rollover)
        .add_system(handle_player_movement)
        .add_system(handle_projectile_map_rollover)
        .add_system(handle_projectile)
        .add_system(handle_player_weapon)
        .add_system(handle_collision);

    if debug_render {
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}
