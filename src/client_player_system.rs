use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::input::Input;
use bevy::log::warn;
use bevy::prelude::{
    ColorMaterial, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, KeyCode, Mesh, Query, Res, ResMut, Time, Transform,
};
use bevy_rapier2d::dynamics::Velocity;

use crate::client_player::{spawn_other_player_at_client, spawn_this_player_at_client};
use crate::constants::PLAYER_UPDATE_INTERVAL;
use crate::types::{FireWeapon, Player, PlayerMessage};

pub fn handle_spawn_player_at_client(
    mut player_message_reader: EventReader<PlayerMessage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for player_message in player_message_reader.iter() {
        if !player_message.is_incoming {
            continue;
        }

        if player_message.event != "spawn" {
            continue;
        }

        let mut transform = Transform::default();
        transform.translation.x = player_message.translation_x;
        transform.translation.y = player_message.translation_y;
        transform.translation.z = player_message.translation_z;
        transform.rotation.x = player_message.rotation_x;
        transform.rotation.y = player_message.rotation_y;
        transform.rotation.z = player_message.rotation_z;
        transform.rotation.w = player_message.rotation_w;

        // let mut velocity = Velocity::default();
        // velocity.linvel.x = player_message.linvel_x;
        // velocity.linvel.y = player_message.linvel_y;
        // velocity.angvel = player_message.angvel;

        if player_message.is_for_this_player {
            let meshes = meshes.borrow_mut();
            let materials = materials.borrow_mut();
            let commands = commands.borrow_mut();
            spawn_this_player_at_client(
                player_message.player_uuid,
                player_message.color,
                transform,
                meshes,
                materials,
                commands,
            );
        } else {
            let meshes = meshes.borrow_mut();
            let materials = materials.borrow_mut();
            let commands = commands.borrow_mut();
            spawn_other_player_at_client(
                player_message.player_uuid,
                player_message.color,
                transform,
                meshes,
                materials,
                commands,
            );
        }

        warn!("handle_spawn_player; player_message={:?}", player_message);
    }
}

pub fn handle_despawn_player_at_client(
    mut player_message_reader: EventReader<PlayerMessage>,
    mut player_query: Query<(Entity, &Player)>,
    mut commands: Commands,
) {
    let mut player_uuids = vec![];

    for player_message in player_message_reader.iter() {
        if !player_message.is_incoming {
            continue;
        }

        if player_message.event != "despawn" {
            continue;
        }

        player_uuids.push(player_message.player_uuid);
    }

    if player_uuids.len() == 0 {
        return;
    }

    for (entity, player) in player_query.iter_mut() {
        if !player_uuids.contains(&player.player_uuid) {
            continue;
        }

        commands.entity(entity).despawn_recursive();

        warn!("handle_despawn_player; player={:?}", player);
    }
}

pub fn handle_player_input_at_client(
    mut player_query: Query<(&mut Player, &Transform, &Velocity)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_message_writer: EventWriter<PlayerMessage>,
) {
    for (mut _player, _transform, _velocity) in player_query.iter_mut() {
        let player: &mut Player = &mut _player;
        let transform: &Transform = _transform;
        let velocity: &Velocity = _velocity;

        if !player.is_local {
            continue;
        }

        let is_left = keyboard_input.pressed(player.left_key);
        let is_right = keyboard_input.pressed(player.right_key);
        let is_forward = keyboard_input.pressed(player.forward_key);
        let is_backward = keyboard_input.pressed(player.backward_key);
        let is_firing = keyboard_input.pressed(player.fire_key);

        let player_message = PlayerMessage {
            player_uuid: player.player_uuid,
            event: "update".to_string(),
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
            has_input: is_left || is_right || is_forward || is_backward || is_firing,
            is_left,
            is_right,
            is_forward,
            is_backward,
            is_firing,
        };

        player_message_writer.send(player_message);

        if time.elapsed_seconds_f64() - player.last_position_update < PLAYER_UPDATE_INTERVAL {
            continue;
        }

        player.last_position_update = time.elapsed_seconds_f64();
    }
}

pub fn handle_player_update_at_client(
    mut player_message_reader: EventReader<PlayerMessage>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    mut fire_weapon_writer: EventWriter<FireWeapon>,
) {
    for player_message in player_message_reader.iter() {
        if !player_message.is_incoming {
            continue;
        }

        for (player, mut transform, mut velocity) in player_query.iter_mut() {
            if player.player_uuid != player_message.player_uuid {
                continue;
            }

            transform.translation.x = player_message.translation_x;
            transform.translation.y = player_message.translation_y;
            transform.translation.z = player_message.translation_z;

            transform.rotation.x = player_message.rotation_x;
            transform.rotation.y = player_message.rotation_y;
            transform.rotation.z = player_message.rotation_z;
            transform.rotation.w = player_message.rotation_w;

            velocity.linvel.x = player_message.linvel_x;
            velocity.linvel.y = player_message.linvel_y;
            velocity.angvel = player_message.angvel;

            if player_message.is_firing {
                fire_weapon_writer.send(FireWeapon {
                    weapon_uuid: player.weapon_uuid,
                })
            }
        }
    }
}
