use crate::{
    application_window::ApplicationWindow,
    container::{
        ContainerImpl, ContainerImplExt, ContainerScaleCalculate, StaticContainerScaleCalculate, ContainerLayoutEnum,
    },
    layout::{ContainerLayout, LayoutManager},
    prelude::*,
};
use tlib::{
    object::{ObjectImpl, ObjectSubclass},
    stack_impl,
};

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

    fn container_layout(&self) -> ContainerLayoutEnum {
        ContainerLayoutEnum::Stack
    }
}

impl ContainerImplExt for Stack {
    fn add_child<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        child.set_parent(self);
        if self.current_index == self.container.children.len() {
            child.show()
        } else {
            child.hide()
        }

        ApplicationWindow::initialize_dynamic_component(child.as_mut());

        self.container.children.push(child);
        self.update();
    }
}

impl Layout for Stack {
    fn composition(&self) -> Composition {
        Stack::static_composition(self)
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
    ) {
        Stack::container_position_layout(self, previous, parent)
    }
}

impl ContainerLayout for Stack {
    fn static_composition<T: WidgetImpl + ContainerImpl>(_: &T) -> Composition {
        Composition::Stack
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent);

        // deal with the children's position
        let widget_ptr = widget as *const dyn WidgetImpl;
        let previous = unsafe { Some(widget_ptr.as_ref().unwrap()) };

        let stack_trait_obj = cast!(widget as StackTrait).unwrap();
        let index = stack_trait_obj.current_index();

        if let Some(child) = widget.children_mut().get_mut(index) {
            child.position_layout(previous, previous);
        }
    }
}

#[reflect_trait]
pub trait StackTrait {
    fn current_child(&self) -> Option<&dyn WidgetImpl>;

    fn current_child_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    fn current_index(&self) -> usize;

    fn switch(&mut self);

    fn switch_index(&mut self, index: usize);
}
stack_impl!(Stack);

impl ContainerScaleCalculate for Stack {
    #[inline]
    fn container_hscale_calculate(&self) -> f32 {
        Self::static_container_hscale_calculate(self)
    }

    #[inline]
    fn container_vscale_calculate(&self) -> f32 {
        Self::static_container_vscale_calculate(self)
    }
}
impl StaticContainerScaleCalculate for Stack {
    #[inline]
    fn static_container_hscale_calculate(_: &dyn ContainerImpl) -> f32 {
        1.
    }

    #[inline]
    fn static_container_vscale_calculate(_: &dyn ContainerImpl) -> f32 {
        1.
    }
}

impl Stack {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
