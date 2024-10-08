use bevy::asset::Assets;
use bevy::ecs::system::EntityCommands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Color, ColorMaterial, Commands, Component, Entity, Mesh, Query, Rectangle, Res,
    ResMut, Time, Transform,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::RigidBody::Dynamic;
use bevy_rapier2d::dynamics::{Ccd, Sleeping, Velocity};
use bevy_rapier2d::geometry::{
    ActiveEvents, Collider, ColliderMassProperties, Friction, Restitution,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::behaviour::collideable::Collideable;
use crate::behaviour::expireable::Expireable;
use crate::behaviour::moveable::Moveable;
use crate::client::error::{QuatEMA, Vec2EMA, Vec3EMA, EMA};
use crate::constants::{
    FRICTION_COEFFICIENT, MATERIAL_SCALE, PROEJCTILE_DIMENSION_MULTIPLIER, PROJECTILE_DENSITY,
    PROJECTILE_EXPIRY_SECONDS, PROJECTILE_NETWORK_EMA_SMOOTHING_FACTOR,
    PROJECTILE_NETWORK_UPDATE_RATE_SECONDS, RESTITUTION_COEFFICIENT,
};
use crate::identity::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Projectile {
    pub projectile_uuid: Uuid,
    pub weapon_uuid: Uuid,
}

pub fn spawn_projectile(
    projectile_uuid: Uuid,
    game: &Res<Game>,
    projectile_query: &Query<&Projectile>,
    color: Color,
    transform: Transform,
    velocity: Velocity,
    time: Time,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    for projectile in projectile_query.iter() {
        if projectile.projectile_uuid == projectile_uuid {
            return; // spawn not required
        }
    }

    let mut transform = transform;

    let mut size = Vec3::splat(MATERIAL_SCALE);
    size.y *= PROEJCTILE_DIMENSION_MULTIPLIER;
    size.x *= PROEJCTILE_DIMENSION_MULTIPLIER;

    transform.scale = size;

    let rect = Rectangle::from_size(Vec2::new(
        1.0 * PROEJCTILE_DIMENSION_MULTIPLIER,
        1.0 * PROEJCTILE_DIMENSION_MULTIPLIER,
    ));

    let mesh = meshes.add(Mesh::from(rect)).into();

    let material = materials.add(ColorMaterial::from(color));

    let material_mesh = MaterialMesh2dBundle {
        mesh,
        transform,
        material,
        ..default()
    };

    let projectile = Projectile {
        projectile_uuid,
        weapon_uuid: Default::default(),
    };

    let moveable = Moveable {
        entity_uuid: projectile_uuid,
        unhandled_updates: vec![],
        update_to_handle: None,
        translation_error: Vec3EMA::new(PROJECTILE_NETWORK_EMA_SMOOTHING_FACTOR),
        rotation_error: QuatEMA::new(PROJECTILE_NETWORK_EMA_SMOOTHING_FACTOR),
        linvel_error: Vec2EMA::new(PROJECTILE_NETWORK_EMA_SMOOTHING_FACTOR),
        angvel_error: EMA::new(PROJECTILE_NETWORK_EMA_SMOOTHING_FACTOR),
        update_rate_seconds: PROJECTILE_NETWORK_UPDATE_RATE_SECONDS,
        last_update_handled_at: 0.0,
        had_rollover: false,
    };

    let collideable = Collideable {
        entity_uuid: projectile_uuid,
    };

    let expireable = Expireable {
        entity_uuid: projectile_uuid,
        expires_at: time.elapsed_seconds_f64() + PROJECTILE_EXPIRY_SECONDS,
    };

    let mut parent: EntityCommands =
        commands.spawn((material_mesh, projectile, moveable, collideable, expireable));

    parent
        .insert(Dynamic)
        .insert(Sleeping::disabled())
        .insert(velocity);

    if game.role == "server" {
        parent
            .insert(Friction::coefficient(FRICTION_COEFFICIENT))
            .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
            .insert(Ccd::disabled())
            .insert(Collider::cuboid(
                1.0 * PROEJCTILE_DIMENSION_MULTIPLIER,
                1.0 * PROEJCTILE_DIMENSION_MULTIPLIER,
            ))
            .insert(ColliderMassProperties::Density(PROJECTILE_DENSITY))
            .insert(ActiveEvents::all());
    }
}

pub fn despawn_projectile(
    projectile_uuid: Uuid,
    projectile_query: &Query<(Entity, &Projectile)>,
    _time: Time,
    commands: &mut Commands,
) {
    for (entity, projectile) in projectile_query.iter() {
        if projectile.projectile_uuid != projectile_uuid {
            continue;
        }

        let entity_commands = commands.get_entity(entity);
        if entity_commands.is_none() {
            continue;
        }

        entity_commands.unwrap().despawn();
    }
}
