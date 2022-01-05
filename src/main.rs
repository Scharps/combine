mod animation;
mod camera;
mod collision;
mod input;
mod interaction;
mod movement;
mod prelude;
use animation::AnimationState;
use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};
use camera::{CameraTarget, CameraTargetEvent, MainCamera};
use collision::Collider;
use input::WorldCursorPlugin;
use interaction::{MouseOver, MouseOverPlugin};
use movement::{update_weapon, Speed};
use prelude::{Player, PlayerPlugin, Weapon};

fn main() {
    App::build()
        .add_startup_system(startup.system())
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_event::<CameraTargetEvent>()
        .add_system(camera::loose_camera.system().after("movement"))
        .add_plugin(WorldCursorPlugin)
        .add_plugin(MouseOverPlugin)
        .add_system(update_weapon.system().after("movement"))
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures.system()))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures.system()))
        .add_system_set(
            SystemSet::on_enter(AppState::Finished)
                .with_system(set_up_player.system())
                .label("player_loaded"),
        )
        .run();
}

fn set_up_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    let weapon = asset_server.load("textures/sprites/weapons/shooty.png");

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let run1_handle = asset_server.get_handle("textures/sprites/chars/player/run1.png");
    let run2_handle = asset_server.get_handle("textures/sprites/chars/player/run2.png");
    let run3_handle = asset_server.get_handle("textures/sprites/chars/player/run3.png");
    let run4_handle = asset_server.get_handle("textures/sprites/chars/player/run4.png");
    let run5_handle = asset_server.get_handle("textures/sprites/chars/player/run5.png");
    let run6_handle = asset_server.get_handle("textures/sprites/chars/player/run6.png");
    let run1_index = texture_atlas.get_texture_index(&run1_handle).unwrap();
    let run2_index = texture_atlas.get_texture_index(&run2_handle).unwrap();
    let run3_index = texture_atlas.get_texture_index(&run3_handle).unwrap();
    let run4_index = texture_atlas.get_texture_index(&run4_handle).unwrap();
    let run5_index = texture_atlas.get_texture_index(&run5_handle).unwrap();
    let run6_index = texture_atlas.get_texture_index(&run6_handle).unwrap();

    let sprite_indexes = vec![
        run1_index, run2_index, run3_index, run4_index, run5_index, run6_index,
    ];

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: TextureAtlasSprite::new(run2_index as u32),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Player {})
        .insert(Speed(500.0))
        .insert(CameraTarget)
        .insert(Collider::Rectangle(Vec2::new(32.0, 32.0)))
        .insert(MouseOver("Player".to_string()))
        .insert(animation::Animation::Walking(AnimationState::new(
            sprite_indexes,
        )));

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
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    // Create Texture Atlas
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server
        .load_folder("textures/sprites")
        .expect("Unable to load sprite folder");
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}
