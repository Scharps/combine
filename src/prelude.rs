use bevy::prelude::SystemLabel;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Input,
    Movement,
}
