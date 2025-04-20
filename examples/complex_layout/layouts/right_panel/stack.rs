use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(Stack))]
#[derive(Childrenable)]
pub struct MyStack {
    #[children]
    label_1: Tr<Label>,

    #[children]
    label_2: Tr<Label>,
}

impl ObjectSubclass for MyStack {
    const NAME: &'static str = "MyStack";
}

impl ObjectImpl for MyStack {
    fn initialize(&mut self) {
        self.label_1.width_request(10);
        self.label_1.height_request(10);

        self.label_2.width_request(10);
        self.label_2.height_request(10);

        self.label_1.hide();
        self.label_2.hide();
    }
}

impl WidgetImpl for MyStack {}
