use bevy::prelude::{Commands, ResMut};

// use crate::client::input::spawn_button;
// use crate::constants::{
//     UI_BUTTON_BOTTOM_LEFT_LEFT, UI_BUTTON_BOTTOM_LEFT_TOP, UI_BUTTON_BOTTOM_RIGHT_LEFT,
//     UI_BUTTON_BOTTOM_RIGHT_TOP,
// };
use crate::identity::game::Game;

pub fn handle_setup(mut game: ResMut<Game>, mut _commands: Commands) {
    game.role = "client".to_string();

    // TODO
    // spawn_button(
    //     &mut commands,
    //     UI_BUTTON_BOTTOM_LEFT_LEFT,
    //     UI_BUTTON_BOTTOM_LEFT_TOP,
    //     true,
    //     false,
    // );

    // TODO
    // spawn_button(
    //     &mut commands,
    //     UI_BUTTON_BOTTOM_RIGHT_LEFT,
    //     UI_BUTTON_BOTTOM_RIGHT_TOP,
    //     false,
    //     true,
    // );
}
