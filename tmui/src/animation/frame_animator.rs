use crate::{primitive::frame::Frame, widget::WidgetImpl};
use std::{cell::RefCell, ptr::NonNull};
use tlib::{nonnull_mut, nonnull_ref, prelude::*};

#[reflect_trait]
#[allow(unused_variables)]
pub trait FrameAnimator: WidgetImpl {
    #[inline]
    fn on_frame(&mut self, frame: Frame) {}
}
pub(crate) type FrameAnimatorHnd = Option<NonNull<dyn FrameAnimator>>;

thread_local! {
    static INSTANCE: RefCell<FrameAnimatorMgr> = RefCell::new(FrameAnimatorMgr::new());
}

pub(crate) struct FrameAnimatorMgr {
    frame_animators: Vec<FrameAnimatorHnd>,
}

impl FrameAnimatorMgr {
    #[inline]
    fn new() -> Self {
        Self {
            frame_animators: vec![],
        }
    }

    #[inline]
    pub(crate) fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&RefCell<FrameAnimatorMgr>) -> R,
    {
        INSTANCE.with(f)
    }

    #[inline]
    pub(crate) fn add_frame_animator(&mut self, frame_animator: &mut dyn FrameAnimator) {
        self.frame_animators.push(NonNull::new(frame_animator))
    }

    #[inline]
    pub(crate) fn remove_frame_animator(&mut self, id: ObjectId) {
        self.frame_animators.retain(|r| nonnull_ref!(r).id() != id);
    }

    #[inline]
    pub(crate) fn process(&mut self, frame: Frame) {
        for frame_animator in self.frame_animators.iter_mut() {
            nonnull_mut!(frame_animator).on_frame(frame)
        }
    }
}
