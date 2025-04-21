use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct MyDiffWidget {}

impl ObjectSubclass for MyDiffWidget {
    const NAME: &'static str = "MyDiffWidget";
}

impl ObjectImpl for MyDiffWidget {
    fn initialize(&mut self) {
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_background(Color::GREY_MEDIUM);
        self.set_render_difference(true);
    }
}

impl WidgetImpl for MyDiffWidget {}

impl MyDiffWidget {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}
