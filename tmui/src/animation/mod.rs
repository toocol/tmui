pub mod inner;

mod progress;

use self::{progress::Progress, inner::AnimationsHolder};
use std::time::Duration;
use tlib::{utils::TimeStamp, reflect_trait, prelude::*};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Animations {
    Linear,
    EaseIn,
    EaseOut,
    FadeLinear,
    FadeEaseIn,
    FadeEaseOut,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AnimationState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
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
    animation: Animations,
    animation_holder: Option<AnimationsHolder>,

    duration: Duration,
    start_time: u64,
    end_time: u64,

    /// The progress of the animation, the value was 0-1.
    progress: Progress,
}

impl AnimationModel {
    #[inline]
    pub fn new(animation: Animations, duration: Duration) -> Self {
        Self {
            state: AnimationState::Stopped,
            animation,
            animation_holder: None,
            duration,
            start_time: 0,
            end_time: 0,
            progress: Default::default(),
        }
    }

    #[inline]
    pub fn start(&mut self, holder: AnimationsHolder) {
        if self.state != AnimationState::Stopped {
            return;
        }

        self.animation_holder = Some(holder);
        self.start_time = TimeStamp::timestamp();
        self.end_time = self.start_time + self.duration.as_millis() as u64;
        self.state = AnimationState::Playing;
    }

    #[inline]
    pub fn update(&mut self, current_time: u64) -> bool {
        if let Some(ref mut holder) = self.animation_holder {
            self.progress = Progress(
                (current_time as f32 - self.start_time as f32)
                    / (self.end_time as f32 - self.start_time as f32),
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
}

#[reflect_trait]
pub trait Animatable {
    fn set_animation(&mut self, animation: Animations);

    fn animation(&self) -> Animations;
}