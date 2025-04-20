use derivative::Derivative;
use tlib::object::ObjectSubclass;
use tmui::{label::Label, prelude::*};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct HBoxLayout {
    #[children]
    label_1: Tr<Label>,

    #[children]
    label_2: Tr<Label>,

    #[children]
    label_3: Tr<Label>,
}

impl ObjectSubclass for HBoxLayout {
    const NAME: &'static str = "HBoxLayout";
}

impl ObjectImpl for HBoxLayout {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_homogeneous(true);

        self.label_1.set_background(Color::RED);
        self.label_1.set_content_halign(Align::End);
        self.label_1.set_margin_top(10);
        self.label_1.set_margin_left(5);

        self.label_2.set_background(Color::BLUE);
        self.label_2.set_content_halign(Align::End);
        self.label_2.set_margin_top(10);
        self.label_2.set_margin_left(5);

        self.label_3.set_background(Color::YELLOW);
        self.label_3.set_content_halign(Align::End);
        self.label_3.set_margin_top(10);
        self.label_3.set_margin_left(5);

        self.label_1.set_text("Label 1");
        self.label_2.set_text("Label 2");
        self.label_3.set_text("Label 3");
    }
}

impl WidgetImpl for HBoxLayout {}
