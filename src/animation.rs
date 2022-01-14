use bevy::{
    core::Time,
    prelude::{Component, Query, Res},
    sprite::TextureAtlasSprite,
};

pub const MAX_FRAME_DURATION: f32 = 0.5;

pub type Animation = Vec<usize>;

#[derive(Component)]
pub struct AnimationState {
    pub duration_on_frame: f32,
    pub reverse: bool,
    sprite_index: usize,
    sprite_indexes: Animation,
}

impl AnimationState {
    pub fn new(sprite_indexes: Animation) -> Self {
        AnimationState {
            duration_on_frame: 0.0,
            reverse: false,
            sprite_index: 0,
            sprite_indexes,
        }
    }

    fn increment_animation(&mut self) -> usize {
        self.sprite_index = self.sprite_index.overflowing_add(1).0;
        self.duration_on_frame = 0.0;
        self.sprite_index % self.sprite_indexes.len()
    }

    fn decrement_animation(&mut self) -> usize {
        self.sprite_index = self.sprite_index.overflowing_sub(1).0;
        self.duration_on_frame = 0.0;
        self.sprite_index % self.sprite_indexes.len()
    }

    pub fn reset_animation(&mut self) {
        self.sprite_index = 0;
        self.duration_on_frame = 0.0;
    }

    pub fn set_sprite_indexes(&mut self, sprite_indexes: Vec<usize>) {
        self.sprite_indexes = sprite_indexes;
    }
}

pub fn animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite)>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.duration_on_frame += time.delta_seconds();
        if animation.duration_on_frame >= MAX_FRAME_DURATION {
            match animation.reverse {
                true => {
                    let idx = animation.decrement_animation();
                    sprite.index = animation.sprite_indexes[idx];
                }
                false => {
                    let idx = animation.increment_animation();
                    sprite.index = animation.sprite_indexes[idx];
                }
            }
        }
    }
}
