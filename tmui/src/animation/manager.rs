use super::snapshot::Snapshot;
use crate::primitive::frame::Frame;
use std::{cell::RefCell, ptr::NonNull};
use tlib::nonnull_mut;

thread_local! {
    static INSTANCE: RefCell<AnimationManager> = RefCell::new(AnimationManager::new());
}

pub(crate) struct AnimationManager {
    snapshots: Vec<Option<NonNull<dyn Snapshot>>>,
}

impl AnimationManager {
    #[inline]
    fn new() -> Self {
        Self { snapshots: vec![] }
    }

    #[inline]
    pub(crate) fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&RefCell<AnimationManager>) -> R,
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
