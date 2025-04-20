use super::TrAllocater;
use crate::widget::WidgetImpl;
use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Tr<R: WidgetImpl> {
    raw: *mut R,
    ref_count: Rc<Cell<i32>>,
}

impl<R: WidgetImpl> Tr<R> {
    #[inline]
    pub(crate) fn from_raw(raw: *mut R) -> Tr<R> {
        Self {
            raw,
            ref_count: Rc::new(Cell::new(1)),
        }
    }

    #[inline]
    pub(crate) fn new_directly(raw: *mut R, ref_count: Rc<Cell<i32>>) -> Tr<R> {
        Self { raw, ref_count }
    }

    #[inline]
    pub(crate) fn clone_ref_count(&self) -> Rc<Cell<i32>> {
        self.ref_count.clone()
    }

    /// # SAFETY
    ///
    /// Destruction of the underlying object is managed by reference counting, and exceptions should never occur.
    #[inline]
    pub fn bind(&self) -> &R {
        unsafe {
            self.raw
                .as_ref()
                .expect("Fatal error, try to access the removed reference.")
        }
    }

    /// # SAFETY
    ///
    /// Destruction of the underlying object is managed by reference counting, and exceptions should never occur.
    #[inline]
    pub fn bind_mut(&mut self) -> &mut R {
        unsafe {
            self.raw
                .as_mut()
                .expect("Fatal error, try to access the removed reference.")
        }
    }

    #[inline]
    pub fn as_dyn(&self) -> &dyn WidgetImpl {
        self.bind() as &dyn WidgetImpl
    }

    #[inline]
    pub fn as_dyn_mut(&mut self) -> &mut dyn WidgetImpl {
        self.bind_mut() as &mut dyn WidgetImpl
    }

    #[inline]
    pub fn get_ref_count(&self) -> usize {
        self.ref_count.get() as usize
    }
}

impl<R: WidgetImpl> Clone for Tr<R> {
    #[inline]
    fn clone(&self) -> Self {
        let ref_count = self.ref_count.clone();
        ref_count.set(ref_count.get() + 1);

        Self {
            raw: self.raw,
            ref_count,
        }
    }
}

impl<R: WidgetImpl> Drop for Tr<R> {
    #[inline]
    fn drop(&mut self) {
        self.ref_count.set(self.ref_count.get() - 1);

        let ref_cnt = self.ref_count.get();
        debug_assert!(ref_cnt >= 0);

        if ref_cnt == 0 {
            TrAllocater::remove(self.id());
        }
    }
}

impl<R: WidgetImpl> Deref for Tr<R> {
    type Target = R;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.bind()
    }
}

impl<R: WidgetImpl> DerefMut for Tr<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.bind_mut()
    }
}
