use bevy::prelude::{Query, Transform, Vec3};

use crate::behaviour::moveable::Moveable;
use crate::constants::{BOUNDS, HALF};

pub fn handle_rollover_for_moveable(mut moveable_query: Query<(&mut Moveable, &mut Transform)>) {
    for (mut moveable, mut transform) in moveable_query.iter_mut() {
        let extents: Vec3 = Vec3::from((BOUNDS * HALF, 0.0));

        let right = extents.x;
        let left = -extents.x;
        let up = extents.y;
        let down = -extents.y;

        let mut had_rollover = false;

        if transform.translation.x > right {
            transform.translation.x = left;
            had_rollover = true;
        }

        if transform.translation.x < left {
            transform.translation.x = right;
            had_rollover = true;
        }

        if transform.translation.y > up {
            transform.translation.y = down;
            had_rollover = true;
        }

        if transform.translation.y < down {
            transform.translation.y = up;
            had_rollover = true;
        }

        // don't remove an unhandled had_rollover state
        if moveable.had_rollover && !had_rollover {
            continue;
        }

        moveable.had_rollover = had_rollover;
    }
}
