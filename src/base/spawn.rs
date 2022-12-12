use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, EventReader, Mesh, Res, ResMut, Time};

use crate::identity::entity::spawn_entity;
use crate::types::event::Spawn;

pub fn base_handle_spawn_event(
    mut spawn_event_reader: EventReader<Spawn>,
    time: Res<Time>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_event_reader.iter() {
        spawn_entity(
            spawn_event.clone(),
            time.clone(),
            &mut meshes,
            &mut materials,
            &mut commands,
        );
    }
}
