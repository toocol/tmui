use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl, label::Label,
};

#[extends(Widget)]
pub struct SelfAdaptionWidget {}

impl ObjectSubclass for SelfAdaptionWidget {
   const NAME: &'static str = "SelfAdaptionWidget";
}

impl ObjectImpl for SelfAdaptionWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_background(Color::RED);
        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_hscale(0.4);
        self.set_vscale(0.4);
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);

        let mut label = Label::new(Some("Hello, World!"));
        label.set_background(Color::CYAN);
        label.set_vexpand(true);
        label.set_hexpand(true);
        label.set_hscale(0.85);
        label.set_vscale(0.9);
        label.set_size(40);
        label.set_content_halign(Align::Center);
        label.set_content_valign(Align::Center);
        label.set_padding_left(20);
        label.set_margin_top(50);
        label.set_borders(2., 2., 2., 2.);
        println!("size {:?}", label.size());

        self.child(label)
    }
}

impl WidgetImpl for SelfAdaptionWidget {}

impl SelfAdaptionWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}