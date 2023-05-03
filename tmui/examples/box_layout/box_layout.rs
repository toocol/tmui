use tlib::object::ObjectSubclass;
use tmui::{prelude::*, label::Label};

#[extends(Widget, Layout(VBox))]
#[derive(Default, Childrenable)]
pub struct BoxLayout {
    #[children]
    label_1: Label,
    #[children]
    label_2: Label,
    #[children]
    label_3: Label,
}

impl ObjectSubclass for BoxLayout {
    const NAME: &'static str = "BoxLayout";
}

impl ObjectImpl for BoxLayout {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_homogeneous(true);

        self.label_1.set_background(Color::RED);
        self.label_1.set_margin_top(10);

        self.label_2.set_background(Color::BLUE);
        self.label_2.set_margin_top(10);

        self.label_3.set_background(Color::YELLOW);
        self.label_3.set_margin_top(10);
    }

    fn initialize(&mut self) {
        self.label_1.set_text("Label 1");
        self.label_2.set_text("Label 2");
        self.label_3.set_text("Label 3");
    }
}

impl WidgetImpl for BoxLayout {}

impl BoxLayout {
    pub fn new() -> Self {
        Object::new(&[])
    }
}