use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl, container::SCALE_ADAPTION,
};

#[extends(Container)]
pub struct Pane {
    direction: Direction,
}

impl ObjectSubclass for Pane {
    const NAME: &'static str = "Pane";
}

impl ObjectImpl for Pane {}

impl WidgetImpl for Pane {}

impl ContainerImpl for Pane {
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

impl ContainerImplExt for Pane {
    fn add_child<T>(&mut self, child: Box<T>)
    where
        T: WidgetImpl,
    {
        todo!()
    }
}

impl ContainerScaleCalculate for Pane {
    fn container_hscale_calculate(&self) -> f32 {
        Self::static_container_hscale_calculate(self)
    }

    fn container_vscale_calculate(&self) -> f32 {
        Self::static_container_vscale_calculate(self)
    }
}
impl StaticContainerScaleCalculate for Pane {
    fn static_container_hscale_calculate(c: &dyn ContainerImpl) -> f32 {
        c.children().iter().map(|c| c.hscale()).sum()
    }

    fn static_container_vscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_ADAPTION
    }
}

impl ChildContainerDiffRender for Pane {
    fn container_diff_render(&mut self, painter: &mut Painter, background: Color) {
        todo!()
    }
}

impl Layout for Pane {
    fn composition(&self) -> Composition {
        todo!()
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        todo!()
    }
}

#[derive(Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}
