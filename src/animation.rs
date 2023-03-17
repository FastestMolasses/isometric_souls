use benimator::FrameRate;
use bevy::prelude::*;
use std::hash::Hash;
use std::time::Duration;

#[derive(Component, Default)]
pub struct AnimationSpriteSheet<T> {
    pub current_animation: T, // T should be an enum of all the animations

    pub locked: bool,
    pub column_count: usize,
    pub state: benimator::State,
}

pub trait AnimationSpriteSheetTrait<T> {
    fn set_animation(&mut self, animation: T);
    fn get_current_animation(&self) -> benimator::Animation;
    fn update_state(&mut self, duration: Duration);
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

    /// Looks up the current animation based on the enum provided
    fn get_current_animation(&self) -> benimator::Animation {
        Self::get_animation(&self.current_animation)
    }

    /// Update the animation state
    fn update_state(&mut self, duration: Duration) {
        self.state.update(&self.get_current_animation(), duration);
    }

    fn get_animation(_: &T) -> benimator::Animation {
        // unimplemented!()
        benimator::Animation::from_indices(12..20, FrameRate::from_fps(12.0))
    }
}
