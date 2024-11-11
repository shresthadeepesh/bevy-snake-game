use bevy::{asset::Handle, color::Color, math::{Vec2, Vec3}, prelude::{Component, Event, Image, Resource}};


#[derive(Resource)]
pub struct GameTextures {
    pub regular_food: Handle<Image>,
    pub special_food: Handle<Image>,
    pub rare_food: Handle<Image>,
    // level_up_sound: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct GameState {
    pub running: bool,
}

#[derive(Component)]
pub struct Player {
    pub direction: Vec2,
    pub speed: f32,
    pub segments: Vec<Vec3>,
    pub segment_spacing: f32,
}

#[derive(Component)]
pub struct Segment;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Component, Debug)]
pub enum FoodType {
    Regular { score: u32, color: Color },
    Special { score: u32, color: Color },
    Rare { score: u32, color: Color },
}

#[derive(Component)]
pub struct Food {
    pub food_type: FoodType,
}

#[derive(Event)]
pub struct FoodEatenEvent;

#[derive(Component)]
pub struct ScoreText;


#[derive(Component)]
pub struct Wall;