use std::f32::consts::PI;

use bevy::math::Vec2;
use bevy::prelude::{Color, KeyCode};

// misc
pub const ZERO: f32 = 0.0;
pub const HALF: f32 = 0.5;
pub const DEGREES_MAX: f32 = 360.0;
pub const RADIANS_TO_DEGREES: f32 = 180.0 / PI;

// app
pub const TITLE: &str = "eds-game-for-ftp-game-jam-2022";
pub const BOUNDS: Vec2 = Vec2::new(891.0, 445.0);
pub const PIXELS_PER_METER: f32 = 1.0;
pub const BASE_TIME_STEP: f64 = 1.0 / 30.0;
pub const BASE_TIME_STEP_NAME: &str = "base_time_step";
pub const BACKGROUND_COLOR: Color = Color::srgb(0.41, 0.41, 0.41);

// server
pub const LISTEN_HOST: &str = "0.0.0.0";
pub const LISTEN_PORT: i32 = 8080;

// common
pub const MATERIAL_SCALE: f32 = 36.0;
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
pub const PLAYER_FORWARD_KEY: KeyCode = KeyCode::ArrowUp;
pub const PLAYER_BACKWARD_KEY: KeyCode = KeyCode::ArrowDown;
pub const PLAYER_LEFT_KEY: KeyCode = KeyCode::ArrowLeft;
pub const PLAYER_RIGHT_KEY: KeyCode = KeyCode::ArrowRight;
pub const PLAYER_FIRE_KEY: KeyCode = KeyCode::AltLeft;
pub const PLAYER_ANGULAR_VELOCITY_MAX: f32 = 10.0 / 3.0;
pub const PLAYER_ANGULAR_VELOCITY_STEP: f32 = 1.0 / 2.0;
pub const PLAYER_LINEAR_VELOCITY_MAX: f32 = (10.0 / 3.0) * 2.0;
pub const PLAYER_NETWORK_UPDATE_RATE_SECONDS: f64 = 1.0 / 15.0;
pub const PLAYER_NETWORK_EMA_SMOOTHING_FACTOR: f64 = 0.99;

// weapon
pub const WEAPON_FIRE_RATE_SECONDS: f64 = 0.25;
pub const PROJECTILE_LINEAR_VELOCITY: f32 = (1000.0 / 3.0) * 2.0;
pub const PROEJCTILE_DIMENSION_MULTIPLIER: f32 = 1.0 / 3.5;
pub const PROJECTILE_DENSITY: f32 = 50.0;
pub const PROJECTILE_EXPIRY_SECONDS: f64 = 2.5;
pub const PROJECTILE_NETWORK_UPDATE_RATE_SECONDS: f64 = 1.0 / 15.0;
pub const PROJECTILE_NETWORK_EMA_SMOOTHING_FACTOR: f64 = 0.95;

// particles
pub const PARTICLE_EXPIRY_SECONDS: f64 = 0.5;
pub const PARTICLE_CHANGE_RATE_SECONDS: f64 = 0.05;
pub const PARTICLE_DIMENSION_MULTIPLIER: f32 = 1.0 / 4.0;
pub const PARTICLE_LINEAR_VELOCITY: f32 = 1000.0 / 3.0;
pub const PARTICLE_LINEAR_VELOCITY_CHANGE: f32 = 50.0;

// ui
pub const UI_BUTTON_WIDTH: f32 = 100.0;
pub const UI_BUTTON_HEIGHT: f32 = 50.0;
pub const UI_BUTTON_BOTTOM_LEFT_LEFT: f32 = 10.0;
pub const UI_BUTTON_BOTTOM_LEFT_TOP: f32 = BOUNDS.y - (10.0 + UI_BUTTON_HEIGHT);
pub const UI_BUTTON_BOTTOM_RIGHT_LEFT: f32 = BOUNDS.x - (10.0 + UI_BUTTON_WIDTH);
pub const UI_BUTTON_BOTTOM_RIGHT_TOP: f32 = BOUNDS.y - (10.0 + UI_BUTTON_HEIGHT);
pub const UI_BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
pub const UI_BUTTON_HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
pub const UI_BUTTON_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
