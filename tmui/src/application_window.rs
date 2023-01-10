use tlib::{prelude::*, object::{ObjectSubclass, ObjectImpl}};

#[extends_object]
#[derive(Default)]
pub struct ApplicationWindow {

}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";

    type Type = ApplicationWindow;

    type ParentType = Object;
}

impl ObjectImpl for ApplicationWindow {}
