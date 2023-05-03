use crate::{
    container::{ContainerImpl, ContainerImplExt},
    layout::{ContainerLayout, LayoutManager},
    prelude::*,
};
use tlib::object::{ObjectImpl, ObjectSubclass};

#[extends(Container)]
#[derive(Default)]
pub struct Stack {}

impl ObjectSubclass for Stack {
    const NAME: &'static str = "Stack";
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
    fn composition(&self) -> Composition {
        Stack::static_composition()
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        Stack::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for Stack {
    fn static_composition() -> Composition {
        Composition::Overlay
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent, manage_by_container);

        // deal with the children's position
        let widget_ptr = widget as *const dyn WidgetImpl;
        let mut previous = unsafe { widget_ptr.as_ref().unwrap() };
        for child in widget.children_mut().into_iter() {
            child.position_layout(previous, previous, true);
            previous = child;
        }
    }
}

impl Stack {
    pub fn new() -> Self {
        Object::new(&[])
    }
}
