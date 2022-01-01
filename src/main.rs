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

fn startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn().insert_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(32.0, 32.0)),
        ..Default::default()
    });
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .insert(Transform::default())
        .insert(Player {})
        .insert(Speed(500.0));

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
