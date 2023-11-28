use derivative::Derivative;
use tlib::{connect, events::MouseEvent};
use tmui::{
    label::Label,
    prelude::*,
    scroll_area::ScrollArea,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl, scroll_bar::ScrollBarPosition,
};

#[extends(Widget)]
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
        // label.width_request(200);
        // label.height_request(40);
        label.set_content_halign(Align::End);
        label.set_content_valign(Align::End);
        label.set_hexpand(true);
        label.set_vexpand(true);
        label.set_size(30);

        connect!(label, mouse_wheel(), self, when_laebl_receive_wheel(MouseEvent));

        let mut scroll_area: Box<ScrollArea> = Object::new(&[]);
        scroll_area.set_area(label);
        // scroll_area.width_request(400);
        // scroll_area.height_request(300);
        scroll_area.set_halign(Align::End);
        scroll_area.set_valign(Align::End);
        scroll_area.set_hexpand(true);
        scroll_area.set_vexpand(true);
        scroll_area.set_hscale(0.7);
        scroll_area.set_vscale(0.7);
        scroll_area.set_background(Color::RED);
        scroll_area.set_scroll_bar_position(ScrollBarPosition::End);

        self.child(scroll_area);
    }
}

impl WidgetImpl for ScrollWindow {}

impl ScrollWindow {
    fn when_laebl_receive_wheel(&self, evt: MouseEvent) {
        let pos = evt.position();
        println!("Label receive wheel event. position: {:?}", pos)
    }
}
