use bevy::{prelude::*, render::camera::Camera, utils::HashMap};

use crate::{
    camera::CameraTargetEvent,
    movement::{Axis, Direction, PlayerMovementEvent},
};

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
        map.insert(
            KeyCode::C,
            InputEvent::SwitchCameraTarget(CameraTargetEvent),
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
    mut camera_target_event_writer: EventWriter<CameraTargetEvent>,
) {
    for pressed_key in keyboard_input.get_pressed() {
        if let Some(input_event) = keybinds.get_action(pressed_key) {
            match input_event {
                InputEvent::PlayerMovement(move_event) => movement_event_writer.send(*move_event),
                InputEvent::SwitchCameraTarget(target_event) => {
                    camera_target_event_writer.send(*target_event)
                }
            }
        }
    }
}

pub fn update_world_cursor(
    mut world_cursor: ResMut<WorldCursor>,
    windows: Res<Windows>,
    query: Query<&Transform, With<Camera>>,
) {
    let main_camera_transform = query.single().expect("No camera found");

    if let Some(window) = windows.get_primary() {
        if let Some(position) = window.cursor_position() {
            let size = Vec2::new(window.width(), window.height());
            let p = position - size / 2.0;

            let pos_wld = main_camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            world_cursor.0.x = pos_wld.x;
            world_cursor.0.y = pos_wld.y;
        }
    }
}

pub enum InputEvent {
    PlayerMovement(PlayerMovementEvent),
    SwitchCameraTarget(CameraTargetEvent),
}

pub struct WorldCursor(Vec2);

impl Default for WorldCursor {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl WorldCursor {
    pub fn position(&self) -> &Vec2 {
        &self.0
    }
}

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WorldCursor::default())
            .add_system(update_world_cursor.system());
    }
}
