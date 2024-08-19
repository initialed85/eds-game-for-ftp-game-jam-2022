use bevy::app::{Startup, Update};
use bevy::log::trace;
use bevy::prelude::{
    default, BackgroundColor, Button, ButtonBundle, Changed, Color, Commands, Component,
    Interaction, PositionType, Query, ResMut, Style, Val, With,
};
use wasm_bindgen::prelude::wasm_bindgen;

use eds_game_for_ftp_game_jam_2022::base::app::get_base_app;
use eds_game_for_ftp_game_jam_2022::constants::{
    UI_BUTTON_BOTTOM_LEFT_LEFT, UI_BUTTON_BOTTOM_LEFT_TOP, UI_BUTTON_BOTTOM_RIGHT_LEFT,
    UI_BUTTON_BOTTOM_RIGHT_TOP, UI_BUTTON_HEIGHT, UI_BUTTON_WIDTH,
};
use eds_game_for_ftp_game_jam_2022::identity::game::Game;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Clone, Component)]
pub struct ButtonRole {
    pub is_bottom_left: bool,
    pub is_bottom_right: bool,
}

pub fn spawn_button(
    commands: &mut Commands,
    left: f32,
    top: f32,
    is_bottom_left: bool,
    is_bottom_right: bool,
) {
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(UI_BUTTON_WIDTH),
                height: Val::Px(UI_BUTTON_HEIGHT),
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                right: Default::default(),
                top: Val::Px(top),
                bottom: Default::default(),
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        ButtonRole {
            is_bottom_left,
            is_bottom_right,
        },
    ));
}

fn handle_setup(mut game: ResMut<Game>, mut commands: Commands) {
    game.role = "client".to_string();

    spawn_button(
        &mut commands,
        UI_BUTTON_BOTTOM_LEFT_LEFT,
        UI_BUTTON_BOTTOM_LEFT_TOP,
        true,
        false,
    );

    spawn_button(
        &mut commands,
        UI_BUTTON_BOTTOM_RIGHT_LEFT,
        UI_BUTTON_BOTTOM_RIGHT_TOP,
        false,
        true,
    );
}

#[warn(clippy::type_complexity)]
fn handle_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonRole),
        (Changed<Interaction>, With<Button>),
    >,
) {
    trace!("hi?");

    for (interaction, mut color, button_role) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                trace!("click; button_role={:?}", button_role);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                trace!("hover; button_role={:?}", button_role);
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                trace!("normal; button_role={:?}", button_role);
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = get_base_app();

    app.add_systems(Startup, handle_setup);
    app.add_systems(Update, handle_button);

    app.run();
}

pub fn main() {
    // wasm_bindgen invokes run() for us
    // run();
}
