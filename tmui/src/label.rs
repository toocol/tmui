use tlib::object::{ObjectSubclass, ObjectImpl};
use widestring::U16String;
use crate::{prelude::*, widget::WidgetImpl, graphics::painter::Painter};

#[extends_widget]
#[derive(Default)]
pub struct Label {
    label: Vec<u16>,
    content_halign: Align,
    content_valign: Align,
}

impl ObjectSubclass for Label {
    const NAME: &'static str = "Label";

    type Type = Label;

    type ParentType = Widget;
}

impl ObjectImpl for Label {}

impl WidgetImpl for Label {
    fn paint(&mut self, mut painter: Painter) {
    }
}

impl Label {
    pub fn new(text: Option<&str>) -> Self {
        let mut label: Label = Object::new(&[]);
        if let Some(text) = text {
            label.label = U16String::from_str(text).as_slice().to_vec();
        }
        label
    }
}