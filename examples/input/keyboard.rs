use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(keyboard_input_system.system())
        .run();
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    let pressed_keys = keyboard_input.get_pressed();
    for key in pressed_keys {
        match key {
            KeyCode::W => println!("W key is pressed"),
            KeyCode::A => println!("A key is pressed"),
            KeyCode::S => println!("S key is pressed"),
            KeyCode::D => println!("D key is pressed"),
            _ => (),
        }
    }
}
