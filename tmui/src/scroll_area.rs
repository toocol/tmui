use crate::{
    application_window::ApplicationWindow,
    layout::LayoutManager,
    prelude::*,
    scroll_bar::{ScrollBar, ScrollBarPosition, DEFAULT_SCROLL_BAR_WIDTH},
};
use derivative::Derivative;
use tlib::{
    events::{DeltaType, MouseEvent},
    namespace::{KeyboardModifier, Orientation},
    object::ObjectSubclass,
    prelude::extends,
};

#[extends(Container)]
pub struct ScrollArea {
    #[derivative(Default(value = "Object::new(&[])"))]
    scroll_bar: Box<ScrollBar>,
    area: Option<Box<dyn WidgetImpl>>,
}

impl ScrollArea {
    #[inline]
    pub fn set_area<T: WidgetImpl>(&mut self, mut area: Box<T>) {
        ApplicationWindow::initialize_dynamic_component(area.as_mut());
        area.set_parent(self);
        self.area = Some(area)
    }

    #[inline]
    pub fn get_area(&self) -> Option<&dyn WidgetImpl> {
        self.area.as_ref().and_then(|w| Some(w.as_ref()))
    }

    #[inline]
    pub fn get_area_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
        self.area.as_mut().and_then(|w| Some(w.as_mut()))
    }

    #[inline]
    pub fn get_area_cast<T: WidgetImpl + ObjectSubclass>(&self) -> Option<&T> {
        self.area.as_ref().and_then(|w| w.downcast_ref::<T>())
    }

    #[inline]
    pub fn get_area_cast_mut<T: WidgetImpl + ObjectSubclass>(&mut self) -> Option<&mut T> {
        self.area.as_mut().and_then(|w| w.downcast_mut::<T>())
    }

    #[inline]
    pub fn get_scroll_bar(&self) -> &ScrollBar {
        &self.scroll_bar
    }

    #[inline]
    pub fn get_scroll_bar_mut(&mut self) -> &mut ScrollBar {
        &mut self.scroll_bar
    }

    #[inline]
    pub fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition) {
        self.scroll_bar.set_scroll_bar_position(scroll_bar_position)
    }

    #[inline]
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.scroll_bar.set_orientation(orientation)
    }

    /// Scroll the widget. </br>
    /// delta was positive value when scroll down/right.
    #[inline]
    pub fn scroll(&mut self, delta: i32, delta_type: DeltaType) {
        self.scroll_bar
            .scroll_by_delta(KeyboardModifier::NoModifier, delta, delta_type);
    }
}

impl ObjectSubclass for ScrollArea {
    const NAME: &'static str = "ScrollArea";
}

impl ObjectImpl for ScrollArea {
    fn construct(&mut self) {
        self.parent_construct();

        ApplicationWindow::initialize_dynamic_component(self.scroll_bar.as_mut());
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
        match self.scroll_bar.orientation() {
            Orientation::Horizontal => Composition::VerticalArrange,
            Orientation::Vertical => Composition::HorizontalArrange,
        }
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(self, previous, parent, manage_by_container);

        // Deal with the area and scroll bar's position:
        let rect = self.rect();
        let scroll_bar = &mut self.scroll_bar;
        match scroll_bar.scroll_bar_position() {
            ScrollBarPosition::Start => {
                scroll_bar.set_fixed_x(rect.x() + scroll_bar.margin_left());
                scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());
                if let Some(ref mut area) = self.area {
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
                if let Some(ref mut area) = self.area {
                    area.set_fixed_x(rect.x() + area.margin_left());
                    area.set_fixed_y(rect.y() + area.margin_top());

                    let area_rect = area.rect();
                    scroll_bar.set_fixed_x(
                        rect.x() + rect.width() + scroll_bar.margin_left()
                            - DEFAULT_SCROLL_BAR_WIDTH,
                    );
                    scroll_bar.set_fixed_y(area_rect.y() + scroll_bar.margin_top());
                } else {
                    scroll_bar.set_fixed_x(
                        rect.x() + rect.width() + scroll_bar.margin_left()
                            - DEFAULT_SCROLL_BAR_WIDTH,
                    );
                    scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());
                }
            }
        }
    }
}
