use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::my_diff_widget::MyDiffWidget;

#[extends(Widget, Layout(SplitPane))]
pub struct MySplitPane {}

impl ObjectSubclass for MySplitPane {
    const NAME: &'static str = "MySplitPane";
}

impl ObjectImpl for MySplitPane {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_background(Color::CYAN);

        self.add_child(MyDiffWidget::new());
    }
}

impl WidgetImpl for MySplitPane {}

impl MySplitPane {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}
