use tlib::run_after;
use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget)]
#[run_after]
pub struct RunAfterWidget {}

impl ObjectSubclass for RunAfterWidget {
   const NAME: &'static str = "RunAfterWidget";
}

impl ObjectImpl for RunAfterWidget {}

impl WidgetImpl for RunAfterWidget {
    fn run_after(&mut self) {
        self.parent_run_after();

        println!("Execute funtion `run_after()`")
    }
}

impl RunAfterWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}