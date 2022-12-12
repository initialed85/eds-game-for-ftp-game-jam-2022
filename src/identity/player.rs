use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{
    default, shape, trace, Color, ColorMaterial, Commands, Component, Mesh, Query, ResMut, Time, Transform,
};
use bevy::reflect::Uuid;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::RigidBody::Dynamic;
use bevy_rapier2d::dynamics::{Ccd, Damping, Sleeping};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, ColliderMassProperties, Friction, Restitution};
use bevy_rapier2d::prelude::Velocity;
use serde::{Deserialize, Serialize};

use crate::behaviour::can_move::CanMove;
use crate::constants::{
    FRICTION_COEFFICIENT, MATERIAL_SCALE, PLAYER_ANGULAR_DAMPING, PLAYER_COLLIDER_BALL_RADIUS,
    PLAYER_DENSITY, PLAYER_HEIGHT_MULTIPLIER, PLAYER_LINEAR_DAMPING, PLAYER_POLYGON_RADIUS,
    PLAYER_POLYGON_SIDES, PLAYER_WIDTH_MULTIPLIER, RESTITUTION_COEFFICIENT,
};

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Player {
    pub player_uuid: Uuid,
}

pub fn spawn_player(
    player_uuid: Uuid,
    color: Color,
    transform: Transform,
    velocity: Velocity,
    time: Time,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
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

    let player = Player { player_uuid };

    let can_move = CanMove {};

    trace!("spawning bundle={:?}", player);

    commands
        .spawn((material_mesh, player, can_move))
        .insert(Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::disabled())
        .insert(Collider::ball(PLAYER_COLLIDER_BALL_RADIUS))
        .insert(Friction::coefficient(FRICTION_COEFFICIENT))
        .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
        .insert(ColliderMassProperties::Density(PLAYER_DENSITY))
        .insert(velocity.clone())
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(ActiveEvents::all());
}
