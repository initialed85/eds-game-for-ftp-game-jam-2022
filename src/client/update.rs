use bevy::prelude::{EventReader, Query};

use crate::behaviour::moveable::Moveable;
use crate::types::event::UpdateEvent;

pub fn handle_update_event(
    mut update_event_reader: EventReader<UpdateEvent>,
    mut moveable_query: Query<&mut Moveable>,
) {
    for update in update_event_reader.read() {
        for mut moveable in moveable_query.iter_mut() {
            if update.entity_uuid != moveable.entity_uuid {
                continue;
            }

            moveable.unhandled_updates.insert(0, update.clone());
        }
    }
}
