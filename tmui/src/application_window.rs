use crate::{prelude::*, widget::WidgetImpl};
use tlib::object::{ObjectImpl, ObjectSubclass};

#[extends_widget]
#[derive(Default)]
pub struct ApplicationWindow {}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";

    type Type = ApplicationWindow;

    type ParentType = Object;
}

impl ObjectImpl for ApplicationWindow {
    fn construct(&self) {
        self.parent_construct();
        println!("`ApplicationWindow` construct: static_type: {}", Self::static_type().name())
    }
}

impl WidgetImpl for ApplicationWindow {}
