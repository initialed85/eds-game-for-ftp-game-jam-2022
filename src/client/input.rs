use bevy::prelude::{EventReader, EventWriter, Input as KeyInput, KeyCode, Query, Res, With};
use bevy_debug_text_overlay::screen_print;

use crate::base::helpers::serialize;
use crate::constants::{
    PLAYER_BACKWARD_KEY, PLAYER_FIRE_KEY, PLAYER_FORWARD_KEY, PLAYER_LEFT_KEY, PLAYER_RIGHT_KEY,
};
use crate::identity::entity::Local;
use crate::identity::player::Player;
use crate::types::event::Input;
use crate::types::network::{Container, OutgoingMessage};

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
