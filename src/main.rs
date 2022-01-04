mod animation;
mod camera;
mod collision;
mod input;
mod interaction;
mod movement;
mod prelude;
use bevy::prelude::*;
use camera::{CameraTarget, CameraTargetEvent, MainCamera};
use collision::Collider;
use input::WorldCursorPlugin;
use interaction::{MouseOver, MouseOverPlugin};
use movement::{update_weapon, Speed};
use prelude::{Player, PlayerPlugin, Weapon};

fn main() {
    App::build()
        .add_startup_system(startup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(set_up_player.system())
        .add_event::<CameraTargetEvent>()
        .add_system(camera::loose_camera.system().after("movement"))
        .add_plugin(WorldCursorPlugin)
        .add_plugin(MouseOverPlugin)
        .add_system(update_weapon.system().after("movement"))
        .run();
}

fn set_up_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let sprite = asset_server.load("sprites/runedit_wohands.png");
    let weapon = asset_server.load("sprites/shooty.png");

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(sprite.into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(150.0, 150.0)),
            ..Default::default()
        })
        .insert(Player {})
        .insert(Speed(500.0))
        .insert(CameraTarget)
        .insert(Collider::Rectangle(Vec2::new(32.0, 32.0)))
        .insert(MouseOver("Player".to_string()));

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(weapon.into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(150.0, 150.0)),
            ..Default::default()
        })
        .insert(Weapon);
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
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .insert(Collider::Rectangle(Vec2::new(32.0, 32.0)))
        .insert(MouseOver("Other".to_string()));

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
