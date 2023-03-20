use std::time::Duration;
use super::{Animation, Frame, Mode};

/// Animation state
#[derive(Debug, Default, Clone)]
pub struct State {
    animation_frame_index: usize,
    sprite_frame_index: usize,
    elapsed_in_frame: Duration,
    /// Control ping_pong backward frame navigation.
    going_backward: bool,
    is_ended: bool,
}

impl State {
    /// Create a new state
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset animation state
    ///
    /// The animation will restart from the first frame, like if the animation was freshly spawned.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns the current frame index
    #[must_use]
    pub fn frame_index(&self) -> usize {
        self.sprite_frame_index
    }

    /// Returns true if the animation has ended
    #[must_use]
    pub fn is_ended(&self) -> bool {
        self.is_ended
    }

    #[must_use]
    fn frame<'a>(&self, animation: &'a Animation) -> &'a Frame {
        &animation.frames[self.animation_frame_index % animation.frames.len()]
    }

    /// Update the animation state
    #[allow(dead_code)]
    pub fn update(&mut self, animation: &Animation, delta: Duration) {
        debug_assert!(animation.has_frames());
        let mut frame = self.frame(animation);
        self.sprite_frame_index = frame.index;
        self.elapsed_in_frame += delta;
        while self.elapsed_in_frame >= frame.duration {
            let on_last_frame = self.animation_frame_index >= animation.frames.len() - 1;
            match animation.mode {
                Mode::RepeatFrom(loop_from) => {
                    if on_last_frame {
                        self.animation_frame_index = loop_from;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                Mode::PingPong => {
                    if self.going_backward {
                        if self.animation_frame_index == 0 {
                            self.going_backward = false;
                            self.animation_frame_index += 1;
                        } else {
                            self.animation_frame_index -= 1;
                        }
                    } else if on_last_frame {
                        self.going_backward = true;
                        self.animation_frame_index -= 1;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                Mode::Once => {
                    if on_last_frame {
                        self.is_ended = true;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
            }

            self.elapsed_in_frame -= frame.duration;
            frame = self.frame(animation);
            self.sprite_frame_index = frame.index;
        }
    }
}
