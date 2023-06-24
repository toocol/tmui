use std::sync::atomic::{AtomicU8, Ordering};
use crate::{
    application,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

static COUNTER: AtomicU8 = AtomicU8::new(0);

#[extends(Widget)]
pub struct SharedWidget {}

impl ObjectSubclass for SharedWidget {
    const NAME: &'static str = "SharedWidget";
}

impl ObjectImpl for SharedWidget {
    fn construct(&mut self) {
        if !application::is_shared() {
            panic!("`SharedWidget` can only used in shared memory application.");
        }

        if COUNTER.load(Ordering::Acquire) > 0 {
            panic!("Only support one `SharedWidget` in application.");
        }
        COUNTER.fetch_add(1, Ordering::Release);

        self.parent_construct();
    }
}

impl WidgetImpl for SharedWidget {}
