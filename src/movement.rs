use bevy::prelude::*;

use crate::prelude::Player;

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
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut player_transform, speed) = query.single_mut().expect("There is no player");
    let mut v = Vec3::default();
    for movement_event in event_reader.iter() {
        match movement_event.0.axis {
            Axis::X => match movement_event.0.direction {
                Direction::Positive => v.x += 1.0,
                Direction::Negative => v.x -= 1.0,
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
