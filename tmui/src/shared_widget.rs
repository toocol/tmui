use std::sync::atomic::Ordering;

use crate::{
    application::{self, PLATFORM_CONTEXT},
    prelude::*,
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        run_after,
    },
    widget::WidgetImpl,
};

#[extends(Widget)]
#[run_after]
pub struct SharedWidget {}

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
        let platform_context = unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_mut().unwrap() };
        platform_context.add_shared_region(self.rect());
    }
}
