use bevy::prelude::{Camera2dBundle, Commands, Component, ResMut, Vec2};
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_rapier2d::prelude::TimestepMode::Fixed;

use crate::constants::{TIME_STEP, ZERO};

#[derive(Component, Debug)]
pub struct MainCamera;

pub fn base_handle_setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(ZERO, ZERO);
    rapier_config.timestep_mode = Fixed {
        dt: TIME_STEP as f32,
        substeps: 1,
    };

    commands.spawn((Camera2dBundle::default(), MainCamera));
}
