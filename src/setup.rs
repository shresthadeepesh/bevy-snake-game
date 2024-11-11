use bevy::{asset::AssetServer, color::Color, math::{Vec2, Vec3}, prelude::{Camera2dBundle, Commands, Res, TextBundle, Transform}, sprite::{Sprite, SpriteBundle}, text::TextStyle, utils::default};
use rand::Rng;

use crate::{components::{Food, FoodType, GameTextures, Player, ScoreText, Wall}, constants::{FOOD_SIZE, MOVE_RIGHT, PLAYER_SIZE, WALL_COLOR, WALL_THICKNESS, WINDOW_BOUND_X, WINDOW_BOUND_Y, WINDOW_HEIGHT, WINDOW_WIDTH}};

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures = GameTextures {
        regular_food: asset_server.load("models/apple.png"),
        special_food: asset_server.load("models/cockroach.png"),
        rare_food: asset_server.load("models/frog.png"),
        // level_up_sound: asset_server.load("sounds/levelUp.ogg"),
    };

    commands.insert_resource(textures);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    setup_spawns(commands, asset_server);
}

pub fn setup_spawns(mut commands: Commands, asset_server: Res<AssetServer>) {
    //score
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        ScoreText,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.75, 0.75, 0.75),
                custom_size: PLAYER_SIZE,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Player {
            direction: MOVE_RIGHT.normalize(),
            speed: 250.0,
            segments: Vec::new(),
            segment_spacing: 30.0,
        },
    ));

    //spawn initial food
    spawn_initial_food(&mut commands, &asset_server);

    //spawn walls
    spawn_walls(commands, &asset_server);
}

fn spawn_initial_food(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let color = Color::srgb(0.0, 1.0, 0.0);
    let spawn_bounds = Vec2::new(
        (WINDOW_WIDTH - WALL_THICKNESS * 2.0) / 2.0,
        (WINDOW_HEIGHT - WALL_THICKNESS * 2.0) / 2.0,
    );
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-spawn_bounds.x..spawn_bounds.x);
    let y = rng.gen_range(-spawn_bounds.y..spawn_bounds.y);

    let regular_food = asset_server.load("models/apple.png");

    commands.spawn((
        SpriteBundle {
            texture: regular_food.clone(),
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                custom_size: FOOD_SIZE,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        },
        Food {
            food_type: FoodType::Regular { score: 1, color },
        },
    ));
}

fn spawn_walls(mut commands: Commands, asset_server: &Res<AssetServer>) {
    let wall_vertical_texture = asset_server.load("models/wall-vertical.png");
    let wall_horizontal_texture = asset_server.load("models/wall-horizontal.png");

    commands.spawn((
        SpriteBundle {
            texture: wall_vertical_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(-WINDOW_BOUND_X / 2.0, 0.0, 0.0),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            texture: wall_vertical_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_BOUND_X / 2.0, 0.0, 0.0),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            texture: wall_horizontal_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, WINDOW_BOUND_Y / 2.0, 0.0),
            ..default()
        },
        Wall,
    ));
    commands.spawn((
        SpriteBundle {
            texture: wall_horizontal_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -WINDOW_BOUND_Y / 2.0, 0.0),
            ..default()
        },
        Wall,
    ));
}