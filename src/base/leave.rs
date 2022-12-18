use bevy::prelude::{EventReader, ResMut};

use crate::identity::game::Game;
use crate::types::event::Leave;

pub fn base_handle_leave_event(mut leave_event_reader: EventReader<Leave>, mut game: ResMut<Game>) {
    for leave in leave_event_reader.iter() {
        game.player_uuids.remove(&leave.player_uuid);
    }
}
