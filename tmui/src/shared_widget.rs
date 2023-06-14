use crate::{
    application,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct SharedWidget {}

impl ObjectSubclass for SharedWidget {
    const NAME: &'static str = "SharedWidget";
}

impl ObjectImpl for SharedWidget {
    fn construct(&mut self) {
        self.parent_construct();

        if !application::is_shared() {
            panic!("`SharedWidget` can only used in shared memory mode.");
        }
    }
}

impl WidgetImpl for SharedWidget {}
