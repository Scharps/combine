use bevy::{prelude::*, render::camera::Camera};

use crate::prelude::Player;

const CAMERA_LOCK_RIGIDITY: f32 = 5.0;

pub fn camera_lock(
    time: Res<Time>,
    mut query: QuerySet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let (px, py) = {
        let player_transform = query.q0_mut().single_mut().expect("No player in game");
        (
            player_transform.translation.x,
            player_transform.translation.y,
        )
    };

    let mut camera_transform = query.q1_mut().single_mut().expect("No camera found");
    camera_transform.translation.x -=
        (camera_transform.translation.x - px) * time.delta_seconds() * CAMERA_LOCK_RIGIDITY;
    camera_transform.translation.y -=
        (camera_transform.translation.y - py) * time.delta_seconds() * CAMERA_LOCK_RIGIDITY;
}
