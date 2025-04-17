use crate::{
    application_window::ApplicationWindow,
    container::{
        ContainerImpl, ContainerImplExt, ContainerLayoutEnum, ContainerScaleCalculate,
        StaticContainerScaleCalculate,
    },
    layout::{ContainerLayout, LayoutMgr},
    prelude::*,
    widget::InnerEventProcess,
};
use tlib::{
    object::{ObjectImpl, ObjectSubclass},
    ptr_ref, stack_impl,
};

#[extends(Container)]
pub struct Stack {
    current_index: usize,
}

impl ObjectSubclass for Stack {
    const NAME: &'static str = "Stack";
}

impl ObjectImpl for Stack {
    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Stack, ReflectStackImpl>();
    }

    fn on_property_set(&mut self, name: &str, value: &Value) {
        match name {
            "visible" => {
                let visible = value.get::<bool>();
                emit!(self, visibility_changed(visible));
                self.inner_visibility_changed(visible);
                self.on_visibility_changed(visible);
                if visible {
                    if let Some(c) = self.current_child_mut() {
                        if !c.visibility_check() {
                            return;
                        }
                        if let Some(iv) = cast!(c as IsolatedVisibility) {
                            if iv.auto_hide() {
                                return;
                            }
                        }

                        c.set_property("visible", true.to_value());
                        c.set_render_styles(true);
                    }
                } else {
                    for c in self.children_mut() {
                        c.set_property("visible", false.to_value());
                    }
                }
            }
            _ => self.parent_on_property_set(name, value),
        }
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

        ApplicationWindow::initialize_dynamic_component(child.as_mut(), self.is_in_tree());

        self.container.children.push(child);
        self.update();
    }

    fn remove_children(&mut self, id: ObjectId) {
        if let Some(index) = self.container.children.iter().position(|w| w.id() == id) {
            let removed = self.container.children.remove(index);
            if self.current_index == self.container.children.len() && self.current_index != 0 {
                self.current_index -= 1;
            }

            let window = ApplicationWindow::window();
            window._add_removed_widget(removed);
            window.layout_change(self);
        }
    }
}

impl Layout for Stack {
    #[inline]
    fn composition(&self) -> Composition {
        Stack::static_composition(self)
    }

    #[inline]
    fn position_layout(&mut self, parent: Option<&dyn WidgetImpl>) {
        Stack::container_position_layout(self, parent)
    }
}

impl ContainerLayout for Stack {
    #[inline]
    fn static_composition<T: WidgetImpl + ContainerImpl>(_: &T) -> Composition {
        Composition::Stack
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        parent: Option<&dyn WidgetImpl>,
    ) {
        LayoutMgr::base_widget_position_layout(widget, parent);

        // deal with the children's position
        let widget_ptr = widget as *const dyn WidgetImpl;

        widget.children_mut().iter_mut().for_each(|child| {
            LayoutMgr::base_widget_position_layout_inner(*child, Some(ptr_ref!(widget_ptr)));
        });
    }
}

#[reflect_trait]
pub trait StackImpl {
    fn current_child(&self) -> Option<&dyn WidgetImpl>;

    fn current_child_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    fn current_index(&self) -> usize;

    fn switch(&mut self);

    fn switch_index(&mut self, index: usize);

    fn remove_index(&mut self, index: usize);
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
