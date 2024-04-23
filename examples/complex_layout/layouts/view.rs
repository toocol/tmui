use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::{CentralPanel, StatusBar, TitleBar};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct View {
    #[children]
    title_bar: Box<TitleBar>,

    #[children]
    central_panel: Box<CentralPanel>,

    #[children]
    status_bar: Box<StatusBar>,
}

impl ObjectSubclass for View {
    const NAME: &'static str = "View";
}

impl ObjectImpl for View {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);
    }
}

impl WidgetImpl for View {}

impl View {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
