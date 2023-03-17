use crate::animation::AnimationSpriteSheet;
use crate::util::Direction;
use benimator::Animation;
use benimator::FrameRate;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerAnimation {
    Idle,
    Run,
    RunStop,
    DashAttack,
    Roll,
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

impl AnimationSpriteSheet<PlayerAnimation> {
    fn get_animation(anim_enum: &PlayerAnimation) -> Animation {
        match anim_enum {
            PlayerAnimation::Idle => Animation::from_indices(0..=6, FrameRate::from_fps(12.0)),
            PlayerAnimation::Run => Animation::from_indices(20..=26, FrameRate::from_fps(12.0)),
            PlayerAnimation::RunStop => Animation::from_indices(30..=34, FrameRate::from_fps(12.0)),
            PlayerAnimation::DashAttack => {
                Animation::from_indices(80..=89, FrameRate::from_fps(12.0)).once()
            }
            PlayerAnimation::Roll => {
                Animation::from_indices(90..=99, FrameRate::from_fps(12.0)).once()
            }
            PlayerAnimation::Attack1 => {
                Animation::from_indices(100..=105, FrameRate::from_fps(12.0)).once()
            }
            PlayerAnimation::Attack2 => {
                Animation::from_indices(110..=118, FrameRate::from_fps(12.0)).once()
            }
            PlayerAnimation::Attack3 => {
                Animation::from_indices(120..=126, FrameRate::from_fps(12.0)).once()
            }
        }
    }
}
