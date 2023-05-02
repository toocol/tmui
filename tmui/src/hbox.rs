use crate::prelude::*;
use tlib::object::ObjectSubclass;

#[extends(Container)]
#[derive(Default)]
pub struct HBox {}

impl ObjectSubclass for HBox {
    const NAME: &'static str = "HBox";
}

impl ObjectImpl for HBox {}

impl WidgetImpl for HBox {}

impl ContainerImpl for HBox {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for HBox {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.children.push(Box::new(child))
    }
}

impl Layout for HBox {
    fn composition(&self) -> Composition {
        Composition::HorizontalArrange
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        HBox::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for HBox {
    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        todo!()
    }
}
