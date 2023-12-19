use crate::{
    application_window::ApplicationWindow,
    container::{ContainerScaleCalculate, SCALE_ADAPTION, SCALE_DISMISS},
    graphics::painter::Painter,
    layout::LayoutManager,
    prelude::*,
    scroll_bar::{ScrollBar, ScrollBarPosition, DEFAULT_SCROLL_BAR_WIDTH},
};
use derivative::Derivative;
use log::debug;
use tlib::{
    connect,
    events::{DeltaType, MouseEvent},
    namespace::{KeyboardModifier, Orientation},
    object::ObjectSubclass,
    prelude::extends,
    ptr_mut,
};

#[extends(Container)]
pub struct ScrollArea {
    #[derivative(Default(value = "Object::new(&[])"))]
    scroll_bar: Box<ScrollBar>,
    area: Option<Box<dyn WidgetImpl>>,
}

/////////////////////////////////////////// Start: ScrollArea self implementations ///////////////////////////////////////////
#[reflect_trait]
pub trait ScrollAreaExt: WidgetImpl {
    fn get_area(&self) -> Option<&dyn WidgetImpl>;

    fn get_area_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    fn get_scroll_bar(&self) -> &ScrollBar;

    fn get_scroll_bar_mut(&mut self) -> &mut ScrollBar;

    fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition);

    fn set_orientation(&mut self, orientation: Orientation);

    fn scroll(&mut self, delta: i32, delta_type: DeltaType);

    fn adjust_area_layout(&mut self, size: Size);
}

pub trait ScrollAreaGenericExt {
    fn set_area<T: WidgetImpl>(&mut self, area: Box<T>);

    fn get_area_cast<T: WidgetImpl + ObjectSubclass>(&self) -> Option<&T>;

    fn get_area_cast_mut<T: WidgetImpl + ObjectSubclass>(&mut self) -> Option<&mut T>;
}

impl ScrollAreaExt for ScrollArea {
    #[inline]
    fn get_area(&self) -> Option<&dyn WidgetImpl> {
        self.area.as_ref().and_then(|w| Some(w.as_ref()))
    }

    #[inline]
    fn get_area_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
        self.area.as_mut().and_then(|w| Some(w.as_mut()))
    }

    #[inline]
    fn get_scroll_bar(&self) -> &ScrollBar {
        &self.scroll_bar
    }

    #[inline]
    fn get_scroll_bar_mut(&mut self) -> &mut ScrollBar {
        &mut self.scroll_bar
    }

    #[inline]
    fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition) {
        self.scroll_bar.set_scroll_bar_position(scroll_bar_position);
        self.update();
    }

    #[inline]
    fn set_orientation(&mut self, orientation: Orientation) {
        self.scroll_bar.set_orientation(orientation);
        self.update();
    }

    /// Scroll the widget. </br>
    /// delta was positive value when scroll down/right.
    #[inline]
    fn scroll(&mut self, delta: i32, delta_type: DeltaType) {
        self.scroll_bar
            .scroll_by_delta(KeyboardModifier::NoModifier, delta, delta_type);
    }

    #[inline]
    fn adjust_area_layout(&mut self, size: Size) {
        if size.width() == 0 || size.height() == 0 {
            debug!("The size of `ScrollArea` was not specified, skip adjust_area_layout()");
            return;
        }

        if let Some(area) = self.get_area_mut() {
            area.set_vexpand(true);
            area.set_hexpand(true);
            // area.set_hscale(size.width() as f32 - 10.);
        }
    }
}

impl ScrollAreaGenericExt for ScrollArea {
    #[inline]
    fn set_area<T: WidgetImpl>(&mut self, mut area: Box<T>) {
        area.set_parent(self);
        area.set_vexpand(true);
        area.set_hexpand(true);

        ApplicationWindow::initialize_dynamic_component(area.as_mut());
        self.area = Some(area);

        self.adjust_area_layout(self.size());
    }

    #[inline]
    fn get_area_cast<T: WidgetImpl + ObjectSubclass>(&self) -> Option<&T> {
        self.area.as_ref().and_then(|w| w.downcast_ref::<T>())
    }

    #[inline]
    fn get_area_cast_mut<T: WidgetImpl + ObjectSubclass>(&mut self) -> Option<&mut T> {
        self.area.as_mut().and_then(|w| w.downcast_mut::<T>())
    }
}
/////////////////////////////////////////// End: ScrollArea self implementations ///////////////////////////////////////////

impl ObjectSubclass for ScrollArea {
    const NAME: &'static str = "ScrollArea";
}

