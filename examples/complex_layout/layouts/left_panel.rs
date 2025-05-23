use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::{ActivityBar, WorkspacePanel};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct LeftPanel {
    #[children]
    activity_bar: Tr<ActivityBar>,

    #[children]
    workspace_panel: Tr<WorkspacePanel>,
}

impl ObjectSubclass for LeftPanel {
    const NAME: &'static str = "LeftPanel";
}

impl ObjectImpl for LeftPanel {
    fn initialize(&mut self) {
        self.width_request(300);
        self.set_vexpand(true);
    }
}

impl WidgetImpl for LeftPanel {}

impl LeftPanel {
    #[inline]
    pub fn toggle_visibility(&mut self) {
        if self.visible() {
            self.hide()
        } else {
            self.show()
        }
    }
}
