use tlib::run_after;
use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};
use crate::run_after_layout_widget::RunAfterLayoutWidget;

#[extends(Widget)]
#[run_after]
pub struct RunAfterWidget {}

impl ObjectSubclass for RunAfterWidget {
   const NAME: &'static str = "RunAfterWidget";
}

impl ObjectImpl for RunAfterWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.child(RunAfterLayoutWidget::new());
        self.set_valign(Align::Center);
        self.set_halign(Align::Center);
    }
}

impl WidgetImpl for RunAfterWidget {
    fn run_after(&mut self) {
        self.parent_run_after();

        println!("`RunAfterWidget` run after.")
    }
}

impl RunAfterWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}