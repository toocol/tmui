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
pub struct SharedWidget {
    shared_type: SharedType,
    shared_id: &'static str,
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
        self.parent_run_after();

        if self.shared_type == SharedType::Master {
            let platform_context =
                unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_mut().unwrap() };
            platform_context.add_shared_region(self.shared_id(), self.rect());
        }
    }
}

pub trait SharedWidgetExt {
    fn shared_type(&self) -> SharedType;

    fn set_shared_type(&mut self, shared_type: SharedType);

    fn shared_id(&self) -> &'static str;

    fn set_shared_id(&mut self, id: &'static str);
}

impl SharedWidgetExt for SharedWidget {
    #[inline]
    fn shared_type(&self) -> SharedType {
        self.shared_type
    }

    #[inline]
    fn set_shared_type(&mut self, shared_type: SharedType) {
        self.shared_type = shared_type
    }

    #[inline]
    fn shared_id(&self) -> &'static str {
        self.shared_id
    }

    #[inline]
    fn set_shared_id(&mut self, id: &'static str) {
        self.shared_id = id
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SharedType {
    #[default]
    Master,
    Slave,
}
