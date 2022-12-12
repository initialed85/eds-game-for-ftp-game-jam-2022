use bevy::math::Vec2;
use bevy::prelude::Color;

// misc
pub const ZERO: f32 = 0.0;
pub const DEGREES_MAX: f32 = 360.0;

// app
pub const TITLE: &str = "eds-game-for-ftp-game-jam-2022";
pub const BOUNDS: Vec2 = Vec2::new(640.0, 400.0);
pub const PIXELS_PER_METER: f32 = 1.0;
pub const TIME_STEP: f64 = 1.0 / 5.0;
pub const BACKGROUND_COLOR: Color = Color::DARK_GRAY;

// server
pub const LISTEN_HOST: &str = "0.0.0.0";
pub const LISTEN_PORT: i32 = 8080;

// common
pub const MATERIAL_SCALE: f32 = 48.0;
pub const FRICTION_COEFFICIENT: f32 = 0.7;
pub const RESTITUTION_COEFFICIENT: f32 = 0.3;

// player
pub const PLAYER_HEIGHT_MULTIPLIER: f32 = 1.25;
pub const PLAYER_WIDTH_MULTIPLIER: f32 = 1.0;
pub const PLAYER_POLYGON_RADIUS: f32 = 0.50;
pub const PLAYER_POLYGON_SIDES: usize = 3;
pub const PLAYER_COLLIDER_BALL_RADIUS: f32 = 0.44;
pub const PLAYER_DENSITY: f32 = 2.0;
pub const PLAYER_LINEAR_DAMPING: f32 = 1.0;
pub const PLAYER_ANGULAR_DAMPING: f32 = 1.0;
