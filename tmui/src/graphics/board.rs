use tipc::{RwLock, lock_api::RwLockWriteGuard, RawRwLock};
use tlib::{nonnull_mut, ptr_ref};
use super::{drawing_context::DrawingContext, element::ElementImpl};
use crate::{backend::Backend, skia_safe::Surface, primitive::bitmap::Bitmap};
use std::{
    cell::{RefCell, RefMut},
    ptr::NonNull,
    sync::{Once, Arc},
};

thread_local! {
    static NOTIFY_UPDATE: RefCell<bool> = RefCell::new(true);
    static FORCE_UPDATE: RefCell<bool> = RefCell::new(false);
}
static ONCE: Once = Once::new();

/// Basic drawing Board with Skia surface.
///
/// Board contains all the Elements.
///
/// Board contains a renderer method `invalidate_visual`, every frame will call this function automaticly and redraw the invalidated element.
/// (All elements call it's `update()` method can set it's `invalidate` field to true, or call `force_update()` to invoke `invalidate_visual` directly)
pub struct Board {
    bitmap: Arc<RwLock<Bitmap>>,
    backend: Box<dyn Backend>,
    surface: RefCell<Surface>,
    element_list: RefCell<Vec<Option<NonNull<dyn ElementImpl>>>>,
}

impl Board {
    #[inline]
    pub(crate) fn new(bitmap: Arc<RwLock<Bitmap>>, backend: Box<dyn Backend>) -> Self {
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
    pub fn force_update() {
        FORCE_UPDATE.with(|force_update| *force_update.borrow_mut() = true)
    }

    #[inline]
    pub fn is_force_update() -> bool {
        FORCE_UPDATE.with(|force_update| *force_update.borrow_mut())
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.bitmap.read().width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.bitmap.read().height()
    }

    #[inline]
    pub(crate) fn resize(&mut self) {
        self.surface().flush_submit_and_sync_cpu();
        self.backend
            .resize(self.bitmap.clone());

        self.surface = RefCell::new(self.backend.surface());
    }

    #[inline]
    pub(crate) fn add_element(&self, element: *mut dyn ElementImpl) {
        self.element_list.borrow_mut().push(NonNull::new(element))
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
                let mut bitmap_guard = self.bitmap.write();
                
                // Invoke `ipc_write()` here, so that the following code can 
                // be executed with cross process lock,
                // when program was under cross process rendering.
                let _guard = ptr_ref!(&bitmap_guard as *const RwLockWriteGuard<'_, RawRwLock, Bitmap>).ipc_write();

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

                // let row_bytes = bitmap_guard.row_bytes();
                // let pixels = bitmap_guard.get_pixels_mut();
                // self.surface().read_pixels(
                //     self.backend.image_info(),
                //     pixels,
                //     row_bytes,
                //     (0, 0),
                // );

                bitmap_guard.prepared();

                *notify_update.borrow_mut() = false;
                FORCE_UPDATE.with(|force_update| *force_update.borrow_mut() = false);
            }
            update
        })
    }
}
