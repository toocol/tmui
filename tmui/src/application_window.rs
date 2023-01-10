use tlib::{object::{ObjectSubclass, ObjectImpl}};
use crate::{prelude::*, widget::WidgetImpl};

#[extends_widget]
#[derive(Default)]
pub struct ApplicationWindow {

}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";

    type Type = ApplicationWindow;

    type ParentType = Object;
}

impl ObjectImpl for ApplicationWindow {}

impl WidgetImpl for ApplicationWindow {}
