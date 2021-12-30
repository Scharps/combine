mod input;
mod movement;
mod prelude;
use bevy::prelude::*;
use movement::Speed;
use prelude::{Player, PlayerPlugin};

fn main() {
    App::build()
        .add_startup_system(startup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

fn startup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Transform::default())
        .insert(Player {})
        .insert(Speed(5.0));
}
