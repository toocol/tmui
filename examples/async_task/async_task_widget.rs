use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget)]
#[async_task(name = "AsyncTask", value = "i32")]
pub struct AsyncTaskWidget {}

impl ObjectSubclass for AsyncTaskWidget {
   const NAME: &'static str = "AsyncTaskWidget";
}

impl ObjectImpl for AsyncTaskWidget {
    fn construct(&mut self) {
        self.parent_construct();
    }
}

impl WidgetImpl for AsyncTaskWidget {}