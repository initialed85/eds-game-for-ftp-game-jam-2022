use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Mesh, ResMut, Time};

use crate::identity::player::spawn_player;
use crate::types::event::Spawn;

pub fn spawn_entity(
    spawn: Spawn,
    time: Time,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    if spawn.entity_type == "player" {
        spawn_player(
            spawn.entity_uuid,
            spawn.color.unwrap(),
            spawn.transform.unwrap().to_transform(),
            spawn.velocity.unwrap().to_velocity(),
            time,
            meshes,
            materials,
            commands,
        )
    } else {
        panic!(
            "unsupported spawn.entity_type={:?} for spawn={:?}",
            spawn.entity_type, spawn
        );
    }
}
