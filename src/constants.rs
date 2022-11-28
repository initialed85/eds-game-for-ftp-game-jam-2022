use bevy::prelude::{Color, KeyCode, Vec2};

pub const TIME_STEP: f32 = 1.0 / 30.0;

// origin       =    0,    0
// right top    =  600,  320
// right bottom =  600, -320
// left bottom  = -600, -320
// left top     = -600,  320
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

pub const PLAYER_MATERIAL_SCALE: f32 = 64.0;
pub const PLAYER_FRICTION_COEFFICIENT: f32 = 0.7;
pub const PLAYER_RESTITUTION_COEFFICIENT: f32 = 0.3;
pub const PLAYER_DENSITY: f32 = 2.0;
pub const PLAYER_LINEAR_DAMPING: f32 = 1.0;
pub const PLAYER_ANGULAR_DAMPING: f32 = 1.0;

pub const WEAPON_FIRE_INTERVAL_S: f64 = 0.5;
pub const PROJECTILE_EXPIRY_S: f64 = 30.0;

pub const PLAYER_1_COLOR: Color = Color::CRIMSON;
pub const PLAYER_1_NAME: &str = "Player 1";
pub const PLAYER_1_FORWARD_KEY: KeyCode = KeyCode::Up;
pub const PLAYER_1_BACKWARD_KEY: KeyCode = KeyCode::Down;
pub const PLAYER_1_LEFT_KEY: KeyCode = KeyCode::Left;
pub const PLAYER_1_RIGHT_KEY: KeyCode = KeyCode::Right;
pub const PLAYER_1_FIRE_KEY: KeyCode = KeyCode::LAlt;
pub const PLAYER_1_STARTING_TRANSLATION_MULTIPLIER: f32 = 0.5;
pub const PLAYER_1_STARTING_ROTATION_MULTIPLIER: f32 = 0.25;

pub const PLAYER_2_COLOR: Color = Color::MIDNIGHT_BLUE;
pub const PLAYER_2_NAME: &str = "Player 2";
pub const PLAYER_2_FORWARD_KEY: KeyCode = KeyCode::W;
pub const PLAYER_2_BACKWARD_KEY: KeyCode = KeyCode::S;
pub const PLAYER_2_LEFT_KEY: KeyCode = KeyCode::A;
pub const PLAYER_2_RIGHT_KEY: KeyCode = KeyCode::D;
pub const PLAYER_2_FIRE_KEY: KeyCode = KeyCode::P;
pub const PLAYER_2_STARTING_TRANSLATION_MULTIPLIER: f32 = -0.5;
pub const PLAYER_2_STARTING_ROTATION_MULTIPLIER: f32 = -0.25;
