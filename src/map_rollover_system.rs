use std::borrow::BorrowMut;

use bevy::math::Vec3;
use bevy::prelude::{GlobalTransform, Query, Transform};

use crate::constants::{BOUNDS, HALF};
use crate::particle::Particle;
use crate::player::Player;
use crate::projectile::Projectile;

fn handle_map_rollover(transform: &mut Transform, global_transform: &GlobalTransform) {
    let extents: Vec3 = Vec3::from((BOUNDS * HALF, 0.0));

    let right = extents.x;
    let left = -extents.x;
    let up = extents.y;
    let down = -extents.y;

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
    for (_, mut transform, global_transform) in query.iter_mut() {
        handle_map_rollover(transform.borrow_mut(), global_transform)
    }
}

pub fn handle_projectile_map_rollover(mut query: Query<(&Projectile, &mut Transform, &GlobalTransform)>) {
    for (item, mut transform, global_transform) in query.iter_mut() {
        if item.has_ricocheted {
            continue;
        }

        handle_map_rollover(transform.borrow_mut(), global_transform)
    }
}

pub fn _handle_particle_map_rollover(mut query: Query<(&Particle, &mut Transform, &GlobalTransform)>) {
    for (_, mut transform, global_transform) in query.iter_mut() {
        handle_map_rollover(transform.borrow_mut(), global_transform)
    }
}
