use bevy::prelude::{Color, KeyCode, Vec2};

// various
pub const ZERO: f32 = 0.0;
pub const HALF: f32 = 0.5;
pub const ONE: f32 = 1.0;
pub const PIXELS_PER_METER: f32 = 1.0;
pub const TIME_STEP: f64 = 1.0 / 5.0;
pub const DEGREES_MAX: f32 = 360.0;
pub const TITLE: &str = "eds-game-for-ftp-game-jam-2022";
pub const BACKGROUND_COLOR: Color = Color::DARK_GRAY;

// world
// origin       =    0,    0
// right top    =  600,  320
// right bottom =  600, -320
// left bottom  = -600, -320
// left top     = -600,  320
pub const BOUNDS: Vec2 = Vec2::new(800.0, 400.0);

// common
pub const MATERIAL_SCALE: f32 = 48.0;
pub const FRICTION_COEFFICIENT: f32 = 0.7;
pub const RESTITUTION_COEFFICIENT: f32 = 0.3;

// particles
pub const PARTICLE_EXPIRY_S: f64 = 0.5;
pub const PARTICLE_CHANGE_S: f64 = 0.05;
pub const PARTICLE_COUNT_FOR_PROJECTILES: i8 = 8;
pub const PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PLAYERS: i8 = 4;
pub const PARTICLE_COUNT_FOR_PLAYERS_AGAINST_PROJECTILES: i8 = 8;
pub const PARTICLE_Z_INDEX: f32 = 1.0;
pub const PARTICLE_DIMENSION: f32 = 0.05;
pub const PARTICLE_SPEED: f32 = 250.0;
pub const PARTICLE_SPEED_CHANGE: f32 = 50.0;

// weaponry
pub const WEAPON_FIRE_INTERVAL: f64 = 0.1;
pub const PROJECTILE_SPAWN_OFFSET: f32 = (MATERIAL_SCALE * HALF) + 6.0;
pub const PROJECTILE_DENSITY: f32 = 10.0;
pub const PROJECTILE_EXPIRY: f64 = 5.0;
pub const PROJECTILE_RICOCHET_EXPIRY: f64 = 1.0;
pub const PROJECTILE_Z_INDEX: f32 = 0.8;
pub const PROJECTILE_DIMENSION: f32 = 0.1;
pub const PROJECTILE_SPEED: f32 = 1000.0;

// player
pub const PLAYER_POLYGON_RADIUS: f32 = 0.50;
pub const PLAYER_POLYGON_SIDES: usize = 3;
pub const PLAYER_HEIGHT_MULTIPLIER: f32 = 1.25;
pub const PLAYER_WIDTH_MULTIPLIER: f32 = 1.0;
pub const PLAYER_Z_INDEX: f32 = 0.9;
pub const PLAYER_DENSITY: f32 = 2.0;
pub const PLAYER_LINEAR_DAMPING: f32 = 1.0;
pub const PLAYER_ANGULAR_DAMPING: f32 = 1.0;
pub const PLAYER_ANGULAR_VELOCITY_MAX: f32 = 4.0;
pub const PLAYER_ANGULAR_VELOCITY_STEP: f32 = 0.1;
pub const PLAYER_LINEAR_VELOCITY: f32 = 10.0;
pub const PLAYER_COLLIDER_BALL_RADIUS: f32 = 0.44;

// player 1
pub const PLAYER_1_COLOR: Color = Color::CRIMSON;
pub const PLAYER_1_NAME: &str = "Player 1";
pub const PLAYER_1_FORWARD_KEY: KeyCode = KeyCode::Up;
pub const PLAYER_1_BACKWARD_KEY: KeyCode = KeyCode::Down;
pub const PLAYER_1_LEFT_KEY: KeyCode = KeyCode::Left;
pub const PLAYER_1_RIGHT_KEY: KeyCode = KeyCode::Right;
pub const PLAYER_1_FIRE_KEY: KeyCode = KeyCode::LAlt;
pub const PLAYER_1_STARTING_TRANSLATION_MULTIPLIER: f32 = 0.5;
pub const PLAYER_1_STARTING_ROTATION_MULTIPLIER: f32 = 0.0;

// player 2
pub const PLAYER_2_COLOR: Color = Color::MIDNIGHT_BLUE;
pub const PLAYER_2_NAME: &str = "Player 2";
pub const PLAYER_2_FORWARD_KEY: KeyCode = KeyCode::W;
pub const PLAYER_2_BACKWARD_KEY: KeyCode = KeyCode::S;
pub const PLAYER_2_LEFT_KEY: KeyCode = KeyCode::A;
pub const PLAYER_2_RIGHT_KEY: KeyCode = KeyCode::D;
pub const PLAYER_2_FIRE_KEY: KeyCode = KeyCode::P;
pub const PLAYER_2_STARTING_TRANSLATION_MULTIPLIER: f32 = -0.5;
pub const PLAYER_2_STARTING_ROTATION_MULTIPLIER: f32 = 0.0;

// network
pub const SERVER_SUFFIX: &str = "/ws";
pub const PLAYER_UPDATE_INTERVAL: f64 = 0.1;
pub const PLAYER_EXPIRY_SECONDS: f64 = 5.0;
