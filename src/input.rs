use bevy::{prelude::*, utils::HashMap};

use crate::movement::{Axis, Direction, PlayerMovementEvent};

pub struct Keybinds {
    keybinds: HashMap<KeyCode, InputEvent>,
}

impl Default for Keybinds {
    fn default() -> Self {
        let mut map: HashMap<KeyCode, InputEvent> = Default::default();
        map.insert(
            KeyCode::W,
            InputEvent::PlayerMovement(PlayerMovementEvent::new(Axis::Y, Direction::Positive)),
        );
        map.insert(
            KeyCode::A,
            InputEvent::PlayerMovement(PlayerMovementEvent::new(Axis::X, Direction::Negative)),
        );
        map.insert(
            KeyCode::S,
            InputEvent::PlayerMovement(PlayerMovementEvent::new(Axis::Y, Direction::Negative)),
        );
        map.insert(
            KeyCode::D,
            InputEvent::PlayerMovement(PlayerMovementEvent::new(Axis::X, Direction::Positive)),
        );
        Self { keybinds: map }
    }
}

impl Keybinds {
    pub fn get_action(&self, key: &KeyCode) -> Option<&InputEvent> {
        self.keybinds.get(key)
    }

    // pub fn bind_action(&mut self, input_event: InputEvent, key: KeyCode) -> Option<InputEvent> {
    //     self.keybinds.insert(key, input_event)
    // }
}

pub fn player_input_capture(
    keybinds: Res<Keybinds>,
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_event_writer: EventWriter<PlayerMovementEvent>,
) {
    for pressed_key in keyboard_input.get_pressed() {
        if let Some(input_event) = keybinds.get_action(pressed_key) {
            match input_event {
                InputEvent::PlayerMovement(move_event) => movement_event_writer.send(*move_event),
            }
        }
    }
}

pub enum InputEvent {
    PlayerMovement(PlayerMovementEvent),
}
