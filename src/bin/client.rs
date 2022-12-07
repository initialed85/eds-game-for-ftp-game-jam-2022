use bevy::log::LogPlugin;
use bevy::prelude::{default, App, ClearColor, PluginGroup, SystemSet, WindowDescriptor, WindowPlugin};
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Location};

use eds_game_for_ftp_game_jam_2022::client::{handle_client_read, handle_client_write};
use eds_game_for_ftp_game_jam_2022::client_player_system::{
    handle_despawn_player_at_client, handle_player_input_at_client, handle_player_update_at_client, handle_spawn_player_at_client,
};
use eds_game_for_ftp_game_jam_2022::collision_system::handle_collision_at_client;
use eds_game_for_ftp_game_jam_2022::constants::{BACKGROUND_COLOR, BOUNDS, PIXELS_PER_METER, TIME_STEP, TITLE};
use eds_game_for_ftp_game_jam_2022::map_rollover_system::{handle_player_map_rollover, handle_projectile_map_rollover};
use eds_game_for_ftp_game_jam_2022::particle_system::handle_particle;
use eds_game_for_ftp_game_jam_2022::projectile_system::handle_projectile;
use eds_game_for_ftp_game_jam_2022::setup::handle_setup;
use eds_game_for_ftp_game_jam_2022::types::{FireWeapon, PlayerMessage};
use eds_game_for_ftp_game_jam_2022::weapon_system::handle_player_weapon_at_client;
use eds_game_for_ftp_game_jam_2022::websocket_client::WebSocketClient;

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let location: Location = window().unwrap().location();

    let protocol: &str = &location.protocol().unwrap();
    let host: &str = &location.host().unwrap();

    let mut ws_protocol = "ws:";
    if protocol.contains("https:") {
        ws_protocol = "wss:";
    }

    let server_uri = format!("{:}//{:}/ws", ws_protocol, host);

    let web_socket_client = WebSocketClient::new(server_uri.as_str());

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
    .add_system_set(SystemSet::default().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
    .insert_non_send_resource(web_socket_client)
    .add_startup_system(handle_setup)
    .add_system(handle_client_read)
    .add_system(handle_client_write)
    .add_system(handle_player_input_at_client)
    .add_system(handle_player_update_at_client)
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

fn main() {
    // noop; run() is the actual entrypoint and #[wasm_bindgen(start)] seems to bring it's own main()
}
