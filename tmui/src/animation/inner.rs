use super::progress::Progress;
use std::{
    ops::{Add, Div, Mul, Sub},
    ptr::NonNull,
};
use tlib::{
    figure::FRect,
    nonnull_mut,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum InnerAnimations {
    Linear,
    // At the beginning of the animation, the acceleration is slow, and then gradually increases
    EaseIn,
    // Slow deceleration at the end of the animation, faster speed at the beginning of the animation
    EaseOut,
}

impl InnerAnimations {
    /// The interpolation function to calculate the animation's middle value.
    ///
    /// t: time progress parameter, the value was between 0-1
    /// start: animation's start state (e.g. Point,Rect,Color...)
    /// end: animation's end state (e.g. Point,Rect,Color...)
    pub fn interpolation<
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
        P: Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + Add<P, Output = P>
            + Sub<P, Output = P>
            + Mul<P, Output = P>
            + Div<P, Output = P>
            + From<f32>
            + Copy,
    >(
        &self,
        t: P,
        start: T,
        end: T,
    ) -> T {
        match self {
            Self::Linear => start + t * (end - start),
            Self::EaseIn => {
                let ft = t * t * t;
                start + ft * (end - start)
            }
            Self::EaseOut => {
                let tp = P::from(1.) - t;
                let ft = P::from(1.) - tp * tp * tp;
                start + ft * (end - start)
            }
        }
    }
}

/// Should not be used directly, this structure will be processed by macros.
#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationsHolder {
    Linear { start: FRect, end: FRect, hold: Option<NonNull<FRect>> },

    EaseIn { start: FRect, end: FRect, hold: Option<NonNull<FRect>> },

    EaseOut { start: FRect, end: FRect, hold: Option<NonNull<FRect>> },

    FadeLinear { start: i32, end: i32, hold: Option<NonNull<i32>> },

    FadeEaseIn { start: i32, end: i32, hold: Option<NonNull<i32>> },

    FadeEaseOut { start: i32, end: i32, hold: Option<NonNull<i32>> },
}

macro_rules! interpolation {
    ( $inner_animation:expr, $t:expr, $start:expr, $end:expr, $hold:expr ) => {
        if $t.0 == 1. {
            *nonnull_mut!($hold) = *$end;
        } else {
            *nonnull_mut!($hold) = $inner_animation.interpolation($t, *$start, *$end);
        }
    };
}

impl AnimationsHolder {
    #[inline]
    pub(crate) fn inner_animation(&self) -> InnerAnimations {
        match self {
            Self::Linear { .. } => InnerAnimations::Linear,
            Self::EaseIn { .. } => InnerAnimations::EaseIn,
            Self::EaseOut { .. } => InnerAnimations::EaseOut,
            Self::FadeLinear { .. } => InnerAnimations::Linear,
            Self::FadeEaseIn { .. } => InnerAnimations::EaseIn,
            Self::FadeEaseOut { .. } => InnerAnimations::EaseOut,
        }
    }

    pub(crate) fn update(&mut self, t: Progress) {
        let inner_animation = self.inner_animation();
        match self {
            Self::Linear { start, end, hold } => {
                interpolation!(inner_animation, t, start, end, hold);
            }
            Self::EaseIn { start, end, hold } => {
                interpolation!(inner_animation, t, start, end, hold);
            }
            Self::EaseOut { start, end, hold } => {
                interpolation!(inner_animation, t, start, end, hold);
            }
            Self::FadeLinear { start, end, hold } => {
                interpolation!(inner_animation, t, start, end, hold);
            }
            Self::FadeEaseIn { start, end, hold } => {
                interpolation!(inner_animation, t, start, end, hold);
            }
            Self::FadeEaseOut { start, end, hold } => {
                interpolation!(inner_animation, t, start, end, hold);
            }
        };
    }
}
