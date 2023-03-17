use bevy::prelude::*;
use std::hash::Hash;
use std::time::Duration;

#[derive(Component, Default)]
pub struct AnimationSpriteSheet<T> {
    pub current_animation: T,  // T should be an enum of all the animations

    pub row_length: usize,
    pub state: benimator::State,
    pub locked: bool,
}

pub trait AnimationSpriteSheetTrait<T> {
    fn set_animation(&mut self, animation: T);
    fn update_state(&mut self, animation: benimator::Animation, duration: Duration);
    fn get_animation(anim_enum: &T) -> benimator::Animation;
}

/// AnimationSpriteSheet should always have T as HashMap<U, benimator::Animation>
/// U should be an enum of all the animations
impl<T: Default + Eq + Hash> AnimationSpriteSheetTrait<T> for AnimationSpriteSheet<T> {
    fn set_animation(&mut self, animation: T) {
        if self.current_animation == animation {
            return;
        }

        self.current_animation = animation;
        self.state.reset();
    }

    /// Update the animation state
    fn update_state(&mut self, animation: benimator::Animation, duration: Duration) {
        self.state.update(&animation, duration);
    }

    fn get_animation(_: &T) -> benimator::Animation {
        unimplemented!()
    }
}
