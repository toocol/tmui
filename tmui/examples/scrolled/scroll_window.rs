use derivative::Derivative;
use tmui::{
    label::Label,
    prelude::*,
    scroll_area::ScrollArea,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct ScrollWindow {}

impl ObjectSubclass for ScrollWindow {
    const NAME: &'static str = "ScrollWindow";
}

impl ObjectImpl for ScrollWindow {
    fn construct(&mut self) {
        self.parent_construct();

        let mut label = Label::new(Some("Hello World!"));
        label.set_background(Color::CYAN);
        label.set_halign(Align::Center);
        label.set_valign(Align::Center);
        label.width_request(200);
        label.height_request(40);
        label.set_content_halign(Align::End);
        label.set_content_valign(Align::End);
        label.set_size(30);

        let mut scroll_area: ScrollArea = Object::new(&[]);
        scroll_area.set_area(label);

        self.child(scroll_area);
    }
}

impl WidgetImpl for ScrollWindow {}
