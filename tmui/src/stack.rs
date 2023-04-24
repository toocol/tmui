use tlib::object::{ObjectImpl, ObjectSubclass};
use crate::{
    container::{ContainerImpl, ContainerImplExt},
    prelude::*,
};

#[extends(Container)]
#[derive(Default)]
pub struct Stack {}

impl ObjectSubclass for Stack {
    const NAME: &'static str = "Stack";

    type Type = Stack;

    type ParentType = Container;
}

impl ObjectImpl for Stack {}

impl WidgetImpl for Stack {}

impl ContainerImpl for Stack {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for Stack {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.children.push(Box::new(child))
    }
}

impl Layout for Stack {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Overlay
    }

    fn position_layout(&mut self, previous: &dyn WidgetImpl, parent: &dyn WidgetImpl) {}
}

impl Stack {
    pub fn new() -> Self {
        Object::new(&[])
    }
}