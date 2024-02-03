use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Childable)]
pub struct ChildAttrWidget {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for ChildAttrWidget {
    const NAME: &'static str = "ChildAttrWidget";
}

impl ObjectImpl for ChildAttrWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.label.set_text("Hello, World!");
        self.label.set_background(Color::from_rgb(100, 100, 100));
    }
}

impl WidgetImpl for ChildAttrWidget {}

impl ChildAttrWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
