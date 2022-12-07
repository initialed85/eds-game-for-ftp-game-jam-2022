use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{ColorMaterial, Commands, Entity, EventReader, EventWriter, Mesh, Query, ResMut, Transform, Vec3};
use bevy_rapier2d::prelude::Velocity;

use crate::constants::{PLAYER_ANGULAR_VELOCITY_MAX, PLAYER_ANGULAR_VELOCITY_STEP, PLAYER_LINEAR_VELOCITY};
use crate::server_player::{spawn_other_player_at_server, spawn_this_player_at_server};
use crate::types::{FireWeapon, Player, PlayerMessage, Weapon};

pub fn handle_spawn_player_at_server(mut player_message_reader: EventReader<PlayerMessage>, mut commands: Commands) {
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
            let commands = commands.borrow_mut();
            spawn_this_player_at_server(player_message.player_uuid, player_message.color, transform, commands);
        } else {
            let commands = commands.borrow_mut();
            spawn_other_player_at_server(player_message.player_uuid, player_message.color, transform, commands);
        }

        println!("handle_spawn_player; player_message={:?}", player_message);
    }
}

pub fn handle_despawn_player_at_server(
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

        println!("handle_despawn_player; player={:?}", player);
    }
}

pub fn handle_player_input_at_server(
    mut player_message_reader: EventReader<PlayerMessage>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    mut fire_weapon_writer: EventWriter<FireWeapon>,
) {
    for incoming_player_message in player_message_reader.iter() {
        if !incoming_player_message.is_incoming {
            continue;
        }

        for (player, mut transform, mut velocity) in player_query.iter_mut() {
            if player.player_uuid != incoming_player_message.player_uuid {
                continue;
            }

            let mut linear_velocity = Vec3::ZERO;
            let mut angular_velocity = velocity.angvel.clone();

            if incoming_player_message.is_left {
                if angular_velocity <= PLAYER_ANGULAR_VELOCITY_MAX {
                    angular_velocity += PLAYER_ANGULAR_VELOCITY_STEP;
                }
            }

            if incoming_player_message.is_right {
                if angular_velocity >= -PLAYER_ANGULAR_VELOCITY_MAX {
                    angular_velocity -= PLAYER_ANGULAR_VELOCITY_STEP;
                }
            }

            if incoming_player_message.is_forward {
                linear_velocity.y = PLAYER_LINEAR_VELOCITY;
            }

            if incoming_player_message.is_backward {
                linear_velocity.y = -PLAYER_LINEAR_VELOCITY;
            }

            velocity.linvel += transform.rotation.mul_vec3(linear_velocity).truncate();
            velocity.angvel = angular_velocity;

            if incoming_player_message.is_firing {
                fire_weapon_writer.send(FireWeapon {
                    weapon_uuid: player.weapon_uuid,
                })
            }
        }
    }
}

pub fn handle_player_update_at_server(
    mut player_query: Query<(&mut Player, &mut Weapon, &mut Transform, &mut Velocity)>,
    mut fire_weapon_reader: EventReader<FireWeapon>,
    mut player_message_writer: EventWriter<PlayerMessage>,
) {
    let mut firing_weapon_uuids = vec![];
    for fire_weapon_event in fire_weapon_reader.iter() {
        firing_weapon_uuids.push(fire_weapon_event.weapon_uuid);
    }

    for (player, weapon, transform, velocity) in player_query.iter_mut() {
        let outgoing_player_message = PlayerMessage {
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
            has_input: false,
            is_left: false,
            is_right: false,
            is_forward: false,
            is_backward: false,
            is_firing: firing_weapon_uuids.contains(&weapon.weapon_uuid),
        };

        player_message_writer.send(outgoing_player_message);

        // if player_message.is_firing {
        //     fire_weapon_writer.send(FireWeapon {
        //         weapon_uuid: player.weapon_uuid,
        //     })
        // }
    }
}
