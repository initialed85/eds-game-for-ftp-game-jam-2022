use bevy::prelude::{Camera2dBundle, Commands, ResMut, Vec2};
use bevy_rapier2d::prelude::RapierConfiguration;

pub fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, 0.0);

    commands.spawn(Camera2dBundle::default());
}
