use bevy::prelude::{trace, EventReader, Res, ResMut, Time};

use crate::identity::game::Game;
use crate::types::event::Join;

pub fn base_handle_join_event(
    mut join_event_reader: EventReader<Join>,
    mut game: ResMut<Game>,
    time: Res<Time>,
) {
    assert_ne!(game.role, "base");

    for join in join_event_reader.iter() {
        game.player_uuids.insert(join.player_uuid);

        if game.role != "client" {
            continue;
        }

        if join.is_for_local_player {
            game.local_player_uuid = Some(join.player_uuid);
            game.server_time_at_join = join.server_time_at_join;
            game.client_time_at_join = time.elapsed_seconds_f64();
        }

        trace!("base_handle_join_event; game={:?}", game);
    }
}
