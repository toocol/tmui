use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(SharedWidget, id = "shmem_widget")]
pub struct MasterSharedWidget {}

impl ObjectSubclass for MasterSharedWidget {
    const NAME: &'static str = "MasterSharedWidget";
}

impl ObjectImpl for MasterSharedWidget {}

impl WidgetImpl for MasterSharedWidget {}

impl MasterSharedWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
