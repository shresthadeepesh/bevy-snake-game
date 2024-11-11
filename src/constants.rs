use bevy::{color::Color, math::Vec2};


pub const FOOD_SIZE: Option<Vec2> = Some(Vec2::new(50.0, 50.0));
pub const PLAYER_SIZE: Option<Vec2> = Some(Vec2::new(50.0, 50.0));

pub const WALL_THICKNESS: f32 = 50.0;
pub const WALL_COLOR: Color = Color::srgb(204.0, 0.0, 255.0);

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_BOUND_X: f32 = WINDOW_WIDTH;
pub const WINDOW_BOUND_Y: f32 = WINDOW_HEIGHT;
 
pub const MOVE_RIGHT: Vec2 = Vec2::new(1.0, 0.0);
pub const MOVE_LEFT: Vec2 = Vec2::new(-1.0, 0.0);
pub const MOVE_UP: Vec2 = Vec2::new(0.0, 1.0);
pub const MOVE_DOWN: Vec2 = Vec2::new(0.0, -1.0);