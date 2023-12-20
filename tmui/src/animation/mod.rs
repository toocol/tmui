pub mod inner;
pub mod manager;
pub mod snapshot;
pub mod state_holder;

mod progress;

use crate::application_window::ApplicationWindow;

use self::{inner::AnimationsHolder, progress::Progress};
use std::time::Duration;
use tlib::{prelude::*, reflect_trait, utils::TimeStamp};

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
    win_id: ObjectId,
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
            win_id: 0,
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
    pub fn start(&mut self, holder: AnimationsHolder, win_id: ObjectId) {
        if self.state != AnimationState::Stopped {
            return;
        }

        ApplicationWindow::window_of(win_id).high_load_request(true);

        self.win_id = win_id;
        self.animation_holder = Some(holder);
        self.start_time = TimeStamp::timestamp();
        self.end_time = self.start_time + self.duration.as_millis() as u64;
        self.state = AnimationState::Playing;
    }

    #[inline]
    pub fn update(&mut self, current_time: u64) -> bool {
        if let Some(ref mut holder) = self.animation_holder {
            self.progress = Progress(
                ((current_time as f64 - self.start_time as f64)
                    / (self.end_time as f64 - self.start_time as f64))
                    .min(1.) as f32,
            );

            holder.update(self.progress);

            if self.progress.0 == 1. {
                self.state = AnimationState::Stopped;
                ApplicationWindow::window_of(self.win_id).high_load_request(false);
                println!("Animation stopped.");
            }

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

#[test]
fn test() {
    let (current_time, start_time, end_time) = (1703062043983u64, 1703062043968u64, 1703062044468u64);
    let progress = 
                ((current_time as f64 - start_time as f64)
                    / (end_time as f64 - start_time as f64))
                    .min(1.);
    println!("{}", progress)
}