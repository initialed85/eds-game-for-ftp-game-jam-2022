use bevy::prelude::{EventReader, ResMut};

use crate::identity::game::Game;
use crate::types::event::LeaveEvent;

pub fn base_handle_leave_event(
    mut leave_event_reader: EventReader<LeaveEvent>,
    mut game: ResMut<Game>,
) {
    for leave in leave_event_reader.read() {
        game.player_uuids.remove(&leave.player_uuid);
    }
}
