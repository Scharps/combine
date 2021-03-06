use bevy::prelude::*;

use crate::{
    animation::AnimationState,
    input::WorldCursor,
    player::{Player, Weapon},
};

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Clone, Copy)]
pub struct MovementEvent {
    axis: Axis,
    direction: Direction,
}

impl MovementEvent {
    pub fn new(axis: Axis, direction: Direction) -> Self {
        Self { axis, direction }
    }
}

pub fn player_movement(
    time: Res<Time>,
    mut event_reader: EventReader<PlayerMovementEvent>,
    mut query: Query<
        (
            &mut Transform,
            &Speed,
            &mut AnimationState,
            &TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    let (mut player_transform, speed, mut animation_state, sprite) = query.single_mut();
    let mut v = Vec3::default();
    for movement_event in event_reader.iter() {
        match movement_event.0.axis {
            Axis::X => match movement_event.0.direction {
                Direction::Positive => {
                    v.x += 1.0;
                    if sprite.flip_y {
                        animation_state.reverse = true;
                    } else {
                        animation_state.reverse = false;
                    }
                }
                Direction::Negative => {
                    v.x -= 1.0;
                    if sprite.flip_y {
                        animation_state.reverse = false;
                    } else {
                        animation_state.reverse = true;
                    }
                }
            },
            Axis::Y => match movement_event.0.direction {
                Direction::Positive => v.y += 1.0,
                Direction::Negative => v.y -= 1.0,
            },
        }
    }
    player_transform.translation += v.normalize_or_zero() * speed.0 * time.delta_seconds();
}

#[derive(Clone, Copy)]
pub struct PlayerMovementEvent(MovementEvent);

impl PlayerMovementEvent {
    pub fn new(axis: Axis, direction: Direction) -> Self {
        PlayerMovementEvent(MovementEvent::new(axis, direction))
    }
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Positive,
    Negative,
}

pub fn player_face_cursor(
    mut query: Query<(&mut TextureAtlasSprite, &Transform), With<Player>>,
    world_cursor: Res<WorldCursor>,
) {
    if world_cursor.is_changed() {
        let (mut sprite, transform) = query.single_mut();
        let direction_vector = *world_cursor.position() - transform.translation.truncate();
        if direction_vector.x > 0.0 {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}
