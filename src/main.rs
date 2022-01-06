mod animation;
mod camera;
mod collision;
mod input;
mod interaction;
mod movement;
mod player;
mod prelude;
use bevy::prelude::*;
use camera::{CameraTargetEvent, MainCamera};
use collision::Collider;
use input::WorldCursorPlugin;
use interaction::{MouseOver, MouseOverPlugin};
use movement::update_weapon;
use player::PlayerPlugin;
use prelude::Systems;

fn main() {
    App::build()
        .add_startup_system(startup.system())
        .add_startup_system(add_main_camera.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_event::<CameraTargetEvent>()
        .add_system(camera::loose_camera.system().after(Systems::Movement))
        .add_plugin(WorldCursorPlugin)
        .add_plugin(MouseOverPlugin)
        .add_system(update_weapon.system().after(Systems::Movement))
        .run();
}

fn startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let sprite = asset_server.load("textures/sprites/chars/enemies/hogger.png");

    // Other entities
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(sprite.clone().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(150.0, 150.0)),
            ..Default::default()
        })
        .insert(Collider::Rectangle(Vec2::new(150.0, 150.0)))
        .insert(MouseOver("Other".to_string()));
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(sprite.into()),
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(150.0, 150.0)),
            ..Default::default()
        })
        .insert(Collider::Rectangle(Vec2::new(150.0, 150.0)))
        .insert(MouseOver("Other".to_string()));

    // Camera
}

fn add_main_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
