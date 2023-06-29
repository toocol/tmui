use tlib::run_after;
use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl, label::Label,
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[run_after]
pub struct RunAfterLayoutWidget {
    #[children]
    label: Box<Label>
}

impl ObjectSubclass for RunAfterLayoutWidget {
   const NAME: &'static str = "RunAfterLayoutWidget";
}

impl ObjectImpl for RunAfterLayoutWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.label.set_text("Run after.");
        self.label.set_size(50);
        self.label.set_content_halign(Align::Center);
        self.label.set_content_valign(Align::Center);
    }
}

impl WidgetImpl for RunAfterLayoutWidget {
    fn run_after(&mut self) {
        self.parent_run_after();

        println!("`RunAfterLayoutWidget` run after.")
    }
}

impl RunAfterLayoutWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}