use bevy::prelude::*;
use benimator::Animation;
use std::collections::HashMap;
use crate::util::Direction;

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerAnimation {
    Idle,
    Run,
    RunStop,
    Attack,
    DashAttack,
    Roll,
    Attack1,
    Attack2,
    Attack3,
}

// Resources
#[derive(Default)]
pub struct DirectionAtlasHandles(pub HashMap<Direction, Handle<TextureAtlas>>);
// Implement the Resource trait for the newtype
impl Resource for DirectionAtlasHandles {}
