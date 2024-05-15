use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(ScrollArea))]
pub struct ScrollAreaLayout {}

impl ObjectSubclass for ScrollAreaLayout {
    const NAME: &'static str = "ScrollAreaLayout";
}

impl ObjectImpl for ScrollAreaLayout {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_vexpand(true);
        self.set_hexpand(true);

        let mut label = Label::new(Some("Hello World"));
        label.set_background(Color::CYAN);

        self.set_area(label);
    }
}

impl WidgetImpl for ScrollAreaLayout {}

impl ScrollAreaLayout {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
