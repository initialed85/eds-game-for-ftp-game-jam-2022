use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

use bevy::log::LogPlugin;
use bevy::prelude::{default, App, ClearColor, PluginGroup, SystemSet, WindowDescriptor, WindowPlugin};
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use ctrlc::set_handler;

use eds_game_for_ftp_game_jam_2022::client_player_system::{handle_despawn_player_at_client, handle_spawn_player_at_client};
use eds_game_for_ftp_game_jam_2022::collision_system::handle_collision_at_client;
use eds_game_for_ftp_game_jam_2022::constants::{BACKGROUND_COLOR, BOUNDS, PIXELS_PER_METER, TIME_STEP, TITLE};
use eds_game_for_ftp_game_jam_2022::map_rollover_system::{handle_player_map_rollover, handle_projectile_map_rollover};
use eds_game_for_ftp_game_jam_2022::particle_system::handle_particle;
use eds_game_for_ftp_game_jam_2022::projectile_system::handle_projectile;
use eds_game_for_ftp_game_jam_2022::server::{handle_server_read, handle_server_write, handle_websocket};
use eds_game_for_ftp_game_jam_2022::server_player_system::{handle_player_input_at_server, handle_player_update_at_server};
use eds_game_for_ftp_game_jam_2022::setup::handle_setup;
use eds_game_for_ftp_game_jam_2022::types::{FireWeapon, PlayerMessage};
use eds_game_for_ftp_game_jam_2022::weapon_system::handle_player_weapon_at_client;
use eds_game_for_ftp_game_jam_2022::websocket_server::WebSocketServer;

pub fn main() {
    set_handler(move || {
        exit(0);
    })
    .unwrap();

    let web_socket_server = Rc::new(RefCell::new(WebSocketServer::new("0.0.0.0".to_string(), 8080)));

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
                filter: "client=trace,eds-game-for-ftp-name=trace".into(),
                level: bevy::log::Level::WARN,
            }),
    )
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
    .add_event::<PlayerMessage>()
    .add_event::<FireWeapon>()
    .add_system_set(SystemSet::default().with_run_criteria(FixedTimestep::step(TIME_STEP)))
    .insert_non_send_resource(Rc::clone(&web_socket_server))
    .add_startup_system(handle_setup)
    .add_system(handle_websocket)
    .add_system(handle_server_read)
    .add_system(handle_server_write)
    .add_system(handle_player_input_at_server)
    .add_system(handle_player_update_at_server)
    .add_system(handle_spawn_player_at_client)
    .add_system(handle_despawn_player_at_client)
    .add_system(handle_particle)
    .add_system(handle_collision_at_client)
    .add_system(handle_player_map_rollover)
    .add_system(handle_projectile_map_rollover)
    .add_system(handle_projectile)
    .add_system(handle_player_weapon_at_client)
    .insert_resource(ClearColor(BACKGROUND_COLOR));

    app.run();
}
