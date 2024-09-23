use super::WidgetImpl;
use crate::{
    prelude::ApplicationWindow,
    window::{win_builder::WindowBuilder, win_config::WindowConfig},
};
use tlib::{prelude::*, reflect_trait};

#[reflect_trait]
pub trait WinWidget: WidgetImpl {
    fn child_process_fn(&self) -> Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>;

    fn is_win_widget_effect(&self) -> bool;

    fn set_win_widget_effect(&mut self, effect: bool);
}

pub(crate) fn handle_win_widget_create(win_widget: &dyn WinWidget) {
    if !win_widget.is_win_widget_effect() {
        return;
    }

    let rect = win_widget.borderless_rect();
    let child_proc_fn = win_widget.child_process_fn();
    ApplicationWindow::window().create_window(
        WindowBuilder::new()
            .config(
                WindowConfig::builder()
                    .position(rect.top_left())
                    .width(rect.width() as u32)
                    .height(rect.height() as u32)
                    .build(),
            )
            .child_window(true)
            .win_widget_id(win_widget.id())
            .on_activate(move |win| child_proc_fn(win)),
    )
}
