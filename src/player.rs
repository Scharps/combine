use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};

use crate::{
    animation::{self, AnimationState},
    camera::CameraTarget,
    collision::Collider,
    input::{player_input_capture, Keybinds},
    interaction::MouseOver,
    movement::{player_face_cursor, player_movement, PlayerMovementEvent, Speed},
    prelude::Systems,
};

pub struct Player;
pub struct Weapon;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(Keybinds::default())
            .init_resource::<SpriteHandles>()
            .add_event::<PlayerMovementEvent>()
            .add_state(AppState::LoadResources)
            .add_system_set(
                SystemSet::on_enter(AppState::LoadResources).with_system(load_textures.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::LoadResources).with_system(check_textures.system()),
            )
            .add_system_set(
                SystemSet::after(
                    SystemSet::on_enter(AppState::Finished)
                        .with_system(set_up_player_assets.system()),
                    Systems::Movement,
                )
                .with_system(
                    player_movement
                        .system()
                        .after(Systems::Input)
                        .label(Systems::Movement),
                ),
            )
            .add_system(player_input_capture.system().label(Systems::Input))
            .add_system(player_face_cursor.system());
    }
}

fn set_up_player_assets(
    mut materials: ResMut<Assets<ColorMaterial>>,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut commands: Commands,
) {
    // let weapon = asset_server.load("textures/sprites/weapons/shooty.png");

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
}

fn set_up_player(mut commands: Commands) {

    // commands
    //     .spawn()
    //     .insert_bundle(SpriteBundle {
    //         material: materials.add(weapon.into()),
    //         transform: Transform::from_xyz(0.0, 0.0, 1.0),
    //         sprite: Sprite::new(Vec2::new(150.0, 150.0)),
    //         ..Default::default()
    //     })
    //     .insert(Weapon);
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    LoadResources,
    CreateEntities,
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
