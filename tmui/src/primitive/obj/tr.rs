use crate::widget::WidgetImpl;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tr<R: WidgetImpl> {
    raw: *mut R,
}

impl<R: WidgetImpl> Tr<R> {
    #[inline]
    pub(crate) fn from_raw(raw: *mut R) -> Tr<R> {
        Tr { raw }
    }

    /// # SAFETY
    ///
    /// The raw pointer becomes invalid when the widget is removed.
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
    /// The raw pointer becomes invalid when the widget is removed.
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
