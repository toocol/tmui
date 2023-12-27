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

impl ObjectImpl for MasterSharedWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_vexpand(true);
        self.set_hexpand(true);
    }
}

impl WidgetImpl for MasterSharedWidget {}

impl SharedWidgetImpl for MasterSharedWidget {}

impl MasterSharedWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
