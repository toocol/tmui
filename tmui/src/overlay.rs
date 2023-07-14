use crate::{
    layout::LayoutManager,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Container)]
pub struct Overlay {}

impl Overlay {
    pub fn add_overlay<T, P>(&mut self, mut child: Box<T>, point: P)
    where
        T: WidgetImpl,
        P: Into<Point>,
    {
        child.set_parent(self);
        let point = self.map_to_global(&point.into());
        child.set_fixed_x(point.x());
        child.set_fixed_y(point.y());

        ApplicationWindow::initialize_dynamic_component(child.as_mut());

        self.container.children.push(child);
        self.update();
    }
}

impl ObjectSubclass for Overlay {
    const NAME: &'static str = "Overlay";
}

impl ObjectImpl for Overlay {}

impl WidgetImpl for Overlay {}

impl ContainerImpl for Overlay {
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

impl ContainerImplExt for Overlay {
    fn add_child<T>(&mut self, _child: Box<T>)
    where
        T: WidgetImpl,
    {
        panic!("Use function `add_overlay()` instead.")
    }
}

impl Layout for Overlay {
    fn composition(&self) -> Composition {
        Self::static_composition()
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        Self::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for Overlay {
    fn static_composition() -> Composition {
        Composition::Overlay
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent, manage_by_container)

        // Do nothing, users need to manually specify the coordinates of overlay subcomponents
    }
}