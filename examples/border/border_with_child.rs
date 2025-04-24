use log::info;
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct BorderWithChild {
    #[children]
    widget_left: Tr<Widget>,
    #[children]
    widget_right: Tr<Widget>,
}

impl ObjectSubclass for BorderWithChild {
    const NAME: &'static str = "BorderWithChild";
}

impl ObjectImpl for BorderWithChild {
    fn initialize(&mut self) {
        self.set_background(Color::GREY_LIGHT);
        self.set_halign(Align::Center);
        self.enable_bubble(EventBubble::MOUSE_MOVE);

        self.set_margin_top(5);
        self.set_border_radius(6.);

        self.widget_left.width_request(25);
        self.widget_left.height_request(25);

        self.widget_right.width_request(25);
        self.widget_right.height_request(25);
        self.widget_right.set_mouse_tracking(true);
    }
}

impl WidgetImpl for BorderWithChild {
    fn on_mouse_enter(&mut self, _: &MouseEvent) {
        info!("Mouse entered.");
        self.set_background(Color::GREY_MEDIUM);
    }

    fn on_mouse_leave(&mut self, _: &MouseEvent) {
        info!("Mouse leaved.");
        self.set_background(Color::GREY_LIGHT);
    }
}
