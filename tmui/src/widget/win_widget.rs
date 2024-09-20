use tlib::{reflect_trait, prelude::*};

use super::WidgetImpl;

#[reflect_trait]
pub trait WinWidget {
    fn is_win_widget_effect(&self) -> bool;

    fn set_win_widget_effect(&mut self, effect: bool);
}

pub(crate) fn handle_win_widget_register(widget: &mut dyn WidgetImpl) {
    if let Some(win_widget) = cast!(widget as WinWidget) {
        if win_widget.is_win_widget_effect() {

        }
    }
}