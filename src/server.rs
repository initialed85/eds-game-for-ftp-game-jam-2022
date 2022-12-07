use std::cell::RefCell;
use std::rc::Rc;

use bevy::math::Quat;
use bevy::prelude::{Color, EventReader, EventWriter, NonSend, Query, Transform};
use bevy_rapier2d::dynamics::Velocity;
use rand::{thread_rng, Rng};

use crate::constants::{BOUNDS, DEGREES_MAX, HALF, PLAYER_Z_INDEX};
use crate::types::{Player, PlayerMessage};
use crate::websocket_server::WebSocketServer;

pub fn handle_websocket(web_socket_server: NonSend<Rc<RefCell<WebSocketServer>>>) {
    let mut web_socket_server = web_socket_server.borrow_mut();

    web_socket_server.handle();
}

pub fn handle_server_read(
    web_socket_server: NonSend<Rc<RefCell<WebSocketServer>>>,
    mut player_message_writer: EventWriter<PlayerMessage>,
    player_query: Query<(&Player, &Transform, &Velocity)>,
) {
    let mut web_socket_server = web_socket_server.borrow_mut();

    let open_events = web_socket_server.get_open_events();
    let incoming_message_events = web_socket_server.get_incoming_message_events();
    let close_events = web_socket_server.get_close_events();

    //
    // handle joining player
    //

    for session_uuid in open_events.iter() {
        // start of life for a player
        let rotation = Quat::from_rotation_z(f32::to_radians(DEGREES_MAX * thread_rng().gen::<f32>()));
        let mut player_message = PlayerMessage {
            player_uuid: session_uuid.clone(),
            event: "spawn".to_string(),
            color: Color::rgb(thread_rng().gen::<f32>(), thread_rng().gen::<f32>(), thread_rng().gen::<f32>()),
            is_incoming: false,
            is_for_this_player: false,
            translation_x: thread_rng().gen::<f32>() * BOUNDS.x - (BOUNDS.x * HALF),
            translation_y: thread_rng().gen::<f32>() * BOUNDS.y - (BOUNDS.y * HALF),
            translation_z: PLAYER_Z_INDEX,
            rotation_x: rotation.x,
            rotation_y: rotation.y,
            rotation_z: rotation.z,
            rotation_w: rotation.w,
            linvel_x: 0.0,
            linvel_y: 0.0,
            angvel: 0.0,
            has_input: false,
            is_left: false,
            is_right: false,
            is_forward: false,
            is_backward: false,
            is_firing: false,
        };

        // tell the server to spawn the joining player
        player_message.is_incoming = true;
        player_message.is_for_this_player = false;
        player_message_writer.send(player_message.clone());

        // tell the joining player about itself
        player_message.is_incoming = false;
        player_message.is_for_this_player = true;
        let player_message_json_result = serde_json::to_string(&player_message);
        if player_message_json_result.is_err() {
            continue;
        }
        let player_message_json = player_message_json_result.unwrap();
        web_socket_server.send(*session_uuid, player_message_json);

        // sync the joining player and the other players
        for (_player, _transform, _velocity) in player_query.iter() {
            let player: &Player = _player;
            let transform: &Transform = _transform;
            let velocity: &Velocity = _velocity;

            if player.player_uuid == player_message.player_uuid {
                continue;
            }

            let other_player_message = PlayerMessage {
                player_uuid: player.player_uuid,
                event: "spawn".to_string(),
                color: player.color,
                is_incoming: false,
                is_for_this_player: false,
                translation_x: transform.translation.x,
                translation_y: transform.translation.y,
                translation_z: transform.translation.z,
                rotation_x: transform.rotation.x,
                rotation_y: transform.rotation.y,
                rotation_z: transform.rotation.z,
                rotation_w: transform.rotation.w,
                linvel_x: velocity.linvel.x,
                linvel_y: velocity.linvel.y,
                angvel: velocity.angvel,
                has_input: false,
                is_left: false,
                is_right: false,
                is_forward: false,
                is_backward: false,
                is_firing: false,
            };

            // tell the joining player about the other player
            let other_player_message_json_result = serde_json::to_string(&other_player_message);
            if other_player_message_json_result.is_err() {
                continue;
            }
            let other_player_message_json = other_player_message_json_result.unwrap();
            web_socket_server.send(*session_uuid, other_player_message_json);

            // tell the other player about the joining player
            player_message.is_incoming = false;
            player_message.is_for_this_player = false;
            let player_message_json_result = serde_json::to_string(&player_message);
            if player_message_json_result.is_err() {
                continue;
            }
            let player_message_json = player_message_json_result.unwrap();
            web_socket_server.send(player.player_uuid, player_message_json);
        }

        println!("spawn={:?}", player_message);
    }

    //
    // handle player updates
    //

    for (_session_uuid, message) in incoming_message_events.iter() {
        // tell the server about all player inputs etc
        let player_message_result = serde_json::from_str::<PlayerMessage>(message);
        if player_message_result.is_err() {
            continue;
        }
        let mut player_message = player_message_result.unwrap();
        player_message.is_incoming = true;
        player_message.is_for_this_player = false;
        player_message_writer.send(player_message.clone());
    }

    //
    // handle leaving player
    //

    for session_uuid in close_events.iter() {
        // end of life for the a player
        let mut player_message = PlayerMessage {
            player_uuid: session_uuid.clone(),
            event: "despawn".to_string(),
            color: Default::default(),
            is_incoming: false,
            is_for_this_player: false,
            translation_x: 0.0,
            translation_y: 0.0,
            translation_z: 0.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            rotation_w: 0.0,
            linvel_x: 0.0,
            linvel_y: 0.0,
            angvel: 0.0,
            has_input: false,
            is_left: false,
            is_right: false,
            is_forward: false,
            is_backward: false,
            is_firing: false,
        };

        // tell all players to despawn the leaving player
        player_message_writer.send(player_message.clone());
        let player_message_json_result = serde_json::to_string(&player_message);
        if player_message_json_result.is_err() {
            continue;
        }
        let player_message_json = player_message_json_result.unwrap();
        web_socket_server.broadcast(player_message_json);

        // tell the server to despawn the leaving player
        player_message.is_incoming = true;
        player_message.is_for_this_player = false;
        player_message_writer.send(player_message.clone());

        println!("despawn={:?}", player_message);
    }
}

pub fn handle_server_write(
    mut player_message_reader: EventReader<PlayerMessage>,
    web_socket_server: NonSend<Rc<RefCell<WebSocketServer>>>,
    player_query: Query<(&Player, &Transform, &Velocity)>,
) {
    let mut web_socket_server = web_socket_server.borrow_mut();

    // tell all players about their physics updates in response to their inputs etc
    for player_message in player_message_reader.iter() {
        if player_message.is_incoming {
            continue;
        }

        if player_message.event != "update" {
            continue;
        }

        let mut player_message = player_message.clone();

        for (_player, _transform, _velocity) in player_query.iter() {
            let player: &Player = _player;

            player_message.is_for_this_player = player.player_uuid == player_message.player_uuid;

            let player_message_json_result = serde_json::to_string(&player_message);
            if player_message_json_result.is_err() {
                continue;
            }
            let player_message_json = player_message_json_result.unwrap();

            web_socket_server.send(player.player_uuid, player_message_json);
        }
    }
}
