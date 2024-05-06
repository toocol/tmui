use tmui::{
    input::text::Text, label::Label, prelude::*, tlib::object::{ObjectImpl, ObjectSubclass}, widget::WidgetImpl
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct TextBundle {
    #[children]
    label: Box<Label>,

    #[children]
    text: Box<Text>,
}

impl ObjectSubclass for TextBundle {
    const NAME: &'static str = "TextBundle";
}

impl ObjectImpl for TextBundle {}

impl WidgetImpl for TextBundle {}

impl TextBundle {
    #[inline]
    pub fn new(label: &str) -> Box<Self> {
        let mut tb: Box<Self> = Object::new(&[]);
        tb.label.set_margin_top(3);
        tb.label.set_text(label);
        tb
    }
}