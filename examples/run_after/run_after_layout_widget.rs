use tlib::run_after;
use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[run_after]
pub struct RunAfterLayoutWidget {
    #[children]
    label: Tr<Label>,
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
        println!("`RunAfterLayoutWidget` run after.")
    }
}

impl RunAfterLayoutWidget {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}
