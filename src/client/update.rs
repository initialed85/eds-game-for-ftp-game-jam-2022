use bevy::prelude::{EventReader, Query};

use crate::behaviour::moveable::Moveable;
use crate::types::event::Update;

pub fn handle_update_event(
    mut update_event_reader: EventReader<Update>,
    mut moveable_query: Query<&mut Moveable>,
) {
    for update in update_event_reader.iter() {
        for mut moveable in moveable_query.iter_mut() {
            if update.entity_uuid != moveable.entity_uuid {
                continue;
            }

            moveable.unhandled_updates.insert(0, update.clone());
        }
    }
}
