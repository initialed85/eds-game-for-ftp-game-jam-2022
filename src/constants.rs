use bevy::prelude::{Color, KeyCode, Vec2};

pub const TIME_STEP: f32 = 1.0 / 30.0;

// world
// origin       =    0,    0
// right top    =  600,  320
// right bottom =  600, -320
// left bottom  = -600, -320
// left top     = -600,  320
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

// common
pub const MATERIAL_SCALE: f32 = 64.0;
pub const FRICTION_COEFFICIENT: f32 = 0.7;
pub const RESTITUTION_COEFFICIENT: f32 = 0.3;

// particles
pub const PARTICLE_EXPIRY_S: f64 = 0.5;
pub const PARTICLE_CHANGE_S: f64 = 0.05;
pub const PARTICLE_COUNT_FOR_PROJECTILES: i8 = 16;
pub const PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PLAYERS: i8 = 4;
pub const PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PROJECTILES: i8 = 4;

// weaponry
pub const WEAPON_FIRE_INTERVAL_S: f64 = 0.1;
pub const PROJECTILE_DENSITY: f32 = 10.0;
pub const PROJECTILE_EXPIRY_S: f64 = 10.0;
pub const PROJECTILE_RICOCHET_EXPIRY_S: f64 = 1.0;

// player
pub const PLAYER_DENSITY: f32 = 2.0;
pub const PLAYER_LINEAR_DAMPING: f32 = 1.0;
pub const PLAYER_ANGULAR_DAMPING: f32 = 1.0;

// player 1
pub const PLAYER_1_COLOR: Color = Color::CRIMSON;
pub const PLAYER_1_NAME: &str = "Player 1";
pub const PLAYER_1_FORWARD_KEY: KeyCode = KeyCode::Up;
pub const PLAYER_1_BACKWARD_KEY: KeyCode = KeyCode::Down;
pub const PLAYER_1_LEFT_KEY: KeyCode = KeyCode::Left;
pub const PLAYER_1_RIGHT_KEY: KeyCode = KeyCode::Right;
pub const PLAYER_1_FIRE_KEY: KeyCode = KeyCode::LAlt;
pub const PLAYER_1_STARTING_TRANSLATION_MULTIPLIER: f32 = 0.5;
pub const PLAYER_1_STARTING_ROTATION_MULTIPLIER: f32 = 0.25;

// player 2
pub const PLAYER_2_COLOR: Color = Color::MIDNIGHT_BLUE;
pub const PLAYER_2_NAME: &str = "Player 2";
pub const PLAYER_2_FORWARD_KEY: KeyCode = KeyCode::W;
pub const PLAYER_2_BACKWARD_KEY: KeyCode = KeyCode::S;
pub const PLAYER_2_LEFT_KEY: KeyCode = KeyCode::A;
pub const PLAYER_2_RIGHT_KEY: KeyCode = KeyCode::D;
pub const PLAYER_2_FIRE_KEY: KeyCode = KeyCode::P;
pub const PLAYER_2_STARTING_TRANSLATION_MULTIPLIER: f32 = -0.5;
pub const PLAYER_2_STARTING_ROTATION_MULTIPLIER: f32 = -0.25;
