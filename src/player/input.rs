use crate::animation::sprite::AnimationSpriteSheet;
use crate::player::animation::{DirectionAtlasHandles, PlayerAnimation};
use crate::util::{direction_to_texture_atlas_direction, vec2_to_direction, Direction};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputState {
    move_direction: Vec2,
    attack: bool,
    dash: bool,
}

#[derive(Component, Default)]
pub struct AttackState {
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
        &mut AnimationSpriteSheet<PlayerAnimation>,
        &mut Transform,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut character, mut sprite_sheet, mut transform, mut atlas, mut sprite) in query.iter_mut() {
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
            // character.dash_duration.reset();
            println!("Start dashing!");
            sprite_sheet.set_animation(PlayerAnimation::Dash);
        }

        if character.dashing {
            // character.dash_duration.tick(time.delta());
            if sprite_sheet.state.is_ended() {
                character.dashing = false;
                println!("End dashing!");
            } else {
                let move_direction = character.last_move_direction.normalize_or_zero();

                let dash_speed = character.speed * 2.5;
                transform.translation +=
                    move_direction.extend(0.0) * dash_speed * time.delta_seconds();
            }
        }
    }
}

pub fn input_handling_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_state: ResMut<InputState>,
    mut query: Query<(
        &mut AnimationSpriteSheet<PlayerAnimation>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut sprite_sheet, mut sprite) in query.iter_mut() {
        sprite_sheet.update_state(time.delta());
        sprite.index = sprite_sheet.state.frame_index();

        // If the animation is locked, don't change it until it's done
        if sprite_sheet.locked {
            if sprite_sheet.state.is_ended() {
                println!("Animation ended! Unlocking...");
                sprite_sheet.locked = false;
                // This is needed otherwise the animation will be stuck on the last frame
                sprite_sheet.state.reset();
            } else {
                return;
            }
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

        if move_direction == Vec2::ZERO {
            println!("Set animation idle");
            sprite_sheet.set_animation(PlayerAnimation::Idle);
        } else {
            println!("Set animation run");
            sprite_sheet.set_animation(PlayerAnimation::Run);
        }

        input_state.move_direction = move_direction;
        input_state.attack = keyboard_input.pressed(KeyCode::Space);
        input_state.dash = keyboard_input.just_pressed(KeyCode::LShift);
    }
}
