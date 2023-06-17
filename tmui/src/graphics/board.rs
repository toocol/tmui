use super::{drawing_context::DrawingContext, element::ElementImpl};
use crate::skia_safe::Surface;
use std::{
    cell::{RefCell, RefMut},
    ptr::NonNull,
};

thread_local! {static NOTIFY_UPDATE: RefCell<bool> = RefCell::new(true)}

/// Basic drawing Board with Skia surface.
///
/// Board contains all the Elements.
///
/// Board contains a renderer method `invalidate_visual`, every frame will call this function automaticly and redraw the invalidated element.
/// (All elements call it's `update()` method can set it's `invalidate` field to true, or call `force_update()` to invoke `invalidate_visual` directly)
pub struct Board {
    surface: RefCell<Surface>,
    element_list: Vec<RefCell<Option<NonNull<dyn ElementImpl>>>>,
}

impl Board {
    #[inline]
    pub fn new(surface: Surface) -> Self {
        Self {
            surface: RefCell::new(surface),
            element_list: vec![],
        }
    }

    #[inline]
    pub fn notify_update() {
        NOTIFY_UPDATE.with(|notify_update| *notify_update.borrow_mut() = true)
    }

    #[inline]
    pub fn add_element(&mut self, element: *mut dyn ElementImpl) {
        self.element_list.push(RefCell::new(NonNull::new(element)))
    }

    #[inline]
    pub(crate) fn surface(&self) -> RefMut<Surface> {
        self.surface.borrow_mut()
    }

    #[inline]
    pub(crate) fn invalidate_visual(&self) -> bool {
        NOTIFY_UPDATE.with(|notify_update| {
            let mut update = false;
            if *notify_update.borrow() {
                // The parent elements always at the end of `element_list`.
                // We should renderer the parent elements first.
                for element in self.element_list.iter() {
                    let element = unsafe { element.borrow_mut().as_mut().unwrap().as_mut() };
                    if element.invalidate() {
                        let cr = DrawingContext::new(self);
                        element.on_renderer(&cr);
                        element.validate();
                        element.clear_region();
                        update = true;
                    }
                }
                *notify_update.borrow_mut() = false
            }
            update
        })
    }
}
