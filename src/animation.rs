use bevy::{
    core::Time,
    prelude::{Handle, Query, Res, Texture},
    sprite::{ColorMaterial, Sprite, TextureAtlasSprite},
};

use crate::movement::Direction;

pub const MAX_FRAME_DURATION: f32 = 0.066;

pub enum Animation {
    Walking(AnimationState),
    Standing(AnimationState),
}

pub struct AnimationState {
    pub duration_on_frame: f32,
    sprite_index: usize,
    sprite_indexes: Vec<usize>,
}

impl AnimationState {
    pub fn new(sprite_indexes: Vec<usize>) -> Self {
        AnimationState {
            duration_on_frame: 0.0,
            sprite_index: 0,
            sprite_indexes,
        }
    }

    pub fn increment_animation(&mut self) -> usize {
        self.sprite_index = self.sprite_index.overflowing_add(1).0;
        self.sprite_index % self.sprite_indexes.len()
    }

    pub fn decrement_animation(&mut self) -> usize {
        self.sprite_index = self.sprite_index.overflowing_sub(1).0;
        self.sprite_index % self.sprite_indexes.len()
    }

    pub fn reset_animation(&mut self) {
        self.sprite_index = 0;
        self.duration_on_frame = 0.0;
    }
}

pub fn animation(
    time: Res<Time>,
    mut query: Query<(
        &mut Animation,
        &TextureAtlasSprite,
        &mut Handle<ColorMaterial>,
    )>,
) {
    for (mut animation, sprite, mut material) in query.iter_mut() {
        match &mut *animation {
            Animation::Walking(animation_state) => {}
            Animation::Standing(animation_state) => todo!(),
        }
    }
}
