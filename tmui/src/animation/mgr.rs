use super::snapshot::Snapshot;
use crate::primitive::frame::Frame;
use std::{cell::RefCell, ptr::NonNull};
use tlib::nonnull_mut;

thread_local! {
    static INSTANCE: RefCell<AnimationMgr> = RefCell::new(AnimationMgr::new());
}

pub(crate) struct AnimationMgr {
    snapshots: Vec<Option<NonNull<dyn Snapshot>>>,
}

impl AnimationMgr {
    #[inline]
    fn new() -> Self {
        Self { snapshots: vec![] }
    }

    #[inline]
    pub(crate) fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&RefCell<AnimationMgr>) -> R,
    {
        INSTANCE.with(f)
    }

    #[inline]
    pub(crate) fn add_snapshot(&mut self, snapshot: &mut dyn Snapshot) {
        self.snapshots.push(NonNull::new(snapshot))
    }

    #[inline]
    pub(crate) fn process(&mut self, frame: Frame) {
        for snapshot in self.snapshots.iter_mut() {
            nonnull_mut!(snapshot).snapshot(frame)
        }
    }
}
