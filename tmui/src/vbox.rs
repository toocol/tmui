use crate::prelude::*;
use tlib::object::ObjectSubclass;

#[extends(Container)]
#[derive(Default)]
pub struct VBox {}

impl ObjectSubclass for VBox {
    const NAME: &'static str = "VBox";
}

impl ObjectImpl for VBox {}

impl WidgetImpl for VBox {}

impl ContainerImpl for VBox {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for VBox {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.children.push(Box::new(child))
    }
}

impl Layout for VBox {
    fn composition(&self) -> Composition {
        Composition::VerticalArrange
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        VBox::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for VBox {
    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        todo!()
    }
}

impl VBox {
    pub fn new() -> Self {
        Object::new(&[])
    }
}
