use tlib::object::ObjectSubclass;
use crate::{prelude::*, container::{ContainerImpl, ContainerImplExt}};

#[extends(Container)]
#[derive(Default)]
pub struct VBox {}

impl ObjectSubclass for VBox {
    const NAME: &'static str = "VBox";

    type Type = VBox;

    type ParentType = Container;
}

impl ObjectImpl for VBox {}

impl WidgetImpl for VBox {}

impl ContainerImpl for VBox {
    fn children(&self) -> Vec< &dyn WidgetImpl>  {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec< &mut dyn WidgetImpl>  {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for VBox {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget> {
        self.children.push(Box::new(child))
    }
}

impl Layout for VBox {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::VerticalArrange
    }

    fn position_layout(&mut self, previous: &dyn WidgetImpl, parent: &dyn WidgetImpl) {}
}