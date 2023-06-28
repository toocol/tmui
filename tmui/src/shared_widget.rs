use std::sync::atomic::Ordering;

use crate::{
    application::{self, PLATFORM_CONTEXT},
    platform::PlatformType,
    prelude::*,
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        run_after,
    },
    widget::WidgetImpl,
};

#[extends(Widget)]
#[run_after]
pub struct SharedWidget {
    shared_id: Option<&'static str>,
}

impl ObjectSubclass for SharedWidget {
    const NAME: &'static str = "SharedWidget";
}

impl ObjectImpl for SharedWidget {
    fn construct(&mut self) {
        if !application::is_shared() {
            panic!("`SharedWidget` can only used in shared memory application.");
        }

        self.parent_construct();
    }
}

impl WidgetImpl for SharedWidget {
    fn run_after(&mut self) {
        if ApplicationWindow::window_of(self.window_id()).platform_type() == PlatformType::Ipc {
            panic!("`SharedWidget` can not be used on `PlatformType::Ipc`")
        }
        self.parent_run_after();

        let platform_context = unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_mut().unwrap() };
        platform_context.add_shared_region(self.shared_id(), self.rect());
    }
}

pub trait SharedWidgetExt {
    fn shared_id(&self) -> &'static str;

    fn set_shared_id(&mut self, id: &'static str);
}

impl SharedWidgetExt for SharedWidget {
    #[inline]
    fn shared_id(&self) -> &'static str {
        self.shared_id.expect("`SharedWidget` should set the `shared_id`")
    }

    #[inline]
    fn set_shared_id(&mut self, id: &'static str) {
        self.shared_id = Some(id)
    }
}
