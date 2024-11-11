use bevy::{input::ButtonInput, math::Vec3, prelude::{Commands, Entity, KeyCode, Mut, Or, Query, Res, ResMut, Transform, With, Without}, text::Text, time::Time};

use crate::{components::{Food, GameState, GameTextures, Player, Score, Segment}, constants::{MOVE_DOWN, MOVE_LEFT, MOVE_RIGHT, MOVE_UP, WALL_THICKNESS, WINDOW_BOUND_X, WINDOW_BOUND_Y}, setup::setup_spawns};

pub fn move_player(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    mut segment_query: Query<&mut Transform, (With<Segment>, Without<Player>)>,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player_query.single_mut();

    let current_pos = transform.translation;

    let movement = player.direction * player.speed * time.delta_seconds();
    transform.translation += Vec3::new(movement.x, movement.y, 0.0);

    let mut previos_pos = current_pos;
    for (_, segment_pos) in player.segments.iter_mut().enumerate() {
        let temp_pos = *segment_pos;
        *segment_pos = previos_pos;
        previos_pos = temp_pos;
    }

    for (i, mut segment_transform) in segment_query.iter_mut().enumerate() {
        segment_transform.translation = player.segments[i];
    }

    check_boundary_collision(transform);
}

pub fn bind_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    game_state: Res<GameState>,
) {
    if !game_state.running {
        return;
    }

    let (mut tranform, mut player) = player_query.single_mut();

    let movement_speed = 5.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) && player.direction != MOVE_RIGHT {
        tranform.translation.x -= movement_speed;
        player.direction = MOVE_LEFT.normalize();
    } else if keyboard_input.pressed(KeyCode::ArrowRight) && player.direction != MOVE_LEFT {
        tranform.translation.x += movement_speed;
        player.direction = MOVE_RIGHT.normalize();
    } else if keyboard_input.pressed(KeyCode::ArrowUp) && player.direction != MOVE_DOWN {
        tranform.translation.y += movement_speed;
        player.direction = MOVE_UP.normalize();
    } else if keyboard_input.pressed(KeyCode::ArrowDown) && player.direction != MOVE_UP {
        tranform.translation.y -= movement_speed;
        player.direction = MOVE_DOWN.normalize();
    }

    check_boundary_collision(tranform);
}

fn check_boundary_collision(mut player_transform: Mut<'_, Transform>) {
    let clamped_x = (WINDOW_BOUND_X / 2.0) - WALL_THICKNESS;
    let clamped_y = (WINDOW_BOUND_Y / 2.0) - WALL_THICKNESS;
    player_transform.translation.x = player_transform.translation.x.clamp(-clamped_x, clamped_x);
    player_transform.translation.y = player_transform.translation.y.clamp(-clamped_y, clamped_y);
}

pub fn handle_restart(
    game_textures: Res<GameTextures>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    query: Query<Entity, Or<(With<Player>, With<Segment>, With<Food>, With<Text>)>>,
    mut score: ResMut<Score>,
) {
    if !game_state.running && keyboard.just_pressed(KeyCode::KeyR) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        game_state.running = true;
        score.value = 0;

        //respawn player and food
        setup_spawns(commands);
    }
}
