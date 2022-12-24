use bevy::prelude::{
    default, trace, BackgroundColor, Button, ButtonBundle, Changed, Commands, Component, EventReader,
    EventWriter, Input as KeyInput, Interaction, KeyCode, PositionType, Query, Res, ResMut, Resource, Size,
    Style, UiRect, Val, With,
};
use bevy_debug_text_overlay::screen_print;

use crate::base::helpers::serialize;
use crate::constants::{
    PLAYER_BACKWARD_KEY, PLAYER_FIRE_KEY, PLAYER_FORWARD_KEY, PLAYER_LEFT_KEY, PLAYER_RIGHT_KEY,
    UI_BUTTON_HEIGHT, UI_BUTTON_HOVERED, UI_BUTTON_NORMAL, UI_BUTTON_PRESSED, UI_BUTTON_WIDTH,
};
use crate::identity::entity::Local;
use crate::identity::player::Player;
use crate::types::event::Input;
use crate::types::network::{Container, OutgoingMessage};

#[derive(Debug, Clone, Component)]
pub struct ButtonRole {
    pub is_bottom_left: bool,
    pub is_bottom_right: bool,
}

#[derive(Debug, Clone, Resource)]
pub struct ButtonState {
    pub is_bottom_left_pressed: bool,
    pub is_bottom_right_pressed: bool,
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
                size: Size::new(Val::Px(UI_BUTTON_WIDTH), Val::Px(UI_BUTTON_HEIGHT)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(left),
                    right: Default::default(),
                    top: Val::Px(top),
                    bottom: Default::default(),
                },
                ..default()
            },
            background_color: UI_BUTTON_NORMAL.into(),
            ..default()
        },
        ButtonRole {
            is_bottom_left,
            is_bottom_right,
        },
    ));
}

pub fn handle_input_from_keyboard(
    player_query: Query<&Player, With<Local>>,
    keyboard_input: Res<KeyInput<KeyCode>>,
    mut input_event_writer: EventWriter<Input>,
) {
    let result = player_query.get_single();
    if result.is_err() {
        return;
    }

    let player = result.unwrap();

    assert_eq!(player.is_local_player, true);

    let inputs = vec![
        PLAYER_LEFT_KEY,
        PLAYER_RIGHT_KEY,
        PLAYER_FORWARD_KEY,
        PLAYER_BACKWARD_KEY,
        PLAYER_FIRE_KEY,
    ];

    let any_just_pressed = keyboard_input.any_just_pressed(inputs.clone());
    let any_just_released = keyboard_input.any_just_released(inputs.clone());

    if !(any_just_pressed || any_just_released) {
        return;
    }

    let is_left = keyboard_input.pressed(PLAYER_LEFT_KEY);
    let is_right = keyboard_input.pressed(PLAYER_RIGHT_KEY);
    let is_forward = keyboard_input.pressed(PLAYER_FORWARD_KEY);
    let is_backward = keyboard_input.pressed(PLAYER_BACKWARD_KEY);
    let is_firing = keyboard_input.pressed(PLAYER_FIRE_KEY);

    let input = Input {
        player_uuid: player.player_uuid,
        is_left,
        is_right,
        is_forward,
        is_backward,
        is_firing,
    };

    input_event_writer.send(input);
}

pub fn handle_input_from_button(
    player_query: Query<&Player, With<Local>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonRole),
        (Changed<Interaction>, With<Button>),
    >,
    mut button_state: ResMut<ButtonState>,
    mut input_event_writer: EventWriter<Input>,
) {
    let result = player_query.get_single();
    if result.is_err() {
        return;
    }

    let player = result.unwrap();

    assert_eq!(player.is_local_player, true);

    let mut was_input = false;

    for (interaction, mut color, button_role) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = UI_BUTTON_PRESSED.into();

                if button_role.is_bottom_left {
                    button_state.is_bottom_left_pressed = true;
                    was_input = true;
                }

                if button_role.is_bottom_right {
                    button_state.is_bottom_right_pressed = true;
                    was_input = true;
                }
            }
            Interaction::Hovered => {
                *color = UI_BUTTON_HOVERED.into();
            }
            Interaction::None => {
                *color = UI_BUTTON_NORMAL.into();

                if button_role.is_bottom_left {
                    button_state.is_bottom_left_pressed = false;
                    was_input = true;
                }

                if button_role.is_bottom_right {
                    button_state.is_bottom_right_pressed = false;
                    was_input = true;
                }
            }
        }
    }

    if !was_input {
        return;
    }

    let is_left = button_state.is_bottom_left_pressed && !button_state.is_bottom_right_pressed;
    let is_right = !button_state.is_bottom_left_pressed && button_state.is_bottom_right_pressed;
    let is_forward = button_state.is_bottom_left_pressed && button_state.is_bottom_right_pressed;
    let is_backward = false;
    let is_firing = false;

    let input = Input {
        player_uuid: player.player_uuid,
        is_left,
        is_right,
        is_forward,
        is_backward,
        is_firing,
    };

    input_event_writer.send(input);
}

pub fn handle_input_event(
    mut input_event_reader: EventReader<Input>,
    mut outgoing_message_event_writer: EventWriter<OutgoingMessage>,
) {
    for input in input_event_reader.iter() {
        let outgoing_message = OutgoingMessage {
            session_uuid: None,
            not_session_uuid: None,
            message: serialize(Container {
                message_type: "input".to_string(),
                join: None,
                spawn: None,
                input: Some(input.clone()),
                update: None,
                despawn: None,
                leave: None,
                collision: None,
            }),
        };

        outgoing_message_event_writer.send(outgoing_message);

        screen_print!(
            "left={:?}, right={:?}, forward={:?}, backward={:?}, fire={:?}",
            input.is_left,
            input.is_right,
            input.is_forward,
            input.is_backward,
            input.is_firing
        );
    }
}
