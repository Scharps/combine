use bevy::prelude::*;

const CAMERA_LOCK_RIGIDITY: f32 = 5.0;

#[derive(Clone, Copy)]
pub struct CameraTargetEvent;

pub struct MainCamera;

pub struct CameraTarget;

pub fn fixed_camera(
    mut query: QuerySet<(
        Query<&Transform, With<CameraTarget>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
) {
    let (tx, ty) = {
        if let Ok(target_transform) = query.q0().single() {
            (
                target_transform.translation.x,
                target_transform.translation.y,
            )
        } else {
            return;
        }
    };

    let mut camera_transform = query
        .q1_mut()
        .single_mut()
        .expect("No camera is tagged with MainCamera");
    camera_transform.translation.x = tx;
    camera_transform.translation.y = ty;
}

pub fn loose_camera(
    time: Res<Time>,
    mut query: QuerySet<(
        Query<&Transform, With<CameraTarget>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
) {
    let (tx, ty) = {
        if let Ok(target_transform) = query.q0().single() {
            (
                target_transform.translation.x,
                target_transform.translation.y,
            )
        } else {
            return;
        }
    };

    let mut camera_transform = query
        .q1_mut()
        .single_mut()
        .expect("No camera is tagged with MainCamera");
    camera_transform.translation.x -=
        (camera_transform.translation.x - tx) * time.delta_seconds() * CAMERA_LOCK_RIGIDITY;
    camera_transform.translation.y -=
        (camera_transform.translation.y - ty) * time.delta_seconds() * CAMERA_LOCK_RIGIDITY;
}
