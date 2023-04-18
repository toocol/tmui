use tlib::object::{ObjectImpl, ObjectSubclass};

use crate::{prelude::*, widget::WidgetImpl};

#[extends_widget]
#[derive(Default)]
#[allow(dead_code)]
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
    /// Go to[`Function defination`](ContainerExt::children) (Defined in [`ContainerExt`])
    /// Get all the children in `Container`
    fn children(&self) -> Vec<&dyn WidgetImpl>;
}

pub trait ContainerImplExt {
    /// Go to[`Function defination`](ContainerExt::add_child) (Defined in [`ContainerExt`])
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>;
}