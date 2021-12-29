mod input;
mod movement;
mod prelude;
use bevy::prelude::*;
use input::Keybinds;
use movement::{PlayerMovementEvent, Speed};
use prelude::Player;

fn main() {
    App::build()
        .add_startup_system(startup.system())
        .add_plugins(DefaultPlugins)
        .add_event::<PlayerMovementEvent>()
        .insert_resource(Keybinds::default())
        .add_system(input::player_input_capture.system())
        .add_system(movement::player_movement.system())
        .run();
}

fn startup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Transform::default())
        .insert(Player {})
        .insert(Speed(5.0));
}
