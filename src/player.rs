use std::borrow::BorrowMut;

use bevy::asset::Assets;
use bevy::math::Quat;
use bevy::prelude::{default, shape, Color, ColorMaterial, Commands, Component, KeyCode, Mesh, ResMut, Transform, Vec3};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::Uuid;
use bevy_rapier2d::dynamics::RigidBody::Dynamic;
use bevy_rapier2d::geometry::{ActiveEvents, Collider};
use bevy_rapier2d::prelude::{Ccd, ColliderMassProperties, Damping, Friction, Restitution, Sleeping, Velocity};

use crate::constants::{
    BOUNDS, FRICTION_COEFFICIENT, MATERIAL_SCALE, PLAYER_1_BACKWARD_KEY, PLAYER_1_COLOR, PLAYER_1_FIRE_KEY, PLAYER_1_FORWARD_KEY,
    PLAYER_1_LEFT_KEY, PLAYER_1_NAME, PLAYER_1_RIGHT_KEY, PLAYER_1_STARTING_ROTATION_MULTIPLIER, PLAYER_1_STARTING_TRANSLATION_MULTIPLIER,
    PLAYER_2_BACKWARD_KEY, PLAYER_2_COLOR, PLAYER_2_FIRE_KEY, PLAYER_2_FORWARD_KEY, PLAYER_2_LEFT_KEY, PLAYER_2_NAME, PLAYER_2_RIGHT_KEY,
    PLAYER_2_STARTING_ROTATION_MULTIPLIER, PLAYER_2_STARTING_TRANSLATION_MULTIPLIER, PLAYER_ANGULAR_DAMPING, PLAYER_DENSITY,
    PLAYER_LINEAR_DAMPING, RESTITUTION_COEFFICIENT,
};
use crate::weapon::{get_weapon, Weapon};

#[derive(Debug, Component)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub forward_key: KeyCode,
    pub backward_key: KeyCode,
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub fire_key: KeyCode,
    pub size: Vec3,
    pub weapon_uuid: Uuid,
}

pub fn get_player_and_weapon(
    name: String,
    forward_key: KeyCode,
    backward_key: KeyCode,
    left_key: KeyCode,
    right_key: KeyCode,
    fire_key: KeyCode,
) -> (Player, Weapon) {
    let weapon = get_weapon();

    let player = Player {
        uuid: Uuid::new_v4(),
        name,
        forward_key,
        backward_key,
        left_key,
        right_key,
        fire_key,
        size: Vec3::new(1.0, 1.5, 0.0) * MATERIAL_SCALE / 2.0, // TODO
        weapon_uuid: weapon.uuid,
    };

    return (player, weapon);
}

fn get_player_material_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    translation_multiplier: f32,
    rotation_multiplier: f32,
    color: Color,
) -> MaterialMesh2dBundle<ColorMaterial> {
    let mut scale = Vec3::splat(MATERIAL_SCALE);
    scale.y *= 1.25;

    let transform = Transform::default()
        .with_scale(scale)
        .with_translation(Vec3::new((BOUNDS.x / 2.0) * translation_multiplier, 0.0, 1.0))
        .with_rotation(Quat::from_rotation_z(f32::to_radians(360.0 * rotation_multiplier)));

    let mesh = meshes.add(Mesh::from(shape::RegularPolygon::new(0.5, 3))).into();

    let material = materials.add(ColorMaterial::from(color));

    return MaterialMesh2dBundle {
        mesh,
        transform,
        material,
        ..default()
    };
}

pub fn spawn_player(player: Player, player_material_mesh: MaterialMesh2dBundle<ColorMaterial>, weapon: Weapon, mut commands: Commands) {
    commands
        .spawn((player_material_mesh, player, weapon))
        .insert(Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::ball(0.5))
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

pub fn spawn_player_1(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, commands: Commands) {
    let player_and_weapon = get_player_and_weapon(
        PLAYER_1_NAME.to_string(),
        PLAYER_1_FORWARD_KEY,
        PLAYER_1_BACKWARD_KEY,
        PLAYER_1_LEFT_KEY,
        PLAYER_1_RIGHT_KEY,
        PLAYER_1_FIRE_KEY,
    );

    spawn_player(
        player_and_weapon.0,
        get_player_material_mesh(
            meshes.borrow_mut(),
            materials.borrow_mut(),
            PLAYER_1_STARTING_TRANSLATION_MULTIPLIER,
            PLAYER_1_STARTING_ROTATION_MULTIPLIER,
            PLAYER_1_COLOR,
        ),
        player_and_weapon.1,
        commands,
    );
}

pub fn spawn_player_2(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, commands: Commands) {
    let player_and_weapon = get_player_and_weapon(
        PLAYER_2_NAME.to_string(),
        PLAYER_2_FORWARD_KEY,
        PLAYER_2_BACKWARD_KEY,
        PLAYER_2_LEFT_KEY,
        PLAYER_2_RIGHT_KEY,
        PLAYER_2_FIRE_KEY,
    );

    spawn_player(
        player_and_weapon.0,
        get_player_material_mesh(
            meshes.borrow_mut(),
            materials.borrow_mut(),
            PLAYER_2_STARTING_TRANSLATION_MULTIPLIER,
            PLAYER_2_STARTING_ROTATION_MULTIPLIER,
            PLAYER_2_COLOR,
        ),
        player_and_weapon.1,
        commands,
    );
}
