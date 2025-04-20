use super::{Tr, TrAllocater};
use crate::widget::WidgetImpl;
use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq)]
pub struct DynTr {
    raw: *mut dyn WidgetImpl,
    ref_count: Rc<Cell<i32>>,
}

impl DynTr {
    /// # SAFETY
    ///
    /// Destruction of the underlying object is managed by reference counting, and exceptions should never occur.
    #[inline]
    pub fn bind(&self) -> &dyn WidgetImpl {
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
    pub fn bind_mut(&mut self) -> &mut dyn WidgetImpl {
        unsafe {
            self.raw
                .as_mut()
                .expect("Fatal error, try to access the removed reference.")
        }
    }
}

impl<R: WidgetImpl> From<Tr<R>> for DynTr {
    #[inline]
    fn from(mut value: Tr<R>) -> Self {
        let ref_count = value.clone_ref_count();
        ref_count.set(ref_count.get() + 1);

        Self {
            raw: value.as_dyn_mut(),
            ref_count,
        }
    }
}

impl Clone for DynTr {
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

impl Drop for DynTr {
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

impl Deref for DynTr {
    type Target = dyn WidgetImpl;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.bind()
    }
}

impl DerefMut for DynTr {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.bind_mut()
    }
}
