use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Res, Time};

use crate::constants::PROJECTILE_EXPIRY_S;
use crate::projectile::Projectile;

pub fn handle_projectile(mut query: Query<(Entity, &Projectile)>, time: Res<Time>, mut commands: Commands) {
    let now = time.elapsed_seconds_f64();

    for (entity, projectile) in query.iter_mut() {
        if now < projectile.created_at + PROJECTILE_EXPIRY_S {
            continue;
        }

        commands.entity(entity).despawn_recursive();
    }
}
