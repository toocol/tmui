use super::{drawing_context::DrawingContext, element::{ElementImpl, HierachyZ}};
use crate::{
    backend::Backend, opti::tracker::Tracker, primitive::{bitmap::Bitmap, frame::Frame},
    shared_widget::ReflectSharedWidgetImpl, skia_safe::Surface,
};
use std::{cell::RefCell, ptr::NonNull, sync::Arc};
use tipc::{
    parking_lot::RwLock,
    parking_lot::{lock_api::RwLockWriteGuard, RawRwLock},
};
use tlib::{nonnull_mut, nonnull_ref, prelude::*, ptr_ref};

thread_local! {
    static NOTIFY_UPDATE: RefCell<bool> = const { RefCell::new(true) };
    static FORCE_UPDATE: RefCell<bool> = const { RefCell::new(false) };
}

/// Basic drawing Board with Skia surface.
///
/// Board contains all the Elements.
///
/// Board contains a renderer method `invalidate_visual`, every frame will call this function automaticly and redraw the invalidated element.
/// (All elements call it's `update()` method can set it's `invalidate` field to true, or call `force_update()` to invoke `invalidate_visual` directly)
pub struct Board {
    bitmap: Arc<RwLock<Bitmap>>,
    backend: Box<dyn Backend>,
    surface: Surface,
    element_list: RefCell<Vec<Option<NonNull<dyn ElementImpl>>>>,
}

impl Board {
    #[inline]
    pub(crate) fn new(bitmap: Arc<RwLock<Bitmap>>, backend: Box<dyn Backend>) -> Self {
        let surface = backend.surface();

        Self {
            bitmap,
            backend,
            surface,
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
        // self.surface().flush_and_submit();
        self.backend.resize(self.bitmap.clone());

        self.surface = self.backend.surface();
    }

    #[inline]
    pub(crate) fn add_element(&self, element: &mut dyn ElementImpl) {
        self.element_list.borrow_mut().push(NonNull::new(element))
    }

    #[inline]
    pub(crate) fn shuffle(&self) {
        self.element_list.borrow_mut().sort_by(|a, b| {
            let a = nonnull_ref!(a).z_index();
            let b = nonnull_ref!(b).z_index();
            a.cmp(&b)
        });
    }

    #[inline]
    pub(crate) fn invalidate_visual(&mut self, frame: Frame) -> bool {
        NOTIFY_UPDATE.with(|notify_update| {
            let mut update = false;

            let mut bitmap_guard = self.bitmap.write();

            if *notify_update.borrow() || bitmap_guard.is_shared_invalidate() {
                let _track = Tracker::start("invalidate_visual");

                // Invoke `ipc_write()` here, so that the following code can
                // be executed with cross process lock,
                // when program was under cross process rendering.
                //
                // This lock only effect on slave side of shared memory application here,
                // shared buffer will be locked in [`SharedWidgetExt::pixels_render()`] on master side.
                let _guard =
                    ptr_ref!(&bitmap_guard as *const RwLockWriteGuard<'_, RawRwLock, Bitmap>)
                        .ipc_write();

                // The parent elements always at the begining of `element_list`.
                // We should renderer the parent elements first.
                for element in self.element_list.borrow_mut().iter_mut() {
                    let element = nonnull_mut!(element);
                    let shared_widget = cast!(element as SharedWidgetImpl);

                    let need_render = element.invalidate()
                        || (shared_widget.is_some()
                            && shared_widget.as_ref().unwrap().is_shared_invalidate());

                    if need_render {
                        let cr = DrawingContext::new(self.surface.canvas(), frame);

                        if let Some(shared_widget) = shared_widget {
                            shared_widget.shared_validate();
                        }

                        element.before_renderer();
                        element.on_renderer(&cr);
                        element.after_renderer();
                        element.validate();
                        update = true;
                    }
                }

                // self.surface().flush_and_submit();

                bitmap_guard.prepared();

                // Only effected on slave side of shared memory application.
                bitmap_guard.set_shared_invalidate(true);

                *notify_update.borrow_mut() = false;
                FORCE_UPDATE.with(|force_update| *force_update.borrow_mut() = false);
            }
            update
        })
    }
}
