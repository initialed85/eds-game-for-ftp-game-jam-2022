use bevy::math::Vec3;
use bevy::prelude::{
    Color, Component, Event, EventReader, EventWriter, Query, Res, Time, Transform,
};
use bevy_rapier2d::prelude::Velocity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{
    MATERIAL_SCALE, PLAYER_HEIGHT_MULTIPLIER, PROJECTILE_LINEAR_VELOCITY, WEAPON_FIRE_RATE_SECONDS,
    ZERO,
};
use crate::types::event::{SerializableTransform, SerializableVelocity, SpawnEvent};

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct FireEvent {
    pub weapon_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Weaponized {
    pub weapon_uuid: Uuid,
    pub last_fired_at: f64,
}

impl Weaponized {
    pub fn fire(self: &Weaponized, fire_event_writer: &mut EventWriter<'_, FireEvent>) {
        fire_event_writer.send(FireEvent {
            weapon_uuid: self.weapon_uuid,
        });
    }
}

pub fn handle_fire_event(
    mut fire_event_reader: EventReader<FireEvent>,
    time: Res<Time>,
    mut weapon_query: Query<(&mut Weaponized, &Transform)>,
    mut spawn_event_writer: EventWriter<SpawnEvent>,
) {
    for fire_event in fire_event_reader.read() {
        for (mut weapon, transform) in weapon_query.iter_mut() {
            if weapon.weapon_uuid != fire_event.weapon_uuid {
                continue;
            }

            if time.elapsed_seconds_f64() - weapon.last_fired_at < WEAPON_FIRE_RATE_SECONDS {
                continue;
            }

            let projectile_offset = Vec3::new(
                0.0,
                ((MATERIAL_SCALE / 2.0) * PLAYER_HEIGHT_MULTIPLIER) + 10.0,
                0.0,
            );

            let rotated_projected_offset = transform.rotation.mul_vec3(projectile_offset);
            let mut projectile_transform = *transform;
            projectile_transform.translation += rotated_projected_offset;

            let mut projectile_velocity = Velocity::default();
            projectile_velocity.linvel = transform
                .rotation
                .mul_vec3(Vec3::new(ZERO, PROJECTILE_LINEAR_VELOCITY, 0.0))
                .truncate();

            spawn_event_writer.send(SpawnEvent {
                entity_uuid: Uuid::new_v4(),
                entity_type: "projectile".to_string(),
                transform: Some(SerializableTransform::from_transform(projectile_transform)),
                velocity: Some(SerializableVelocity::from_velocity(projectile_velocity)),
                color: Some(Color::srgb(0.0, 1.0, 1.0)),
            });

            weapon.last_fired_at = time.elapsed_seconds_f64();
        }
    }
}
