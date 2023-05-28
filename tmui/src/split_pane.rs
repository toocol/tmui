use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Container)]
#[derive(Default)]
pub struct SplitPane {}

impl ObjectSubclass for SplitPane {
    const NAME: &'static str = "SplitPane";
}

impl ObjectImpl for SplitPane {}

impl WidgetImpl for SplitPane {}

impl ContainerImpl for SplitPane {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for SplitPane {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.children.push(Box::new(child))
    }
}

impl Layout for SplitPane {
    fn composition(&self) -> Composition {
        SplitPane::static_composition()
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        SplitPane::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for SplitPane {
    fn static_composition() -> Composition {
        todo!()
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        todo!()
    }
}
