use std::cell::RefCell;

use skia_safe::Surface;

use super::{drawing_context::DrawingContext, element::ElementImpl};

/// Basic drawing Board with Skia surface.
///
/// Board contains all the Elements.
///
/// Board contains a renderer method `invalidate_visual`, every frame will call this function automaticly and redraw the invalidated element.
/// (All call Elments' `update()` method can set element's `invalidate` filed to true, or call `force_update()` to invoke `invalidate_visual` directly)
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub surface: RefCell<Surface>,
    pub element_list: Vec<Box<dyn ElementImpl>>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        let surface =
            Surface::new_raster_n32_premul((width, height)).expect("No Skia surface available.");
        Self {
            width,
            height,
            surface: RefCell::new(surface),
            element_list: vec![],
        }
    }

    pub fn invalidate_visual(&self) {
        for element in self.element_list.iter() {
            if element.invalidate() {
                let cr = DrawingContext::new(self, element.point());
                element.on_renderer(&cr);
                element.validate();
            }
        }
    }
}
