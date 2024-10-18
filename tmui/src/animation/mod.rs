pub mod frame_animator;
pub mod inner;
pub mod mgr;
pub mod snapshot;
pub mod state_holder;

mod progress;

use crate::application_window::ApplicationWindow;

use self::{inner::AnimationsHolder, progress::Progress};
use std::{ptr::NonNull, time::Duration};
use tlib::{figure::{FRect, Rect}, prelude::*, reflect_trait, utils::Timestamp};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Animation {
    Linear,
    EaseIn,
    EaseOut,
    FadeLinear,
    FadeEaseIn,
    FadeEaseOut,
}
impl Animation {
    #[inline]
    pub(crate) fn create_rect_holder(
        &self,
        start: FRect,
        end: FRect,
        hold: Option<NonNull<FRect>>,
    ) -> AnimationsHolder {
        match self {
            Animation::Linear => AnimationsHolder::Linear { start, end, hold },
            Animation::EaseIn => AnimationsHolder::EaseIn { start, end, hold },
            Animation::EaseOut => AnimationsHolder::EaseOut { start, end, hold },
            _ => panic!("Unexpected `Animation` type for creating rect holder."),
        }
    }

    #[inline]
    pub(crate) fn create_transparency_holder(
        &self,
        start: i32,
        end: i32,
        hold: Option<NonNull<i32>>,
    ) -> AnimationsHolder {
        match self {
            Animation::FadeLinear => AnimationsHolder::FadeLinear { start, end, hold },
            Animation::FadeEaseIn => AnimationsHolder::FadeEaseIn { start, end, hold },
            Animation::FadeEaseOut => AnimationsHolder::FadeEaseOut { start, end, hold },
            _ => panic!("Unexpected `Animation` type for creating color holder."),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum AnimationState {
    #[default]
    Stopped,
    Playing,
    Pending,
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

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum AnimationMode {
    #[default]
    Flex,
    Stealth,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum AnimationEffect {
    #[default]
    Appearance,
    Slide,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AnimationModel {
    mode: AnimationMode,
    win_id: ObjectId,
    state: AnimationState,
    animation: Animation,
    direction: Option<Direction>,
    effect: Option<AnimationEffect>,

    animation_holder: Option<AnimationsHolder>,
    shown: bool,
    origin_rect: Option<Rect>,
    origin_transparency: Option<i32>,

    duration: Duration,
    start_time: u64,
    end_time: u64,

    /// The progress of the animation, the value was 0-1.
    progress: Progress,
}

impl AnimationModel {
    #[inline]
    pub fn new(
        mode: AnimationMode,
        animation: Animation,
        duration: Duration,
        direction: Option<Direction>,
        effect: Option<AnimationEffect>,
    ) -> Self {
        Self {
            mode,
            win_id: 0,
            state: AnimationState::Stopped,
            animation,
            direction,
            effect,
            animation_holder: None,
            shown: false,
            origin_rect: None,
            origin_transparency: None,
            duration,
            start_time: 0,
            end_time: 0,
            progress: Default::default(),
        }
    }

    /// This function will be processed by macros.
    #[inline]
    pub fn start(&mut self, holder: AnimationsHolder, win_id: ObjectId) {
        ApplicationWindow::window_of(win_id).high_load_request(true);

        self.win_id = win_id;
        self.animation_holder = Some(holder);
        self.start_time = Timestamp::now().as_millis();
        self.end_time = self.start_time + self.duration.as_millis() as u64;
        self.state = AnimationState::Playing;
        self.progress = Progress(0.);
    }

    /// Update the animation.
    #[inline]
    pub fn update(&mut self, current_time: u64) {
        if let Some(ref mut holder) = self.animation_holder {
            self.progress = Progress(
                ((current_time as f64 - self.start_time as f64)
                    / (self.end_time as f64 - self.start_time as f64))
                    .min(1.) as f32,
            );

            holder.update(self.progress);

            if self.progress.0 == 1. {
                self.state = AnimationState::Pending;
                ApplicationWindow::window_of(self.win_id).high_load_request(false);
            }
        }
    }

    #[inline]
    pub fn mode(&self) -> AnimationMode {
        self.mode
    }

    #[inline]
    pub fn effect(&self) -> Option<AnimationEffect> {
        self.effect
    }

    #[inline]
    pub fn state(&self) -> AnimationState {
        self.state
    }

    #[inline]
    pub(crate) fn set_state(&mut self, state: AnimationState) {
        self.state = state;
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

    #[inline]
    pub fn shown(&self) -> bool {
        self.shown
    }

    #[inline]
    pub(crate) fn set_shown(&mut self, shown: bool) {
        self.shown = shown
    }

    #[inline]
    pub(crate) fn origin_rect(&self) -> Option<Rect> {
        self.origin_rect
    }

    #[inline]
    pub(crate) fn set_origin_rect(&mut self, rect: Rect) {
        self.origin_rect = Some(rect)
    }

    #[inline]
    pub(crate) fn origin_transparency(&self) -> Option<i32> {
        self.origin_transparency
    }

    #[inline]
    pub(crate) fn set_origin_transparency(&mut self, transparency: i32) {
        self.origin_transparency = Some(transparency)
    }

    #[inline]
    pub(crate) fn pending_clear(&mut self) {
        self.origin_rect = None;
        self.origin_transparency = None;
    }
}

#[reflect_trait]
pub trait Animatable {
    fn set_animation(&mut self, animation: Animation);

    fn animation(&self) -> Animation;

    fn animation_model(&self) -> &AnimationModel;

    fn animation_model_mut(&mut self) -> &mut AnimationModel;
}
