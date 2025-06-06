use tmui::{
    prelude::*,
    tlib::{
        iter_executor,
        object::{ObjectImpl, ObjectSubclass},
    },
    widget::{IterExecutor, WidgetImpl},
};

use crate::{font_icons::FontIcons, svg_icons::SvgIcons};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[iter_executor]
pub struct Icons {
    #[children]
    font_icons: Tr<FontIcons>,

    #[children]
    svg_icons: Tr<SvgIcons>,
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
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}

impl IterExecutor for Icons {
    #[inline]
    fn iter_execute(&mut self) {}
}
