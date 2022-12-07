use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::prelude::{default, shape, Color, ColorMaterial, Commands, Mesh, ResMut, Transform, Vec3};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::Uuid;
use bevy_rapier2d::dynamics::RigidBody::Dynamic;
use bevy_rapier2d::geometry::{ActiveEvents, Collider};
use bevy_rapier2d::prelude::{Ccd, ColliderMassProperties, Damping, Friction, Restitution, Sleeping, Velocity};

use crate::common_player;
use crate::constants::{
    FRICTION_COEFFICIENT, MATERIAL_SCALE, PLAYER_1_BACKWARD_KEY, PLAYER_1_FIRE_KEY, PLAYER_1_FORWARD_KEY, PLAYER_1_LEFT_KEY, PLAYER_1_NAME,
    PLAYER_1_RIGHT_KEY, PLAYER_2_BACKWARD_KEY, PLAYER_2_FIRE_KEY, PLAYER_2_FORWARD_KEY, PLAYER_2_LEFT_KEY, PLAYER_2_NAME,
    PLAYER_2_RIGHT_KEY, PLAYER_ANGULAR_DAMPING, PLAYER_COLLIDER_BALL_RADIUS, PLAYER_DENSITY, PLAYER_HEIGHT_MULTIPLIER,
    PLAYER_LINEAR_DAMPING, PLAYER_POLYGON_RADIUS, PLAYER_POLYGON_SIDES, PLAYER_WIDTH_MULTIPLIER, RESTITUTION_COEFFICIENT,
};
use crate::types::{Player, Weapon};

pub fn get_player_material_mesh(
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut transform: Transform,
    color: Color,
) -> MaterialMesh2dBundle<ColorMaterial> {
    let mut size = Vec3::splat(MATERIAL_SCALE);
    size.y *= PLAYER_HEIGHT_MULTIPLIER;
    size.x *= PLAYER_WIDTH_MULTIPLIER;

    transform.scale = size;

    let mesh = meshes
        .add(Mesh::from(shape::RegularPolygon::new(PLAYER_POLYGON_RADIUS, PLAYER_POLYGON_SIDES)))
        .into();

    let material = materials.add(ColorMaterial::from(color));

    return MaterialMesh2dBundle {
        mesh,
        transform,
        material,
        ..default()
    };
}

pub fn spawn_player_at_server(player: Player, weapon: Weapon, commands: &mut Commands) {
    commands
        .spawn((player, weapon))
        .insert(Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::disabled())
        .insert(Collider::ball(PLAYER_COLLIDER_BALL_RADIUS))
        .insert(Friction::coefficient(FRICTION_COEFFICIENT))
        .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
        .insert(ColliderMassProperties::Density(PLAYER_DENSITY))
        .insert(Velocity::zero())
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(ActiveEvents::all());
}

pub fn spawn_this_player_at_server(player_uuid: Uuid, color: Color, transform: Transform, commands: &mut Commands) {
    let player_and_weapon = common_player::get_player_and_weapon(
        player_uuid,
        color,
        PLAYER_1_NAME.to_string(),
        PLAYER_1_FORWARD_KEY,
        PLAYER_1_BACKWARD_KEY,
        PLAYER_1_LEFT_KEY,
        PLAYER_1_RIGHT_KEY,
        PLAYER_1_FIRE_KEY,
        true,
        transform,
    );

    spawn_player_at_server(player_and_weapon.0, player_and_weapon.1, commands);
}

pub fn spawn_other_player_at_server(player_uuid: Uuid, color: Color, transform: Transform, commands: &mut Commands) {
    let player_and_weapon = common_player::get_player_and_weapon(
        player_uuid,
        color,
        PLAYER_2_NAME.to_string(),
        PLAYER_2_FORWARD_KEY,
        PLAYER_2_BACKWARD_KEY,
        PLAYER_2_LEFT_KEY,
        PLAYER_2_RIGHT_KEY,
        PLAYER_2_FIRE_KEY,
        false,
        transform,
    );

    spawn_player_at_server(player_and_weapon.0, player_and_weapon.1, commands.borrow_mut());
}
