pub fn update_weapon(
    mut query: QuerySet<(
        Query<(&mut Sprite, &mut Transform), With<Weapon>>,
        Query<&Transform, With<Player>>,
    )>,
    world_cursor: Res<WorldCursor>,
) {
    let player_position = { query.q1().single().expect("No player found").translation };

    // Update weapon position
    let (mut weapon_sprite, mut weapon_transform) =
        query.q0_mut().single_mut().expect("No weapon found");

    weapon_transform.translation = player_position;
    if world_cursor.is_changed() {
        let direction_vector = *world_cursor.position() - player_position.truncate();
        if direction_vector.x > 0.0 {
            weapon_sprite.flip_y = false;
            weapon_transform.rotation =
                Quat::from_rotation_z(-direction_vector.angle_between(Vec2::X));
        } else {
            weapon_sprite.flip_y = true;
            weapon_transform.rotation =
                Quat::from_rotation_z(-direction_vector.angle_between(Vec2::X));
        }
    }
}
