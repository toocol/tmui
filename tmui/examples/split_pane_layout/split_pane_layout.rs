use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget, Layout(SplitPane))]
#[derive(Default)]
pub struct SplitPaneLayout {}

impl ObjectSubclass for SplitPaneLayout {
   const NAME: &'static str = "SplitPaneLayout";
}

impl ObjectImpl for SplitPaneLayout {}

impl WidgetImpl for SplitPaneLayout {}