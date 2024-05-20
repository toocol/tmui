use tmui::{
    prelude::*,
    tlib::{
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

impl ObjectImpl for AppIcon {}

impl WidgetImpl for AppIcon {
    #[inline]
    fn run_after(&mut self) {
        self.left_panel = self.finds::<LeftPanel>().first().unwrap().id();
    }

    #[inline]
    fn on_mouse_pressed(&mut self, _: &tlib::events::MouseEvent) {
        self.find_id_mut::<LeftPanel>(self.left_panel)
            .unwrap()
            .toggle_visibility();
    }
}
