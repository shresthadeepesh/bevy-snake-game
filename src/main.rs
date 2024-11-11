use bevy::app::AppExit;
use bevy::prelude::*;
use components::{Food, FoodEatenEvent, FoodType, GameState, GameTextures, Player, Score, ScoreText, Segment, Wall};
use constants::{FOOD_SIZE, PLAYER_SIZE, WALL_THICKNESS, WINDOW_HEIGHT, WINDOW_WIDTH};
use controls::{bind_controls, handle_restart, move_player};
use rand::Rng;
use setup::{load_textures, setup};

mod components;
mod constants;
mod setup;
mod controls;

// #[derive(Resource, Default, Deref, DerefMut)]
// struct LevelUpSound(Handle<AudioSource>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: "Snake Game".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_event::<FoodEatenEvent>()
        .add_systems(Startup, load_textures)
        .add_systems(Startup, setup)
        .init_resource::<Score>()
        .insert_resource(GameState { running: true })
        .add_systems(
            Update,
            (
                bind_controls,
                move_player,
                spawn_food,
                check_food_collision,
                update_score_text,
                handle_restart,
                check_collisions,
                close_on_esc,
            ),
        )
        .run();
}



fn close_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn check_collisions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<(&Transform, &Sprite, &Player)>,
    wall_query: Query<(&Transform, &Sprite), With<Wall>>,
) {
    if !game_state.running {
        return;
    }

    let (player_transform, player_sprite, _) = player_query.single();
    let player_size = player_sprite.custom_size.unwrap_or(Vec2::new(50.0, 50.0));

    for (wall_transform, wall_sprite) in wall_query.iter() {
        let wall_size = wall_sprite
            .custom_size
            .unwrap_or(Vec2::new(WALL_THICKNESS, WALL_THICKNESS));

        // Calculate AABB collision
        let collision = collide_aabb(
            player_transform.translation,
            player_size,
            wall_transform.translation,
            wall_size,
        );

        if collision.is_some() {
            game_over(&mut commands, &mut game_state);
            return;
        }
    }
}

// Helper function for AABB collision detection
fn collide_aabb(pos_a: Vec3, size_a: Vec2, pos_b: Vec3, size_b: Vec2) -> Option<()> {
    let min_a = Vec2::new(pos_a.x - size_a.x / 2.0, pos_a.y - size_a.y / 2.0);
    let max_a = Vec2::new(pos_a.x + size_a.x / 2.0, pos_a.y + size_a.y / 2.0);
    let min_b = Vec2::new(pos_b.x - size_b.x / 2.0, pos_b.y - size_b.y / 2.0);
    let max_b = Vec2::new(pos_b.x + size_b.x / 2.0, pos_b.y + size_b.y / 2.0);

    if max_a.x < min_b.x || min_a.x > max_b.x || max_a.y < min_b.y || min_a.y > max_b.y {
        None
    } else {
        Some(())
    }
}

fn spawn_food_at_random_position(commands: &mut Commands, game_textures: &Res<GameTextures>) {
    let mut rng = rand::thread_rng();

    let food_type = match rng.gen_range(0..100) {
        0..=70 => FoodType::Regular {
            score: 1,
            color: Color::srgb(0.0, 1.0, 0.0),
        },
        71..=90 => FoodType::Special {
            score: 5,
            color: Color::srgb(0.0, 0.0, 1.0),
        },
        _ => FoodType::Rare {
            score: 10,
            color: Color::srgb(1.0, 0.0, 0.0),
        },
    };

    let (color, food_texture) = match &food_type {
        FoodType::Regular { color, .. } => (*color, game_textures.regular_food.clone()),
        FoodType::Special { color, .. } => (*color, game_textures.special_food.clone()),
        FoodType::Rare { color, .. } => (*color, game_textures.rare_food.clone()),
    };

    let spawn_bounds = Vec2::new(
        (WINDOW_WIDTH - WALL_THICKNESS * 2.0) / 2.0,
        (WINDOW_HEIGHT - WALL_THICKNESS * 2.0) / 2.0,
    );

    let x = rng.gen_range(-spawn_bounds.x..spawn_bounds.x);
    let y = rng.gen_range(-spawn_bounds.y..spawn_bounds.y);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                // Add these lines to handle transparency
                color: Color::srgba(1.0, 1.0, 1.0, 1.0), // Full opacity
                flip_x: false,
                flip_y: false,
                custom_size: FOOD_SIZE, // Adjust size as needed
                ..default()
            },
            texture: food_texture,
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        },
        Food { food_type },
    ));
}

fn spawn_food(
    mut commands: Commands,
    mut food_eaten_events: EventReader<FoodEatenEvent>,
    mut player_query: Query<(&Transform, &mut Player)>,
    game_textures: Res<GameTextures>,
) {
    let (player_transform, mut player) = player_query.single_mut();

    for _ in food_eaten_events.read() {
        spawn_food_at_random_position(&mut commands, &game_textures);

        let new_segment_pos = if let Some(last_segment) = player.segments.last() {
            let direction = if player.segments.len() > 1 {
                (*last_segment - player.segments[player.segments.len() - 2]).normalize()
            } else {
                Vec3::new(player.direction.x, player.direction.y, 0.0)
            };
            *last_segment - direction * player.segment_spacing
        } else {
            player_transform.translation
                - Vec3::new(
                    player.direction.x * player.segment_spacing,
                    player.direction.y * player.segment_spacing,
                    0.0,
                )
        };

        player.segments.push(new_segment_pos);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.75, 0.75, 0.75),
                    custom_size: PLAYER_SIZE,
                    ..default()
                },
                transform: Transform::from_translation(new_segment_pos),
                ..default()
            },
            Segment,
        ));
    }
}

fn check_food_collision(
    mut commands: Commands,
    mut food_eaten_events: EventWriter<FoodEatenEvent>,
    player_query: Query<&Transform, With<Player>>,
    food_query: Query<(Entity, &Transform, &Food)>,
    mut score: ResMut<Score>,
    game_textures: Res<GameTextures>,
) {
    let player_transform = player_query.single();

    for (food_entity, food_transform, food) in food_query.iter() {
        let distance = player_transform
            .translation
            .distance(food_transform.translation);

        if distance < 35.0 {
            let points = match &food.food_type {
                FoodType::Regular { score, .. } => *score,
                FoodType::Special { score, .. } => *score,
                FoodType::Rare { score, .. } => *score,
            };

            score.value += points;
            commands.entity(food_entity).despawn();
            food_eaten_events.send(FoodEatenEvent);

            // commands.spawn(AudioBundle {
            //     source: game_textures.level_up_sound.clone(),
            //     settings: PlaybackSettings::DESPAWN,
            // });
        }
    }
}

//update score text
fn update_score_text(score: Res<Score>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.value);
    }
}

fn game_over(commands: &mut Commands, game_state: &mut GameState) {
    game_state.running = false;

    commands.spawn(
        TextBundle::from_section(
            "Game Over!\n Press R to Restart.",
            TextStyle {
                font_size: 50.0,
                color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(Val::Px(100.0), Val::Auto, Val::Px(150.0), Val::Auto),
            ..default()
        }),
    );
}

