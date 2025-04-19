use super::Tr;
use crate::widget::WidgetImpl;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DynTr {
    raw: *mut dyn WidgetImpl,
}

impl DynTr {
    /// # SAFETY
    ///
    /// The raw pointer becomes invalid when the widget is removed.
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
    /// The raw pointer becomes invalid when the widget is removed.
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
    fn from(mut value: Tr<R>) -> Self {
        let dyn_mut = value.as_dyn_mut();

        Self { raw: dyn_mut }
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
