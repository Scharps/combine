use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};

use crate::{
    animation::{self, Animation, AnimationState},
    camera::CameraTarget,
    collision::Collider,
    input::{player_input_capture, Keybinds},
    interaction::MouseOver,
    movement::{player_face_cursor, player_movement, PlayerMovementEvent, Speed},
    prelude::Systems,
};

pub type RunningAnimation = Animation;

#[derive(Component)]
pub struct Player;
pub struct Weapon {
    pub sprite: TextureAtlasSprite,
    pub offset: (f32, f32),
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
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
                SystemSet::on_enter(AppState::Finished)
                    .with_system(set_up_player_assets.system())
                    .before(Systems::Movement),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Finished)
                    .with_system(
                        player_movement
                            .system()
                            .after(Systems::Input)
                            .label(Systems::Movement),
                    )
                    .with_system(
                        player_face_cursor
                            .system()
                            .after(Systems::Input)
                            .label(Systems::Movement),
                    )
                    .with_system(animation::animation.system().after(Systems::Movement)),
            )
            .add_system(player_input_capture.system().label(Systems::Input));
    }
}

fn set_up_player_assets(
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let sprite_indexes: Animation = (1..=6)
        .map(|n| {
            asset_server.get_handle(format!(
                "textures/sprites/chars/player/running/run{}.png",
                n
            ))
        })
        .map(|h| texture_atlas.get_texture_index(&h).unwrap())
        .collect();

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: TextureAtlasSprite::new(sprite_indexes[1]),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(500.0))
        .insert(CameraTarget)
        .insert(Collider::Rectangle(Vec2::new(32.0, 32.0)))
        .insert(MouseOver("Player".to_string()))
        .insert(AnimationState::new(sprite_indexes));
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    LoadResources,
    Finished,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server
        .load_folder("textures/sprites/chars/player/running")
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
