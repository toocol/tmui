use std::{cell::RefCell, ptr::NonNull};
use tlib::{nonnull_ref, object::ObjectId};
use super::InputEle;

type InputEleHnd = Option<NonNull<dyn InputEle>>;

thread_local! {
    static INSTANCE: RefCell<FocusMgr> = RefCell::new(FocusMgr::default());
}

#[derive(Default)]
pub(crate) struct FocusMgr {
    eles: Vec<InputEleHnd>,
    current: Option<usize>,
}

impl FocusMgr {
    #[inline]
    pub(crate) fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&RefCell<FocusMgr>) -> R,
    {
        INSTANCE.with(f)
    }

    pub(crate) fn add(&mut self, ele: &mut dyn InputEle) {
        let mut shuffle = false;
        if let Some(last) = self.eles.last() {
            let last = nonnull_ref!(last);
            if last.tabindex() > ele.tabindex() {
                shuffle = true;
            }
        }

        self.eles.push(NonNull::new(ele));

        if shuffle {
            self.eles.sort_by(|a, b| {
                let a = nonnull_ref!(a).tabindex();
                let b = nonnull_ref!(b).tabindex();
                a.cmp(&b)
            })
        }
    }

    /// Should pre-check the widget is `InputEle`.
    pub(crate) fn set_currrent(&mut self, id: ObjectId) {
        for (idx, ele) in self.eles.iter().enumerate() {
            let ele = nonnull_ref!(ele);
            if id == ele.id() {
                self.current = Some(idx);
                break;
            }
        }
    }

    #[inline]
    pub(crate) fn get_current(&self) -> Option<usize> {
        self.current
    }

    #[inline]
    pub(crate) fn clear_current(&mut self) {
        self.current = None;
    }

    pub(crate) fn next(&mut self) {
        if self.current.is_none() {
            return;
        }

        let current = self.current.unwrap();
        if current == self.eles.len() - 1 {
            self.current = Some(0)
        } else {
            self.current = Some(current + 1)
        }
    }
}