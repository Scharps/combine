use bevy::prelude::{IntoSystem, ParallelSystemDescriptorCoercion, Plugin};

use crate::{
    input::{player_input_capture, Keybinds},
    movement::{player_face_cursor, player_movement, PlayerMovementEvent},
};

pub struct Player;
pub struct Weapon;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(Keybinds::default())
            .add_event::<PlayerMovementEvent>()
            .add_system(player_input_capture.system().label("input"))
            .add_system(player_movement.system().after("input").label("movement"))
            .add_system(player_face_cursor.system());
    }
}
