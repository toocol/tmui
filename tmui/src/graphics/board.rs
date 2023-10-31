use tlib::nonnull_mut;

use super::{drawing_context::DrawingContext, element::ElementImpl};
use crate::{backend::Backend, skia_safe::Surface, primitive::bitmap::Bitmap};
use std::{
    cell::{RefCell, RefMut},
    ptr::NonNull,
    sync::Once,
};

thread_local! {static NOTIFY_UPDATE: RefCell<bool> = RefCell::new(true)}
static ONCE: Once = Once::new();

/// Basic drawing Board with Skia surface.
///
/// Board contains all the Elements.
///
/// Board contains a renderer method `invalidate_visual`, every frame will call this function automaticly and redraw the invalidated element.
/// (All elements call it's `update()` method can set it's `invalidate` field to true, or call `force_update()` to invoke `invalidate_visual` directly)
pub struct Board {
    bitmap: Bitmap,
    backend: Box<dyn Backend>,
    surface: RefCell<Surface>,
    element_list: RefCell<Vec<Option<NonNull<dyn ElementImpl>>>>,
}

impl Board {
    #[inline]
    pub fn new(bitmap: Bitmap, backend: Box<dyn Backend>) -> Self {
        if ONCE.is_completed() {
            panic!("`Board can only construct once.`")
        }
        ONCE.call_once(|| {});

        let surface = backend.surface();

        Self {
            bitmap,
            backend,
            surface: RefCell::new(surface),
            element_list: RefCell::new(vec![]),
        }
    }

    #[inline]
    pub fn notify_update() {
        NOTIFY_UPDATE.with(|notify_update| *notify_update.borrow_mut() = true)
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.bitmap.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.bitmap.height()
    }

    #[inline]
    pub fn resize(&mut self, bitmap: Bitmap) {
        self.surface().flush_submit_and_sync_cpu();
        self.backend
            .resize(bitmap.width() as i32, bitmap.height() as i32);

        self.surface = RefCell::new(self.backend.surface());
        self.bitmap = bitmap;
    }

    #[inline]
    pub fn add_element(&self, element: *mut dyn ElementImpl) {
        self.element_list.borrow_mut().push(NonNull::new(element))
    }

    #[inline]
    pub(crate) fn surface(&self) -> RefMut<Surface> {
        self.surface.borrow_mut()
    }

    #[inline]
    pub fn set_surface(&mut self, surface: Surface) {
        self.surface = RefCell::new(surface);
    }

    #[inline]
    pub(crate) fn invalidate_visual(&self) -> bool {
        NOTIFY_UPDATE.with(|notify_update| {
            let mut update = false;
            if *notify_update.borrow() {
                // The parent elements always at the end of `element_list`.
                // We should renderer the parent elements first.
                for element in self.element_list.borrow_mut().iter_mut() {
                    let element = nonnull_mut!(element);
                    if element.invalidate() {
                        let cr = DrawingContext::new(self);
                        element.on_renderer(&cr);
                        element.validate();
                        element.clear_region();
                        element.clear_region_f();
                        update = true;
                    }
                }

                self.surface().read_pixels(
                    self.backend.image_info(),
                    self.bitmap.get_pixels(),
                    self.bitmap.row_bytes(),
                    (0, 0),
                );

                *notify_update.borrow_mut() = false
            }
            update
        })
    }
}