impl ObjectImpl for ScrollArea {
    fn construct(&mut self) {
        self.parent_construct();

        self.scroll_bar.set_vexpand(true);
        self.scroll_bar.width_request(10);
        // self.scroll_bar.set_hscale(10.);

        let parent = self as *mut dyn WidgetImpl;
        self.scroll_bar.set_parent(parent);

        connect!(self, size_changed(), self, adjust_area_layout(Size));
    }

    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<ScrollArea, ReflectScrollAreaExt>();
    }
}

impl WidgetImpl for ScrollArea {
    fn on_mouse_wheel(&mut self, event: &MouseEvent) {
        self.scroll_bar.on_mouse_wheel(event)
    }
}

impl ContainerImpl for ScrollArea {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        let mut children: Vec<&dyn WidgetImpl> = vec![self.scroll_bar.as_ref()];
        if self.area.is_some() {
            children.push(self.area.as_ref().unwrap().as_ref())
        }
        children
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        let mut children: Vec<&mut dyn WidgetImpl> = vec![self.scroll_bar.as_mut()];
        if self.area.is_some() {
            children.push(self.area.as_mut().unwrap().as_mut())
        }
        children
    }
}

impl ContainerImplExt for ScrollArea {
    fn add_child<T>(&mut self, _: Box<T>)
    where
        T: WidgetImpl,
    {
        panic!("Please use `set_area()` instead in `ScrollArea`")
    }
}

impl Layout for ScrollArea {
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

impl ContainerLayout for ScrollArea {
    fn static_composition<T: WidgetImpl + ContainerImpl>(widget: &T) -> Composition {
        let widget = cast!(widget as ScrollAreaExt).unwrap();
        match widget.get_scroll_bar().orientation() {
            Orientation::Horizontal => Composition::VerticalArrange,
            Orientation::Vertical => Composition::HorizontalArrange,
        }
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent, manage_by_container);

        let widget = cast_mut!(widget as ScrollAreaExt).unwrap();

        // Deal with the area and scroll bar's position:
        let rect = widget.rect();
        let scroll_bar = ptr_mut!(widget as *mut dyn ScrollAreaExt).get_scroll_bar_mut();
        match scroll_bar.scroll_bar_position() {
            ScrollBarPosition::Start => {
                scroll_bar.set_fixed_x(rect.x() + scroll_bar.margin_left());
                scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());

                if let Some(area) = widget.get_area_mut() {
                    let scroll_bar_rect = scroll_bar.rect();
                    area.set_fixed_x(
                        scroll_bar_rect.x() + scroll_bar_rect.width() + area.margin_left(),
                    );
                    area.set_fixed_y(
                        scroll_bar_rect.y() + scroll_bar_rect.width() + area.margin_top(),
                    );
                }
            }
            ScrollBarPosition::End => {
                if let Some(area) = widget.get_area_mut() {
                    area.set_fixed_x(rect.x() + area.margin_left());
                    area.set_fixed_y(rect.y() + area.margin_top());

                    let area_rect = area.rect();
                    scroll_bar.set_fixed_x(
                        rect.x() + rect.width() + scroll_bar.margin_left()
                            - DEFAULT_SCROLL_BAR_WIDTH,
                    );
                    scroll_bar.set_fixed_y(area_rect.y() + scroll_bar.margin_top());
                } else {
                    widget.get_scroll_bar_mut().set_fixed_x(
                        rect.x() + rect.width() + scroll_bar.margin_left()
                            - DEFAULT_SCROLL_BAR_WIDTH,
                    );
                    scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());
                }
            }
        }
    }
}

impl ContainerScaleCalculate for ScrollArea {
    #[inline]
    fn container_hscale_calculate(&self) -> f32 {
        Self::static_container_hscale_calculate(self)
    }

    #[inline]
    fn container_vscale_calculate(&self) -> f32 {
        Self::static_container_vscale_calculate(self)
    }
}
impl StaticContainerScaleCalculate for ScrollArea {
    #[inline]
    fn static_container_hscale_calculate(c: &dyn ContainerImpl) -> f32 {
        let scroll = cast!(c as ScrollAreaExt).unwrap();
        match scroll.get_area() {
            Some(area) => area.hscale(),
            None => SCALE_DISMISS,
        }
    }

    #[inline]
    fn static_container_vscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_ADAPTION
    }
}

impl ChildContainerDiffRender for ScrollArea {
    fn container_diff_render(&mut self, _painter: &mut Painter) {}
}
