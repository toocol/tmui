use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::{right_panel::RightPanel, LeftPanel};

#[extends(Widget, Layout(Pane))]
#[derive(Childrenable)]
pub struct CentralPanel {
    #[children]
    widget1: Tr<LeftPanel>,

    #[children]
    widget2: Tr<RightPanel>,
}

impl ObjectSubclass for CentralPanel {
    const NAME: &'static str = "CentralPanel";
}

impl ObjectImpl for CentralPanel {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_strict_children_layout(true);

        self.widget2.set_background(Color::GREY_MEDIUM);

        self.widget2.set_hexpand(true);
        self.widget2.set_vexpand(true);
    }
}

impl WidgetImpl for CentralPanel {}
