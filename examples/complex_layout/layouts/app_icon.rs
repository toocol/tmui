use tmui::{
    prelude::*,
    tlib::{
        connect,
        object::{ObjectImpl, ObjectSubclass},
        run_after,
    },
    widget::{WidgetFinder, WidgetImpl},
};

use super::LeftPanel;

#[extends(Widget)]
#[run_after]
pub struct AppIcon {
    left_panel: ObjectId,
}

impl ObjectSubclass for AppIcon {
    const NAME: &'static str = "AppIcon";
}

impl ObjectImpl for AppIcon {
    fn initialize(&mut self) {}
}

impl WidgetImpl for AppIcon {
    #[inline]
    fn run_after(&mut self) {
        let lp_owner = self.window().finds::<LeftPanel>();
        let left_panel = lp_owner.first().unwrap();
        self.left_panel = left_panel.id();
        connect!(left_panel, size_changed(), self, linkage_size_change(Size));

        self.linkage_size_change(left_panel.size());
    }

    #[inline]
    fn on_mouse_pressed(&mut self, _: &tlib::events::MouseEvent) {
        self.find_id_mut::<LeftPanel>(self.left_panel)
            .unwrap()
            .toggle_visibility();
    }
}

impl AppIcon {
    fn linkage_size_change(&mut self, size: Size) {
        if size.width() > 0 {
            self.resize(Some(size.width()), None);
        } else if self.size().width() != 30 {
            self.resize(Some(30), None);
        }
    }
}
