use crate::{prelude::*, layout::LayoutManager};
use tlib::object::ObjectSubclass;

#[extends(Container)]
#[derive(Default)]
pub struct HBox {
    content_halign: Align,
    content_valign: Align,
    homogeneous: bool,
}

impl ObjectSubclass for HBox {
    const NAME: &'static str = "HBox";
}

impl ObjectImpl for HBox {
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<HBox, ReflectContentAlignment>();
    }
}

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
        HBox::static_composition()
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
    fn static_composition() -> Composition {
        Composition::HorizontalArrange
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent, manage_by_container);
    }
}

impl ContentAlignment for HBox {
    #[inline]
    fn homogeneous(&self) -> bool {
        self.homogeneous
    }

    #[inline]
    fn set_homogeneous(&mut self,homogeneous:bool) {
        self.homogeneous = homogeneous
    }

    #[inline]
    fn content_halign(&self) -> Align {
        self.content_halign
    }

    #[inline]
    fn content_valign(&self) -> Align {
        self.content_valign
    }

    #[inline]
    fn set_content_halign(&mut self, halign: Align) {
        self.content_halign = halign
    }

    #[inline]
    fn set_content_valign(&mut self, valign: Align) {
        self.content_valign = valign
    }
}

impl HBox {
    pub fn new() -> Self {
        Object::new(&[])
    }
}
