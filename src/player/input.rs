use bevy::prelude::*;
use std::time::Duration;
use crate::util::{Direction, vec2_to_direction, direction_to_texture_atlas_direction};
use crate::player::animation::DirectionAtlasHandles;

#[derive(Resource, Default)]
pub struct InputState {
    move_direction: Vec2,
    attack: bool,
    dash: bool,
}

#[derive(Component, Default)]
pub struct AttackState {
    pub attack_chain: Vec<f32>,
    pub attack_timer: Timer,
    pub current_attack: usize,
}

#[derive(Component)]
pub struct Character {
    pub speed: f32,
    pub dash_duration: Timer,
    pub dashing: bool,
    pub last_move_direction: Vec2,
}

pub fn character_controller_system(
    time: Res<Time>,
    input_state: Res<InputState>,
    direction_atlas_handles: ResMut<DirectionAtlasHandles>,
    mut query: Query<(
        &mut Character,
        &mut Transform,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut character, mut transform, mut atlas, mut sprite) in query.iter_mut() {
        // 8-directional movement
        if !character.dashing {
            if input_state.move_direction != Vec2::ZERO {
                let move_direction = input_state.move_direction.normalize();
                transform.translation +=
                    move_direction.extend(0.0) * character.speed * time.delta_seconds();
                character.last_move_direction = move_direction;

                let direction = vec2_to_direction(move_direction);
                // Flip the sprite based on the current direction
                sprite.flip_x = matches!(
                    direction,
                    Direction::SouthWest | Direction::West | Direction::NorthWest
                );

                let direction = direction_to_texture_atlas_direction(move_direction);
                // Update the texture atlas based on the character's direction
                if let Some(atlas_handle) = direction_atlas_handles.0.get(&direction) {
                    *atlas = atlas_handle.clone();
                }
            }
        }

        // Dash
        if input_state.dash && !character.dashing {
            character.dashing = true;
            character.dash_duration.reset();
            println!("Start dashing!");
        }

        if character.dashing {
            character.dash_duration.tick(time.delta());
            if character.dash_duration.finished() {
                character.dashing = false;
                println!("End dashing!");
            } else {
                let move_direction = if input_state.move_direction != Vec2::ZERO {
                    input_state.move_direction.normalize()
                } else {
                    character.last_move_direction
                };

                let dash_speed = character.speed * 2.5;
                transform.translation +=
                    move_direction.extend(0.0) * dash_speed * time.delta_seconds();
            }
        }
    }
}

pub fn input_handling_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_state: ResMut<InputState>,
    mut query: Query<&mut TextureAtlasSprite>,
) {
    for mut sprite in &mut query {
        sprite.index = 1;
    }

    let mut move_direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }

    input_state.move_direction = move_direction;
    input_state.attack = keyboard_input.pressed(KeyCode::Space);
    input_state.dash = keyboard_input.just_pressed(KeyCode::LShift);
}

pub fn attack_handling_system(
    time: Res<Time>,
    input_state: Res<InputState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut AttackState, &Character)>,
) {
    for (mut attack_state, character) in query.iter_mut() {
        if character.dashing {
            continue;
        }

        attack_state.attack_timer.tick(time.delta());

        if keyboard_input.just_pressed(KeyCode::Space) {
            if attack_state.current_attack == 0 || attack_state.attack_timer.finished() {
                // Start the first attack or chain the next attack
                attack_state.current_attack =
                    (attack_state.current_attack) % attack_state.attack_chain.len() + 1;

                let current_attack_duration =
                    attack_state.attack_chain[attack_state.current_attack - 1];
                attack_state
                    .attack_timer
                    .set_duration(Duration::from_secs_f32(current_attack_duration));
                attack_state.attack_timer.reset();

                println!("Attack {}!", attack_state.current_attack);
            }
        } else if !input_state.attack {
            if attack_state.attack_timer.finished() {
                // If the attack button isn't pressed and the timer has finished, reset the current attack
                attack_state.current_attack = 0;
            }
        }
    }
}
