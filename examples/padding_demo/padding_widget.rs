use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Childable)]
pub struct PaddingWidget {
    #[child]
    child: Tr<Widget>,
}

impl ObjectSubclass for PaddingWidget {
   const NAME: &'static str = "PaddingWidget";
}

impl ObjectImpl for PaddingWidget {
    fn initialize(&mut self) {
        self.set_background(Color::RED);
        self.width_request(500);
        self.height_request(500);
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);
        self.set_paddings(10, 10, 10, 10);

        self.child.set_background(Color::BLUE);
        self.child.set_hexpand(true);
        self.child.set_vexpand(true);
    }
}

impl WidgetImpl for PaddingWidget {}