use bevy::prelude::{Entity, IntoSystem, Plugin, Query, Res, ResMut, Transform, With};

use crate::{collision::Collider, input::WorldCursor};

pub struct MouseOverPlugin;

impl Plugin for MouseOverPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource::<Option<MouseOverEntity>>(None)
            .add_system(set_mouse_over.system())
            .add_system(print_mouse_over.system());
    }
}

pub struct MouseOver(pub String);

pub struct MouseOverEntity(Entity);

pub fn set_mouse_over(
    mut mo_entity: ResMut<Option<MouseOverEntity>>,
    world_cursor: Res<WorldCursor>,
    query: Query<(Entity, &Transform, &Collider), With<MouseOver>>,
) {
    for (entity, transform, collider) in query.iter() {
        if collider.contains_point(&transform.translation.truncate(), world_cursor.position()) {
            *mo_entity = Some(MouseOverEntity(entity));
            return;
        }
    }
    *mo_entity = None;
}

pub fn print_mouse_over(
    mo_entity: Res<Option<MouseOverEntity>>,
    query: Query<(Entity, &MouseOver)>,
) {
    if mo_entity.is_changed() {
        if let Some(mo_entity) = &*mo_entity {
            for (entity, mouse_over) in query.iter() {
                if mo_entity.0 == entity {
                    println!("{}", mouse_over.0)
                }
            }
        }
    }
}
