use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

const ID: &'static str = "shmem_widget";

#[extends(SharedWidget)]
pub struct MasterSharedWidget {}

impl ObjectSubclass for MasterSharedWidget {
   const NAME: &'static str = "MasterSharedWidget";
}

impl ObjectImpl for MasterSharedWidget {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_shared_id(ID)
    }
}

impl WidgetImpl for MasterSharedWidget {}

impl MasterSharedWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}