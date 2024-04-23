use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(Pane))]
#[derive(Childrenable)]
pub struct CentralPanel {
    #[children]
    widget1: Box<Widget>,

    #[children]
    widget2: Box<Widget>,
}

impl ObjectSubclass for CentralPanel {
    const NAME: &'static str = "CentralPanel";
}

impl ObjectImpl for CentralPanel {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_strict_children_layout(true);

        self.widget1.set_background(Color::GREY_DARK);
        self.widget2.set_background(Color::GREY_MEDIUM);

        self.widget1.width_request(300);
        self.widget1.set_vexpand(true);

        self.widget2.set_hexpand(true);
        self.widget2.set_vexpand(true);
    }
}

impl WidgetImpl for CentralPanel {}