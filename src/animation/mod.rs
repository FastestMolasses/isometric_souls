pub mod sprite;
pub mod state;

use core::time::Duration;

pub struct Animation {
    /// Frames
    pub frames: Vec<Frame>,
    /// Animation mode
    pub mode: Mode,
}

/// A single animation frame
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Frame {
    /// Index in the sprite atlas
    pub index: usize,
    /// How long should the frame be displayed
    pub duration: Duration,
}

impl Animation {
    /// Create a new animation from frames
    #[must_use]
    pub fn from_frames(frames: impl IntoIterator<Item = Frame>) -> Self {
        Self {
            frames: frames.into_iter().collect(),
            mode: Mode::default(),
        }
    }

    pub fn from_indices(indices: impl IntoIterator<Item = usize>, frame_rate: FrameRate) -> Self {
        let mut anim: Self = indices
            .into_iter()
            .map(|index| Frame::new(index, frame_rate.frame_duration))
            .collect();

        if frame_rate.is_total_duration {
            #[allow(clippy::cast_precision_loss)]
            let actual_duration = frame_rate.frame_duration.div_f64(anim.frames.len() as f64);
            for frame in &mut anim.frames {
                frame.duration = actual_duration;
            }
        }

        anim
    }

    /// Runs the animation once and then stop playing
    #[must_use]
    pub fn once(mut self) -> Self {
        self.mode = Mode::Once;
        self
    }

    /// Repeat the animation forever
    #[must_use]
    pub fn repeat(mut self) -> Self {
        self.mode = Mode::RepeatFrom(0);
        self
    }

    /// Repeat the animation forever, from a given frame index (loop back to it at the end of the
    /// animation)
    #[must_use]
    pub fn repeat_from(mut self, frame_index: usize) -> Self {
        self.mode = Mode::RepeatFrom(frame_index);
        self
    }

    /// Repeat the animation forever, going back and forth between the first and last frame.
    #[must_use]
    pub fn ping_pong(mut self) -> Self {
        self.mode = Mode::PingPong;
        self
    }

    pub(crate) fn has_frames(&self) -> bool {
        !self.frames.is_empty()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Once,
    RepeatFrom(usize),
    PingPong,
}

impl FromIterator<Frame> for Animation {
    fn from_iter<T: IntoIterator<Item = Frame>>(iter: T) -> Self {
        Self::from_frames(iter)
    }
}

impl Extend<Frame> for Animation {
    fn extend<T: IntoIterator<Item = Frame>>(&mut self, iter: T) {
        self.frames.extend(iter);
    }
}

impl Default for Mode {
    #[inline]
    fn default() -> Self {
        Self::RepeatFrom(0)
    }
}

impl Frame {
    /// Create a new animation frame
    ///
    /// The duration must be > 0
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    #[inline]
    #[must_use]
    pub fn new(index: usize, duration: Duration) -> Self {
        assert!(
            !duration.is_zero(),
            "zero-duration is invalid for animation frame"
        );
        Self { index, duration }
    }
}

/// Frame-Rate definition
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[must_use]
pub struct FrameRate {
    frame_duration: Duration,
    is_total_duration: bool,
}

impl FrameRate {
    /// Frame rate defined by the FPS (Frame-Per-Second)
    ///
    /// # Panics
    ///
    /// This function will panic if `fps` is negative, zero or not finite.
    pub fn from_fps(fps: f64) -> Self {
        assert!(fps.is_finite() && fps > 0.0, "Invalid FPS: ${fps}");
        Self {
            frame_duration: Duration::from_secs(1).div_f64(fps),
            is_total_duration: false,
        }
    }

    /// Frame rate defined by the duration of each frame
    pub fn from_frame_duration(duration: Duration) -> Self {
        Self {
            frame_duration: duration,
            is_total_duration: false,
        }
    }

    /// Frame rate defined by the total duration of the animation
    ///
    /// The actual FPS will then depend on the number of frame
    pub fn from_total_duration(duration: Duration) -> Self {
        Self {
            frame_duration: duration,
            is_total_duration: true,
        }
    }
}
