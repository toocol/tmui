pub mod inner;
pub mod snapshot;
pub mod state_holder;

mod progress;

use self::{inner::AnimationsHolder, progress::Progress};
use std::time::Duration;
use tlib::{prelude::*, reflect_trait};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Animation {
    #[default]
    NoAnimation,
    Linear,
    EaseIn,
    EaseOut,
    FadeLinear,
    FadeEaseIn,
    FadeEaseOut,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum AnimationState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    #[default]
    NoDirection,
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop,
    LeftTopToRightBottom,
    LeftBottomToRightTop,
    RightTopToLeftBottom,
    RightBottomToLeftTop,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AnimationModel {
    state: AnimationState,
    animation: Animation,
    direction: Direction,
    animation_holder: Option<AnimationsHolder>,

    duration: Duration,
    start_time: u64,
    end_time: u64,

    /// The progress of the animation, the value was 0-1.
    progress: Progress,
}

impl AnimationModel {
    #[inline]
    pub fn new(animation: Animation, direction: Direction, duration: Duration) -> Self {
        Self {
            state: AnimationState::Stopped,
            animation,
            direction,
            animation_holder: None,
            duration,
            start_time: 0,
            end_time: 0,
            progress: Default::default(),
        }
    }

    /// This function will be processed by macros.
    #[inline]
    pub fn start(&mut self, start_time: u64, holder: AnimationsHolder) {
        if self.state != AnimationState::Stopped {
            return;
        }

        self.animation_holder = Some(holder);
        self.start_time = start_time;
        self.end_time = self.start_time + self.duration.as_millis() as u64;
        self.state = AnimationState::Playing;
    }

    #[inline]
    pub fn update(&mut self, current_time: u64) -> bool {
        if let Some(ref mut holder) = self.animation_holder {
            self.progress = Progress(
                ((current_time as f32 - self.start_time as f32)
                    / (self.end_time as f32 - self.start_time as f32))
                    .min(1.),
            );

            holder.update(self.progress);

            return true;
        }
        false
    }

    #[inline]
    pub fn state(&self) -> AnimationState {
        self.state
    }

    #[inline]
    pub fn set_animation(&mut self, animation: Animation) {
        self.animation = animation
    }

    #[inline]
    pub fn animation(&self) -> Animation {
        self.animation
    }

    #[inline]
    pub fn is_playing(&self) -> bool {
        self.state == AnimationState::Playing
    }
}

#[reflect_trait]
pub trait Animatable {
    fn set_animation(&mut self, animation: Animation);

    fn animation(&self) -> Animation;

    fn animation_model(&self) -> &AnimationModel;

    fn animation_model_mut(&mut self) -> &mut AnimationModel;
}
