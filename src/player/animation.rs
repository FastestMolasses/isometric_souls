use crate::animation::{
    sprite::{AnimationLookup, AnimationSpriteSheet},
    Animation, FrameRate,
};
use crate::util::Direction;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerAnimation {
    Idle,
    Run,
    RunStop,
    Dash,
    DashAttack,
    Attack1,
    Attack2,
    Attack3,
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        PlayerAnimation::Idle
    }
}

// Resources
#[derive(Default)]
pub struct DirectionAtlasHandles(pub HashMap<Direction, Handle<TextureAtlas>>);
// Implement the Resource trait for the newtype
impl Resource for DirectionAtlasHandles {}

impl AnimationLookup<PlayerAnimation> for PlayerAnimation {
    fn get_animation(anim_enum: &PlayerAnimation) -> Animation {
        match anim_enum {
            PlayerAnimation::Idle => Animation::from_indices(0..=5, FrameRate::from_fps(12.0)),
            PlayerAnimation::Run => Animation::from_indices(20..=25, FrameRate::from_fps(12.0)),
            PlayerAnimation::RunStop => Animation::from_indices(30..=33, FrameRate::from_fps(12.0)),
            PlayerAnimation::DashAttack => {
                Animation::from_indices(80..=89, FrameRate::from_fps(12.0)).once()
            }
            PlayerAnimation::Dash => {
                Animation::from_indices(80..=89, FrameRate::from_fps(26.0)).once()
            }
            PlayerAnimation::Attack1 => {
                Animation::from_indices(90..=95, FrameRate::from_fps(12.0)).once()
            }
            PlayerAnimation::Attack2 => {
                Animation::from_indices(100..=108, FrameRate::from_fps(12.0)).once()
            }
            PlayerAnimation::Attack3 => {
                Animation::from_indices(110..=116, FrameRate::from_fps(12.0)).once()
            }
        }
    }
}
