use bevy::{prelude::*, render::camera::Camera};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_orth_camera.system())
        .add_system(camera_pan.system())
        .add_system(print_camera_location.system())
        .run()
}

fn add_orth_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}

fn print_camera_location(query: Query<&Transform, With<Camera>>) {
    for transform in query.iter() {
        println!("{:?}", transform);
    }
}

fn camera_pan(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let pressed_keys = keyboard_input.get_pressed();
    let mut x = 0.;
    let mut y = 0.;

    for key in pressed_keys {
        match key {
            KeyCode::W => y += 1.,
            KeyCode::A => x -= 1.,
            KeyCode::S => y -= 1.,
            KeyCode::D => x += 1.,
            _ => (),
        }
    }

    for mut transform in camera.iter_mut() {
        transform.translation += Vec3::new(x * time.delta_seconds(), y * time.delta_seconds(), 0.);
    }
}
