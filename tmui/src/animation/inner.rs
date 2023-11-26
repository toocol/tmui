use std::{
    ops::{Add, Div, Mul, Sub},
    ptr::NonNull,
};
use tlib::{
    figure::{Color, Rect},
    global::CreateBy,
    nonnull_mut,
};
use super::progress::Progress;

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
            + CreateBy<f32>
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
                let ft = t * t * (P::create_by(3.) + P::create_by(2.) * t);
                start + ft * (end - start)
            }
            Self::EaseOut => {
                let tp = P::create_by(1.) - t;
                let ft = P::create_by(1.) - tp * tp * tp;
                start + ft * (end - start)
            }
        }
    }
}

/// Should not be used directly, this structure will be processed by macros.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AnimationsHolder {
    Linear {
        start: Rect,
        end: Rect,
        hold: Option<NonNull<Rect>>,
    },
    EaseIn {
        start: Rect,
        end: Rect,
        hold: Option<NonNull<Rect>>,
    },
    EaseOut {
        start: Rect,
        end: Rect,
        hold: Option<NonNull<Rect>>,
    },
    FadeLinear {
        start: Color,
        end: Color,
        hold: Option<NonNull<Color>>,
    },
    FadeEaseIn {
        start: Color,
        end: Color,
        hold: Option<NonNull<Color>>,
    },
    FadeEaseOut {
        start: Color,
        end: Color,
        hold: Option<NonNull<Color>>,
    },
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
