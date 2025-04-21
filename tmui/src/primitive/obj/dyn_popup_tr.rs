use super::{Tr, TrAllocater};
use crate::popup::PopupImpl;
use crate::prelude::*;
use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq)]
pub struct DynPopupTr {
    raw: *mut dyn PopupImpl,
    ref_count: Rc<Cell<i32>>,
}

impl DynPopupTr {
    #[inline]
    pub(crate) fn new_directly(raw: *mut dyn PopupImpl, ref_count: Rc<Cell<i32>>) -> Self {
        ref_count.set(ref_count.get() + 1);

        Self { raw, ref_count }
    }

    /// # SAFETY
    ///
    /// Destruction of the underlying object is managed by reference counting, and exceptions should never occur.
    #[inline]
    pub fn bind(&self) -> &dyn PopupImpl {
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
    pub fn bind_mut(&mut self) -> &mut dyn PopupImpl {
        unsafe {
            self.raw
                .as_mut()
                .expect("Fatal error, try to access the removed reference.")
        }
    }

    #[inline]
    pub fn get_ref_count(&self) -> usize {
        self.ref_count.get() as usize
    }
}

impl<R: PopupImpl> From<Tr<R>> for DynPopupTr {
    #[inline]
    fn from(mut value: Tr<R>) -> Self {
        let ref_count = value.clone_ref_count();
        ref_count.set(ref_count.get() + 1);

        let widget = value.as_mut();
        let raw = cast_mut!(widget as PopupImpl).unwrap();

        Self { raw, ref_count }
    }
}

impl Clone for DynPopupTr {
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

impl Drop for DynPopupTr {
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

impl Deref for DynPopupTr {
    type Target = dyn PopupImpl;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.bind()
    }
}

impl DerefMut for DynPopupTr {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.bind_mut()
    }
}

impl AsRef<dyn PopupImpl> for DynPopupTr {
    #[inline]
    fn as_ref(&self) -> &dyn PopupImpl {
        self.bind()
    }
}

impl AsMut<dyn PopupImpl> for DynPopupTr {
    #[inline]
    fn as_mut(&mut self) -> &mut dyn PopupImpl {
        self.bind_mut()
    }
}
