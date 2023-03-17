use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

#[derive(Component)]
pub struct AnimationSpriteSheet<T, U> {
    pub current_animation: U,  // U should be an enum of all the animations

    pub row_length: usize,
    pub animations: T,  // T should be a HashMap<U, benimator::Animation>
    pub state: benimator::State,
    pub locked: bool,
}

/// AnimationSpriteSheet should always have T as HashMap<U, benimator::Animation>
/// U should be an enum of all the animations
impl<U: Eq + Hash> AnimationSpriteSheet<HashMap<U, benimator::Animation>, U> {
    pub fn set_animation(&mut self, animation: U) {
        if self.current_animation == animation {
            return;
        }

        self.current_animation = animation;
        self.state.reset();
    }

    pub fn get_animation(&self) -> &benimator::Animation {
        self.animations.get(&self.current_animation).unwrap()
    }

    /// Update the animation state
    pub fn update_state(&mut self, duration: Duration) {
        self.state.update(&self.get_animation().clone(), duration);
    }
}
