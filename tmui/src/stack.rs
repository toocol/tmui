use crate::{
    application_window::ApplicationWindow,
    container::{ContainerImpl, ContainerImplExt},
    layout::{ContainerLayout, LayoutManager},
    prelude::*,
};
use tlib::{object::{ObjectImpl, ObjectSubclass}, stack_impl};

#[extends(Container)]
pub struct Stack {
    current_index: usize,
}

impl ObjectSubclass for Stack {
    const NAME: &'static str = "Stack";
}

impl ObjectImpl for Stack {
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Stack, ReflectStackTrait>();
    }
}

impl WidgetImpl for Stack {}

impl ContainerImpl for Stack {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.container.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.container
            .children
            .iter_mut()
            .map(|c| c.as_mut())
            .collect()
    }
}

impl ContainerImplExt for Stack {
    fn add_child<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        ApplicationWindow::initialize_dynamic_component(self, child.as_mut());
        if self.current_index == self.container.children.len() {
            child.show()
        } else {
            child.hide()
        }
        self.container.children.push(child);
        self.update();
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
        let previous = unsafe { widget_ptr.as_ref().unwrap() };

        let stack_trait_obj = cast!(widget as StackTrait).unwrap();
        let index = stack_trait_obj.current_index();

        if let Some(child) = widget.children_mut().get_mut(index) {
            child.position_layout(previous, previous, true);
        }
    }
}

impl Stack {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

#[reflect_trait]
pub trait StackTrait {
    fn current_index(&self) -> usize;

    fn switch(&mut self);

    fn switch_index(&mut self, index: usize);
}
stack_impl!(Stack);