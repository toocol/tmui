use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::{font_icons::FontIcons, svg_icons::SvgIcons};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct Icons {
    #[children]
    font_icons: Box<FontIcons>,

    #[children]
    svg_icons: Box<SvgIcons>,
}

impl ObjectSubclass for Icons {
    const NAME: &'static str = "Icons";
}

impl ObjectImpl for Icons {
    fn initialize(&mut self) {
        self.set_spacing(20);

        self.set_hexpand(true);
        self.set_vexpand(true);
    }
}

impl WidgetImpl for Icons {}

impl Icons {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}