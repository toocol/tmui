use crate::{
    container::SCALE_ADAPTION,
    hbox::hbox_layout_homogeneous,
    layout::LayoutManager,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    vbox::vbox_layout_homogeneous,
    widget::WidgetImpl,
};
use log::error;

#[extends(Container)]
pub struct Pane {
    direction: Direction,
}

impl Pane {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

#[reflect_trait]
pub trait PaneExt {
    fn direction(&self) -> Direction;

    fn set_direction(&mut self, direction: Direction);
}

impl PaneExt for Pane {
    #[inline]
    fn direction(&self) -> Direction {
        self.direction
    }

    #[inline]
    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;

        if self.window().initialized() {
            self.window().layout_change(self)
        }
    }
}

impl ObjectSubclass for Pane {
    const NAME: &'static str = "Pane";
}

impl ObjectImpl for Pane {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_mouse_tracking(true);

        self.enable_bubble(EventBubble::MOUSE_MOVE);
        self.enable_bubble(EventBubble::MOUSE_PRESSED);
        self.enable_bubble(EventBubble::MOUSE_RELEASED);
    }

    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectPaneExt>();
    }
}

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
    fn add_child<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        if self.container.children.len() >= 2 {
            error!("`Pane` can only have two child component.");
            return;
        }

        child.set_parent(self);
        ApplicationWindow::initialize_dynamic_component(child.as_mut());
        self.container.children.push(child);
        self.update();
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
    fn container_diff_render(&mut self, _painter: &mut Painter, _background: Color) {}
}

impl Layout for Pane {
    fn composition(&self) -> Composition {
        Self::static_composition(self)
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        Self::container_position_layout(self, previous, parent, manage_by_container);
    }
}

impl ContainerLayout for Pane {
    fn static_composition<T: WidgetImpl + ContainerImpl>(widget: &T) -> Composition {
        let pane = cast!(widget as PaneExt).unwrap();

        match pane.direction() {
            Direction::Horizontal => Composition::HorizontalArrange,
            Direction::Vertical => Composition::VerticalArrange,
        }
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent, manage_by_container);

        let pane = cast!(widget as PaneExt).unwrap();

        match pane.direction() {
            Direction::Horizontal => hbox_layout_homogeneous(widget, Align::Start, Align::Start),
            Direction::Vertical => vbox_layout_homogeneous(widget, Align::Start, Align::Start),
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}
