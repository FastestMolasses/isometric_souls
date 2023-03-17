use crate::animation;
use bevy::prelude::*;
use std::hash::Hash;
use std::time::Duration;

#[derive(Component, Default)]
pub struct AnimationSpriteSheet<T> {
    pub current_animation: T, // T should be an enum of all the animations

    pub locked: bool,
    pub column_count: usize,
    pub state: animation::state::State,
}

pub trait AnimationLookup<T> {
    fn get_animation(anim_enum: &T) -> animation::Animation;
}

/// AnimationSpriteSheet should always have T as HashMap<U, animation::Animation>
/// U should be an enum of all the animations
impl<T: Default + Eq + Copy + Hash + AnimationLookup<T>> AnimationSpriteSheet<T> {
    pub fn set_animation(&mut self, animation: T) {
        if self.current_animation == animation || self.locked {
            return;
        }

        if self.get_animation(animation).mode == animation::Mode::Once {
            self.locked = true;
        } else {
            self.locked = false;
        }

        self.current_animation = animation;
        self.state.reset();
    }

    /// Looks up the current animation based on the enum provided
    pub fn get_current_animation(&self) -> animation::Animation {
        T::get_animation(&self.current_animation)
    }

    pub fn get_animation(&self, animation: T) -> animation::Animation {
        T::get_animation(&animation)
    }

    /// Update the animation state
    pub fn update_state(&mut self, duration: Duration) {
        self.state.update(&self.get_current_animation(), duration);
    }
}
