use tlib::object::{ObjectSubclass, ObjectImpl};

use crate::{prelude::*, widget::WidgetImpl};

#[extends_widget]
pub struct Container {
    children: Vec<Box<dyn WidgetImpl>>,
}

impl ObjectSubclass for Container {
    const NAME: &'static str = "Container";

    type Type = Container;

    type ParentType = Widget;
}

impl ObjectImpl for Container {}

impl WidgetImpl for Container {}

pub trait ContainerImpl {
}

pub trait ContainerExt {
    fn children(&self) -> Vec<&dyn WidgetImpl>;

    fn add_child<T>(&self, child: T)
        where T: WidgetImpl + IsA<Widget>;
}

impl ContainerExt for Container {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        todo!()
    }

    fn add_child<T>(&self, child: T)
        where T: WidgetImpl + IsA<Widget> {
        todo!()
    }
}