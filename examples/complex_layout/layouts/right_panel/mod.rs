mod split_pane;
mod stack;

use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use self::split_pane::MySplitPane;

#[extends(Widget, Layout(Stack))]
#[derive(Childrenable)]
pub struct RightPanel {
    #[children]
    split_pane: Tr<MySplitPane>,
}

impl ObjectSubclass for RightPanel {
    const NAME: &'static str = "RightPanel";
}

impl ObjectImpl for RightPanel {
    fn initialize(&mut self) {
        self.set_vexpand(true);
        self.set_hexpand(true);
    }
}

impl WidgetImpl for RightPanel {}
