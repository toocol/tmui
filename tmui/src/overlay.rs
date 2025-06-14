use crate::{
    container::{
        ContainerLayoutEnum, ContainerScaleCalculate, StaticContainerScaleCalculate, SCALE_DISMISS,
    },
    graphics::element::ElementInner,
    layout::LayoutMgr,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Container)]
pub struct Overlay {}

impl Overlay {
    pub fn add_overlay<T, P>(&mut self, mut child: Tr<T>, point: P)
    where
        T: WidgetImpl,
        P: Into<Point>,
    {
        child.set_parent(self);
        let point = self.map_to_global(&point.into());
        child.set_fixed_x(point.x());
        child.set_fixed_y(point.y());

        self.container.children.push(child.clone().into());
        ApplicationWindow::initialize_dynamic_component(child.as_dyn_mut(), self.is_in_tree());
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
        self.container.children.iter().map(|c| c.bind()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.container
            .children
            .iter_mut()
            .map(|c| c.bind_mut())
            .collect()
    }

    fn container_layout(&self) -> ContainerLayoutEnum {
        ContainerLayoutEnum::Overlay
    }
}

impl ContainerImplExt for Overlay {
    fn add_child<T>(&mut self, _child: Tr<T>)
    where
        T: WidgetImpl,
    {
        panic!("Use function `add_overlay()` instead.")
    }

    fn remove_children(&mut self, id: ObjectId) {
        if let Some(index) = self.container.children.iter().position(|w| w.id() == id) {
            let removed = self.container.children.remove(index);

            let window = ApplicationWindow::window();
            window._add_removed_widget(removed);
            window.layout_change(self);
        }
    }
}

impl Layout for Overlay {
    #[inline]
    fn composition(&self) -> Composition {
        Self::static_composition(self)
    }

    #[inline]
    fn position_layout(&mut self, parent: Option<&dyn WidgetImpl>) {
        Self::container_position_layout(self, parent)
    }
}

impl ContainerLayout for Overlay {
    #[inline]
    fn static_composition<T: WidgetImpl + ContainerImpl>(_: &T) -> Composition {
        Composition::Overlay
    }

    #[inline]
    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        parent: Option<&dyn WidgetImpl>,
    ) {
        LayoutMgr::base_widget_position_layout(widget, parent)

        // Do nothing, users need to manually specify the position coordinates of overlay subcomponents
    }
}

impl ContainerScaleCalculate for Overlay {
    #[inline]
    fn container_hscale_calculate(&self) -> f32 {
        Self::static_container_hscale_calculate(self)
    }

    #[inline]
    fn container_vscale_calculate(&self) -> f32 {
        Self::static_container_vscale_calculate(self)
    }
}
impl StaticContainerScaleCalculate for Overlay {
    #[inline]
    fn static_container_hscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_DISMISS
    }

    #[inline]
    fn static_container_vscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_DISMISS
    }
}

#[reflect_trait]
pub trait Overlaid {}

#[reflect_trait]
pub trait PartCovered {
    fn is_covered(&self) -> bool;
}
