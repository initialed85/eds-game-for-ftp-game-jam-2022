use bevy::asset::Assets;
use bevy::ecs::system::EntityCommands;
use bevy::math::Vec3;
use bevy::prelude::{
    default, shape, Color, ColorMaterial, Commands, Component, Entity, Mesh, Query, Res, ResMut, Time,
    Transform,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::Uuid;
use bevy_rapier2d::dynamics::RigidBody::Dynamic;
use bevy_rapier2d::dynamics::{Ccd, Damping, Sleeping};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, ColliderMassProperties, Friction, Restitution};
use bevy_rapier2d::prelude::Velocity;
use serde::{Deserialize, Serialize};

use crate::behaviour::collideable::Collideable;
use crate::behaviour::moveable::Moveable;
use crate::behaviour::weaponized::Weaponized;
use crate::client::error::{QuatEMA, Vec2EMA, Vec3EMA, EMA};
use crate::constants::{
    FRICTION_COEFFICIENT, MATERIAL_SCALE, PLAYER_ANGULAR_DAMPING, PLAYER_COLLIDER_BALL_RADIUS,
    PLAYER_DENSITY, PLAYER_HEIGHT_MULTIPLIER, PLAYER_LINEAR_DAMPING, PLAYER_NETWORK_EMA_SMOOTHING_FACTOR,
    PLAYER_NETWORK_UPDATE_RATE_SECONDS, PLAYER_POLYGON_RADIUS, PLAYER_POLYGON_SIDES, PLAYER_WIDTH_MULTIPLIER,
    RESTITUTION_COEFFICIENT,
};
use crate::identity::entity::{Local, Remote};
use crate::identity::game::Game;
use crate::types::event::Input;

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Player {
    pub player_uuid: Uuid,
    pub color: Color,
    pub is_local_player: bool,
    pub last_input: Option<Input>,
}

pub fn spawn_player(
    player_uuid: Uuid,
    game: &Res<Game>,
    player_query: &Query<&Player>,
    color: Color,
    transform: Transform,
    velocity: Velocity,
    _time: Time,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    for player in player_query.iter() {
        if player.player_uuid == player_uuid {
            return; // spawn not required
        }
    }

    let mut transform = transform.clone();

    let mut size = Vec3::splat(MATERIAL_SCALE);
    size.y *= PLAYER_HEIGHT_MULTIPLIER;
    size.x *= PLAYER_WIDTH_MULTIPLIER;

    transform.scale = size;

    let mesh = meshes
        .add(Mesh::from(shape::RegularPolygon::new(
            PLAYER_POLYGON_RADIUS,
            PLAYER_POLYGON_SIDES,
        )))
        .into();

    let material = materials.add(ColorMaterial::from(color));

    let material_mesh = MaterialMesh2dBundle {
        mesh,
        transform,
        material,
        ..default()
    };

    let is_local_player = game.local_player_uuid.is_some() && player_uuid == game.local_player_uuid.unwrap();

    let player = Player {
        player_uuid,
        color,
        is_local_player,
        last_input: None,
    };

    let moveable = Moveable {
        entity_uuid: player_uuid,
        last_update: None,
        translation_error: Vec3EMA::new(PLAYER_NETWORK_EMA_SMOOTHING_FACTOR),
        rotation_error: QuatEMA::new(PLAYER_NETWORK_EMA_SMOOTHING_FACTOR),
        linvel_error: Vec2EMA::new(PLAYER_NETWORK_EMA_SMOOTHING_FACTOR),
        angvel_error: EMA::new(PLAYER_NETWORK_EMA_SMOOTHING_FACTOR),
        update_rate_seconds: PLAYER_NETWORK_UPDATE_RATE_SECONDS,
        last_update_handled_at: 0.0,
        had_rollover: false,
    };

    let collideable = Collideable {
        entity_uuid: player_uuid,
    };

    let mut parent: EntityCommands;

    if is_local_player {
        parent = commands.spawn((material_mesh, player, moveable, collideable, Local {}));
    } else {
        let weaponized = Weaponized {
            weapon_uuid: Uuid::new_v4(),
            last_fired_at: 0.0,
        };

        parent = commands.spawn((
            material_mesh,
            player,
            moveable,
            weaponized,
            collideable,
            Remote {},
        ));
    }

    parent
        .insert(Dynamic)
        .insert(Sleeping::disabled())
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(velocity.clone());

    if game.role == "server" {
        parent
            .insert(Friction::coefficient(FRICTION_COEFFICIENT))
            .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
            .insert(Ccd::disabled())
            .insert(Collider::ball(PLAYER_COLLIDER_BALL_RADIUS))
            .insert(ColliderMassProperties::Density(PLAYER_DENSITY))
            .insert(ActiveEvents::all());
    }
}

pub fn despawn_player(
    player_uuid: Uuid,
    player_query: &Query<(Entity, &Player)>,
    _time: Time,
    commands: &mut Commands,
) {
    for (entity, player) in player_query.iter() {
        if player.player_uuid != player_uuid {
            continue;
        }

        let entity_commands = commands.get_entity(entity);
        if entity_commands.is_none() {
            continue;
        }

        entity_commands.unwrap().despawn();
    }
}
