use std::borrow::BorrowMut;

use bevy::math::Vec3;
use bevy::prelude::{GlobalTransform, Query, Transform};

use crate::constants::BOUNDS;
use crate::player::Player;
use crate::projectile::Projectile;

fn handle_map_rollover(half_width: f32, half_height: f32, transform: &mut Transform, global_transform: &GlobalTransform) {
    let extents: Vec3 = Vec3::from((BOUNDS / 2.0, 0.0));

    let right = extents.x + half_width;
    let left = -extents.x - half_width;
    let up = extents.y + half_height;
    let down = -extents.y - half_height;

    if global_transform.translation().x > right {
        transform.translation.x = left
    }

    if global_transform.translation().x < left {
        transform.translation.x = right
    }

    if global_transform.translation().y > up {
        transform.translation.y = down
    }

    if global_transform.translation().y < down {
        transform.translation.y = up
    }
}

pub fn handle_player_map_rollover(mut query: Query<(&Player, &mut Transform, &GlobalTransform)>) {
    for (player, mut transform, global_transform) in query.iter_mut() {
        handle_map_rollover(player.size.x / 2.0, player.size.y / 2.0, transform.borrow_mut(), global_transform)
    }
}

pub fn handle_projectile_map_rollover(mut query: Query<(&Projectile, &mut Transform, &GlobalTransform)>) {
    for (projectile, mut transform, global_transform) in query.iter_mut() {
        handle_map_rollover(
            projectile.size.x / 2.0,
            projectile.size.y / 2.0,
            transform.borrow_mut(),
            global_transform,
        )
    }
}
