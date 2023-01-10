use super::{drawing_context::DrawingContext, element::ElementImpl};
use skia_safe::Surface;
use std::{cell::RefCell, ptr::NonNull};

/// Basic drawing Board with Skia surface.
///
/// Board contains all the Elements.
///
/// Board contains a renderer method `invalidate_visual`, every frame will call this function automaticly and redraw the invalidated element.
/// (All elements call it's `update()` method can set it's `invalidate` field to true, or call `force_update()` to invoke `invalidate_visual` directly)
pub struct Board {
    pub surface: RefCell<Surface>,
    pub element_list: Vec<Option<NonNull<dyn ElementImpl>>>,
}

impl Board {
    pub fn new(surface: Surface) -> Self {
        Self {
            surface: RefCell::new(surface),
            element_list: vec![],
        }
    }

    pub fn add_element(&mut self, element: *mut dyn ElementImpl) {
        self.element_list
            .push(NonNull::new(element))
    }

    pub fn invalidate_visual(&self) -> bool {
        let mut update = false;
        for element in self.element_list.iter() {
            unsafe {
                let element = element.as_ref().unwrap().as_ref();
                if element.invalidate() {
                    let cr = DrawingContext::new(self, element.rect());
                    element.on_renderer(&cr);
                    element.validate();
                    update = true;
                }
            }
        }
        update
    }
}
