use bevy::math::{Quat, Vec2};
use bevy::prelude::{Color, EventReader, EventWriter, Query, Transform};
use bevy_rapier2d::dynamics::Velocity;
use rand::{thread_rng, Rng};

use crate::base::helpers::serialize;
use crate::constants::{BOUNDS, DEGREES_MAX};
use crate::identity::player::Player;
use crate::types::event::{JoinEvent, SerializableTransform, SerializableVelocity, SpawnEvent};
use crate::types::network::{Container, OutgoingMessageEvent};

pub fn handle_join_event(
    mut join_event_reader: EventReader<JoinEvent>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessageEvent>,
    player_query: Query<(&Player, &Transform, &Velocity)>,
    mut spawn_event_writer: EventWriter<SpawnEvent>,
) {
    for join_event in join_event_reader.read() {
        // tell the joiner about itself
        outgoing_message_event_writer.send(OutgoingMessageEvent {
            session_uuid: Some(join_event.player_uuid),
            not_session_uuid: None,
            message: serialize(Container {
                message_type: "join".to_string(),
                join: Some(join_event.clone()),
                spawn: None,
                input: None,
                update: None,
                despawn: None,
                leave: None,
                collision: None,
            }),
        });

        // tell everyone else about the joiner
        let mut join_event_for_everyone_else = join_event.clone();
        join_event_for_everyone_else.is_for_local_player = false;
        outgoing_message_event_writer.send(OutgoingMessageEvent {
            session_uuid: None,
            not_session_uuid: Some(join_event.player_uuid),
            message: serialize(Container {
                message_type: "join".to_string(),
                join: Some(join_event_for_everyone_else.clone()),
                spawn: None,
                input: None,
                update: None,
                despawn: None,
                leave: None,
                collision: None,
            }),
        });

        let mut rng = thread_rng();

        let translation = Vec2::from((
            rng.gen::<f32>() * BOUNDS.x - (BOUNDS.x / 2.0),
            rng.gen::<f32>() * BOUNDS.y - (BOUNDS.y / 2.0),
        ));

        let translation = translation.extend(
            rng.gen::<f32>() / 2.0, // all players between 0.0 and 0.5 as Z index
        );

        let rotation =
            Quat::from_rotation_z(f32::to_radians(DEGREES_MAX * thread_rng().gen::<f32>()));

        // TODO: something to avoid spawn position collision
        let transform = Transform::from_translation(translation).with_rotation(rotation);

        let velocity = Velocity::zero();
        let color = Color::srgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());

        // tell everyone else to spawn the joiner
        spawn_event_writer.send(SpawnEvent {
            entity_uuid: join_event.player_uuid,
            entity_type: "player".to_string(),
            transform: Some(SerializableTransform::from_transform(transform)),
            velocity: Some(SerializableVelocity::from_velocity(velocity)),
            color: Some(color),
        });

        // tell everyone to ensure everyone is spawned
        for (player, transform, velocity) in player_query.iter() {
            spawn_event_writer.send(SpawnEvent {
                entity_uuid: player.player_uuid,
                entity_type: "player".to_string(),
                transform: Some(SerializableTransform::from_transform(*transform)),
                velocity: Some(SerializableVelocity::from_velocity(*velocity)),
                color: Some(player.color),
            });
        }
    }
}
