use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl, label::Label,
};

use crate::pane_layout::PaneLayout;

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct CentralPane {
    #[children]
    top_label: Box<Label>,

    #[children]
    pane: Box<PaneLayout>,
}

impl ObjectSubclass for CentralPane {
   const NAME: &'static str = "CentralPane";
}

impl ObjectImpl for CentralPane {
    fn initialize(&mut self) {
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_hscale(0.7);
        self.set_vscale(0.7);
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);

        self.top_label.set_text("Top label");
        self.top_label.set_background(Color::CYAN);
        self.top_label.set_hexpand(true);
        self.top_label.height_request(20);
    }
}

impl WidgetImpl for CentralPane {}

impl CentralPane {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}