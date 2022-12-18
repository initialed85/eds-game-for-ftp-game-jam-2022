use bevy::prelude::ResMut;

use crate::identity::game::Game;

pub fn handle_setup(mut game: ResMut<Game>) {
    game.role = "server".to_string();
}
