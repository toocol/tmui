use crate::{
    container::{SCALE_ADAPTION, ContainerLayoutEnum},
    hbox::hbox_layout_homogeneous,
    layout::LayoutManager,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    vbox::vbox_layout_homogeneous,
    widget::{InnerCustomizeEventProcess, WidgetImpl},
};
use log::error;
use tlib::{pane_impl, pane_init, pane_type_register};

#[extends(Container)]
pub struct Pane {
    direction: Direction,
    resize_zone: bool,
    resize_pressed: bool,
}

impl Pane {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

#[reflect_trait]
pub trait PaneExt: ContainerImpl {
    /// Get the direction of `Pane`.
    fn direction(&self) -> Direction;

    /// Set the direction of `Pane`.
    ///
    /// The default value was [`Direction::Horizontal`](Direction::Horizontal)
    fn set_direction(&mut self, direction: Direction);

    /// Those function should be f**king private, dont know how to do it:
    fn is_resize_zone(&self) -> bool;

    fn set_resize_zone(&mut self, resize_zone: bool);

    fn is_resize_pressed(&self) -> bool;

    fn set_resize_pressed(&mut self, resize_pressed: bool);
}

pane_impl!(Pane);

impl ObjectSubclass for Pane {
    const NAME: &'static str = "Pane";
}

impl ObjectImpl for Pane {
    fn initialize(&mut self) {
        pane_init!();
    }

    fn type_register(&self, type_registry: &mut TypeRegistry) {
        pane_type_register!(Pane);
    }
}

impl WidgetImpl for Pane {}

impl<T: PaneExt> InnerCustomizeEventProcess for T {
    fn inner_customize_mouse_move(&mut self, event: &tlib::events::MouseEvent) {
        let children = self.children();
        if children.len() == 0 {
            return;
        }

        let pos = event.position();
        let mut first_rect = children[0].rect();
        first_rect.set_point(&self.map_to_widget(&first_rect.top_left()));

        match self.direction() {
            Direction::Horizontal => {
                if pos.0 >= first_rect.right() - 2 && pos.0 <= first_rect.right() + 2 {
                    if !self.is_resize_zone() {
                        self.set_cursor_shape(SystemCursorShape::SizeHorCursor);
                        self.set_resize_zone(true);
                    }
                } else {
                    if self.is_resize_zone() && !self.is_resize_pressed() {
                        self.set_cursor_shape(SystemCursorShape::ArrowCursor);
                        self.set_resize_zone(false);
                    }
                }

                if self.is_resize_pressed() {
                    let new_width = pos.0 - first_rect.left();

                    let mut children_mut = self.children_mut();
                    let first = children_mut.get_mut(0).unwrap();

                    if new_width == first.size().width() {
                        return;
                    }

                    first.width_request(new_width);
                    let reset_expand = first.hexpand();
                    if reset_expand {
                        first.set_hexpand(false);
                    }

                    self.window().layout_change(self);

                    if reset_expand {
                        self.children_mut()[0].set_hexpand(true);
                    }
                }
            }
            Direction::Vertical => {
                if pos.1 >= first_rect.bottom() - 2 && pos.1 <= first_rect.bottom() + 2 {
                    if !self.is_resize_zone() {
                        self.set_cursor_shape(SystemCursorShape::SizeVerCursor);
                        self.set_resize_zone(true);
                    }
                } else {
                    if self.is_resize_zone() && !self.is_resize_pressed() {
                        self.set_cursor_shape(SystemCursorShape::ArrowCursor);
                        self.set_resize_zone(false);
                    }
                }

                if self.is_resize_pressed() {
                    let new_height = pos.1 - first_rect.top();

                    let mut children_mut = self.children_mut();
                    let first = children_mut.get_mut(0).unwrap();

                    if new_height == first.size().height() {
                        return;
                    }

                    first.height_request(new_height);
                    let reset_expand = first.vexpand();
                    if reset_expand {
                        first.set_vexpand(false);
                    }

                    self.window().layout_change(self);

                    if reset_expand {
                        self.children_mut()[0].set_vexpand(true);
                    }
                }
            }
        }
    }

