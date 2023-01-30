use tlib::object::{ObjectSubclass, ObjectImpl};

use crate::{prelude::*, widget::WidgetImpl};

#[extends_widget]
pub struct Layouts {}

impl ObjectSubclass for Layouts {
    const NAME: &'static str = "Layouts";

    type Type = Layouts;

    type ParentType = Widget;
}

impl ObjectImpl for Layouts {}

impl WidgetImpl for Layouts {}