    #[inline]
    fn inner_customize_mouse_pressed(&mut self, _: &tlib::events::MouseEvent) {
        if self.is_resize_zone() {
            self.set_resize_pressed(true);
        }
    }

    fn inner_customize_mouse_released(&mut self, event: &tlib::events::MouseEvent) {
        let children = self.children();
        if children.len() == 0 {
            return;
        }

        let pos = event.position();
        let first_rect = children[0].rect();

        if self.is_resize_pressed() {
            self.set_resize_pressed(false);

            match self.direction() {
                Direction::Horizontal => {
                    if pos.0 >= first_rect.right() - 2 && pos.0 <= first_rect.right() + 2 {
                        if !self.is_resize_zone() {
                            self.set_cursor_shape(SystemCursorShape::SizeHorCursor);
                            self.set_resize_zone(true);
                        }
                    } else {
                        if self.is_resize_zone() && !self.is_resize_pressed() {
                            self.set_cursor_shape(SystemCursorShape::ArrowCursor);
                            self.set_resize_zone(false);
                        }
                    }
                }
                Direction::Vertical => {
                    if pos.1 >= first_rect.bottom() - 2 && pos.1 <= first_rect.bottom() + 2 {
                        if !self.is_resize_zone() {
                            self.set_cursor_shape(SystemCursorShape::SizeVerCursor);
                            self.set_resize_zone(true);
                        }
                    } else {
                        if self.is_resize_zone() && !self.is_resize_pressed() {
                            self.set_cursor_shape(SystemCursorShape::ArrowCursor);
                            self.set_resize_zone(false);
                        }
                    }
                }
            }
        }
    }
}

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

    fn container_layout(&self) -> ContainerLayoutEnum {
        ContainerLayoutEnum::Pane
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
        let pane = cast!(c as PaneExt).unwrap();

        match pane.direction() {
            Direction::Horizontal => c
                .children()
                .iter()
                .filter(|c| !c.fixed_width())
                .map(|c| c.hscale())
                .sum(),
            Direction::Vertical => SCALE_ADAPTION,
        }
    }

    fn static_container_vscale_calculate(c: &dyn ContainerImpl) -> f32 {
        let pane = cast!(c as PaneExt).unwrap();

        match pane.direction() {
            Direction::Horizontal => SCALE_ADAPTION,
            Direction::Vertical => c
                .children()
                .iter()
                .filter(|c| !c.fixed_height())
                .map(|c| c.vscale())
                .sum(),
        }
    }
}

impl ChildContainerDiffRender for Pane {
    fn container_diff_render(&mut self, _painter: &mut Painter, _background: Color) {}
}

impl SizeUnifiedAdjust for Pane {
    #[inline]
    fn size_unified_adjust(&mut self) {
        Self::static_size_unified_adjust(self)
    }
}
impl StaticSizeUnifiedAdjust for Pane {
    #[inline]
    fn static_size_unified_adjust(container: &mut dyn ContainerImpl) {
        let pane = cast!(container as PaneExt).unwrap();

        match pane.direction() {
            Direction::Horizontal => HBox::static_size_unified_adjust(container),
            Direction::Vertical => VBox::static_size_unified_adjust(container),
        }
    }
}

impl Layout for Pane {
    fn composition(&self) -> Composition {
        Self::static_composition(self)
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
    ) {
        Self::container_position_layout(self, previous, parent);
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
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent);

        let pane = cast!(widget as PaneExt).unwrap();

        match pane.direction() {
            Direction::Horizontal => hbox_layout_homogeneous(widget, Align::Start, Align::Start),
            Direction::Vertical => vbox_layout_homogeneous(widget, Align::Start, Align::Start),
        }
    }
}

pub type PaneDirection = Direction;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}
