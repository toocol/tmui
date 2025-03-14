use crate::{
    application_window::ApplicationWindow,
    container::{ContainerLayoutEnum, ContainerScaleCalculate, SCALE_ADAPTION},
    layout::LayoutMgr,
    prelude::*,
    scroll_bar::{ScrollBar, ScrollBarPosition},
};
use derivative::Derivative;
use tlib::{
    connect, disconnect,
    events::{DeltaType, MouseEvent},
    namespace::{KeyboardModifier, Orientation},
    object::ObjectSubclass,
    prelude::extends,
    ptr_mut,
};

/// The layout mode of scroll bar.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum LayoutMode {
    /// Default layout mode,
    /// the ScrollBar and Area Widget each occupy different sizes of the ScrollArea space.
    ///
    /// At this point, the hide/show operation of the ScrollBar
    /// will cause changes in the size of the Area Widget.
    #[default]
    Normal,

    /// The ScrollBar overlays the Area Widget,
    /// and at this point, the Area Widget occupies the entire space of the ScrollArea.
    ///
    /// At this point, the hide/show operation of the ScrollBar
    /// will `not` cause changes in the size of the Area Widget.
    Overlay,
}

#[extends(Container)]
pub struct ScrollArea {
    layout_mode: LayoutMode,
}

/////////////////////////////////////////// Start: ScrollArea self implementations ///////////////////////////////////////////
#[reflect_trait]
pub trait ScrollAreaExt: WidgetImpl {
    fn area(&self) -> Option<&dyn WidgetImpl>;

    fn area_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    fn scroll_bar(&self) -> &ScrollBar;

    fn scroll_bar_mut(&mut self) -> &mut ScrollBar;

    fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition);

    fn set_orientation(&mut self, orientation: Orientation);

    fn scroll(&mut self, delta: i32, delta_type: DeltaType);

    fn layout_mode(&self) -> LayoutMode;

    fn set_layout_mode(&mut self, layout_mode: LayoutMode);
}

pub trait ScrollAreaGenericExt {
    fn set_area<T: WidgetImpl>(&mut self, area: Box<T>);

    fn get_area_cast<T: WidgetImpl + ObjectSubclass>(&self) -> Option<&T>;

    fn get_area_cast_mut<T: WidgetImpl + ObjectSubclass>(&mut self) -> Option<&mut T>;
}

pub trait ScrollAreaSlots: ScrollAreaExt {
    fn update_scroll_bar(&mut self) {
        let scroll_bar_rect = self.scroll_bar().rect_f();

        let area = self.area().unwrap();
        let mut need_update = area.redraw_region().is_empty();

        let mut styles_region = CoordRegion::new();
        for coord in area.redraw_region().iter() {
            let rect = if coord.coord() == Coordinate::Widget {
                let mut rect = coord.rect();
                rect.set_point(&self.map_to_global_f(&rect.point()));
                CoordRect::new(rect, Coordinate::World)
            } else {
                *coord
            };

            if !rect.rect().is_intersects(&scroll_bar_rect) {
                continue;
            }

            need_update = true;
            styles_region.add_rect(rect);
        }

        if !need_update {
            return;
        }

        let scroll_bar = self.scroll_bar_mut();
        if !scroll_bar.update_region(&styles_region) {
            scroll_bar.update()
        }
    }
}
impl<T: ScrollAreaExt> ScrollAreaSlots for T {}

impl ScrollAreaExt for ScrollArea {
    #[inline]
    fn area(&self) -> Option<&dyn WidgetImpl> {
        if self.container.children.len() == 1 {
            return None;
        }
        self.container.children.first().map(|w| w.as_ref())
    }

    #[inline]
    fn area_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
        if self.container.children.len() == 1 {
            return None;
        }
        self.container.children.first_mut().map(|w| w.as_mut())
    }

    #[inline]
    fn scroll_bar(&self) -> &ScrollBar {
        self.container
            .children
            .last()
            .map(|w| w.downcast_ref::<ScrollBar>().unwrap())
            .unwrap()
    }

    #[inline]
    fn scroll_bar_mut(&mut self) -> &mut ScrollBar {
        self.container
            .children
            .last_mut()
            .map(|w| w.downcast_mut::<ScrollBar>().unwrap())
            .unwrap()
    }

    #[inline]
    fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition) {
        self.scroll_bar_mut()
            .set_scroll_bar_position(scroll_bar_position);
        self.update();
    }

    #[inline]
    fn set_orientation(&mut self, orientation: Orientation) {
        self.scroll_bar_mut().set_orientation(orientation);
        self.window().layout_change(self.scroll_bar_mut());
    }

    /// Scroll the widget. </br>
    /// delta was positive value when scroll down/right.
    #[inline]
    fn scroll(&mut self, delta: i32, delta_type: DeltaType) {
        self.scroll_bar_mut()
            .scroll_by_delta(KeyboardModifier::NoModifier, delta, delta_type);
    }

    #[inline]
    fn layout_mode(&self) -> LayoutMode {
        self.layout_mode
    }

    #[inline]
    fn set_layout_mode(&mut self, layout_mode: LayoutMode) {
        self.layout_mode = layout_mode;
        self.scroll_bar_mut()
            .set_occupy_space(layout_mode == LayoutMode::Normal);
        self.scroll_bar_mut()
            .set_overlaid(layout_mode == LayoutMode::Overlay);
        let layout_mode = self.layout_mode;

        if self.area().is_some() {
            if layout_mode == LayoutMode::Normal {
                disconnect!(self.area_mut().unwrap(), invalidated(), self, null);
                disconnect!(
                    self.scroll_bar_mut(),
                    geometry_changed(),
                    self.area_mut().unwrap(),
                    null
                );
                disconnect!(
                    self.scroll_bar_mut(),
                    need_update(),
                    self.area_mut().unwrap(),
                    null
                );
                self.area_mut().unwrap().set_invalid_area(FRect::default());
            } else {
                connect!(
                    self.area_mut().unwrap(),
                    invalidated(),
                    self,
                    update_scroll_bar()
                );
                connect!(
                    self.scroll_bar_mut(),
                    geometry_changed(),
                    self.area_mut().unwrap(),
                    set_invalid_area(FRect)
                );
                connect!(
                    self.scroll_bar_mut(),
                    need_update(),
                    self.area_mut().unwrap(),
                    update()
                );
            }
        }

        // Area'a overlaid rect will be changed when re-layout.
        self.window().layout_change(self.scroll_bar_mut())
    }
}

impl ScrollAreaGenericExt for ScrollArea {
    #[inline]
    fn set_area<T: WidgetImpl>(&mut self, mut area: Box<T>) {
        area.set_parent(self);
        area.set_vexpand(true);
        area.set_hexpand(true);
        if self.layout_mode == LayoutMode::Overlay {
            connect!(area, invalidated(), self, update_scroll_bar());
            connect!(
                self.scroll_bar_mut(),
                geometry_changed(),
                area,
                set_invalid_area(FRect)
            );
        }

        ApplicationWindow::initialize_dynamic_component(area.as_mut(), self.is_in_tree());
        self.container.children.insert(0, area)
    }

    #[inline]
    fn get_area_cast<T: WidgetImpl + ObjectSubclass>(&self) -> Option<&T> {
        self.area().and_then(|w| w.downcast_ref::<T>())
    }

    #[inline]
    fn get_area_cast_mut<T: WidgetImpl + ObjectSubclass>(&mut self) -> Option<&mut T> {
        self.area_mut().and_then(|w| w.downcast_mut::<T>())
    }
}
/////////////////////////////////////////// End: ScrollArea self implementations ///////////////////////////////////////////

impl ObjectSubclass for ScrollArea {
    const NAME: &'static str = "ScrollArea";
}

impl ObjectImpl for ScrollArea {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_render_difference(true);
        self.container
            .children
            .push(ScrollBar::new(Orientation::Vertical));

        let occupy_space = self.layout_mode == LayoutMode::Normal;
        self.scroll_bar_mut().set_occupy_space(occupy_space);

        let parent = self as *mut dyn WidgetImpl;
        self.scroll_bar_mut().set_parent(parent);
    }

    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<ScrollArea, ReflectScrollAreaExt>();
    }
}

impl WidgetImpl for ScrollArea {
    #[inline]
    fn on_mouse_wheel(&mut self, event: &MouseEvent) {
        self.scroll_bar_mut().on_mouse_wheel(event)
    }
}

impl ContainerImpl for ScrollArea {
    #[inline]
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.container.children.iter().map(|w| w.as_ref()).collect()
    }

    #[inline]
    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.container
            .children
            .iter_mut()
            .map(|w| w.as_mut())
            .collect()
    }

    #[inline]
    fn container_layout(&self) -> ContainerLayoutEnum {
        ContainerLayoutEnum::ScrollArea
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
    #[inline]
    fn composition(&self) -> Composition {
        Self::static_composition(self)
    }

    #[inline]
    fn position_layout(&mut self, parent: Option<&dyn WidgetImpl>) {
        Self::container_position_layout(self, parent);
    }
}

impl ContainerLayout for ScrollArea {
    #[inline]
    fn static_composition<T: WidgetImpl + ContainerImpl>(widget: &T) -> Composition {
        let widget = cast!(widget as ScrollAreaExt).unwrap();
        match widget.layout_mode() {
            LayoutMode::Normal => match widget.scroll_bar().orientation() {
                Orientation::Horizontal => Composition::VerticalArrange,
                Orientation::Vertical => Composition::HorizontalArrange,
            },
            LayoutMode::Overlay => Composition::Stack,
        }
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        parent: Option<&dyn WidgetImpl>,
    ) {
        LayoutMgr::base_widget_position_layout(widget, parent);

        let widget = cast_mut!(widget as ScrollAreaExt).unwrap();

        match widget.layout_mode() {
            LayoutMode::Normal => layout_normal(widget),
            LayoutMode::Overlay => layout_overlay(widget),
        }
    }
}

fn layout_normal(widget: &mut dyn ScrollAreaExt) {
    // Deal with the area and scroll bar's position:
    let rect = widget.rect();
    let scroll_bar = ptr_mut!(widget as *mut dyn ScrollAreaExt).scroll_bar_mut();
    match scroll_bar.scroll_bar_position() {
        ScrollBarPosition::Start => {
            scroll_bar.set_fixed_x(rect.x() + scroll_bar.margin_left());
            scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());

            if let Some(area) = widget.area_mut() {
                let scroll_bar_rect: Rect = scroll_bar.shadow_rect().into();
                area.set_fixed_x(
                    scroll_bar_rect.x() + scroll_bar_rect.width() + area.margin_left(),
                );
                area.set_fixed_y(scroll_bar_rect.y() + scroll_bar_rect.width() + area.margin_top());
            }
        }
        ScrollBarPosition::End => {
            if let Some(area) = widget.area_mut() {
                area.set_fixed_x(rect.x() + area.margin_left());
                area.set_fixed_y(rect.y() + area.margin_top());

                let area_rect = area.rect();
                match scroll_bar.orientation() {
                    Orientation::Vertical => {
                        let offset = if scroll_bar.auto_hide() && !scroll_bar.visible() {
                            scroll_bar.shadow_rect().width() as i32
                        } else {
                            0
                        };
                        scroll_bar.set_fixed_x(
                            rect.x() + area_rect.width() + scroll_bar.margin_left() - offset,
                        );
                        scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());
                    }
                    Orientation::Horizontal => {
                        let offset = if scroll_bar.auto_hide() && !scroll_bar.visible() {
                            scroll_bar.shadow_rect().height() as i32
                        } else {
                            0
                        };
                        scroll_bar.set_fixed_x(rect.x() + scroll_bar.margin_left());
                        scroll_bar.set_fixed_y(
                            rect.y() + area_rect.height() + scroll_bar.margin_top() - offset,
                        );
                    }
                }
            } else {
                let scroll_bar_rect: Rect = scroll_bar.shadow_rect().into();
                widget.scroll_bar_mut().set_fixed_x(
                    rect.x() + rect.width() + scroll_bar.margin_left() - scroll_bar_rect.width(),
                );
                scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());
            }
        }
    }
}

fn layout_overlay(widget: &mut dyn ScrollAreaExt) {
    let rect = widget.rect();
    let scroll_bar = ptr_mut!(widget as *mut dyn ScrollAreaExt).scroll_bar_mut();
    match scroll_bar.scroll_bar_position() {
        ScrollBarPosition::Start => {
            scroll_bar.set_fixed_x(rect.x() + scroll_bar.margin_left());
            scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());

            if let Some(area) = widget.area_mut() {
                area.set_fixed_x(rect.x() + area.margin_left());
                area.set_fixed_y(rect.y() + area.margin_top());
            }
        }
        ScrollBarPosition::End => {
            if let Some(area) = widget.area_mut() {
                area.set_fixed_x(rect.x() + area.margin_left());
                area.set_fixed_y(rect.y() + area.margin_top());
            }

            let scroll_bar_rect: Rect = scroll_bar.shadow_rect().into();
            match scroll_bar.orientation() {
                Orientation::Vertical => {
                    scroll_bar.set_fixed_x(
                        rect.x() + rect.width() + scroll_bar.margin_left()
                            - scroll_bar_rect.width(),
                    );
                    scroll_bar.set_fixed_y(rect.y() + scroll_bar.margin_top());
                }
                Orientation::Horizontal => {
                    scroll_bar.set_fixed_x(rect.x() + scroll_bar.margin_left());
                    scroll_bar.set_fixed_y(
                        rect.y() + rect.height() + scroll_bar.margin_top()
                            - scroll_bar_rect.height(),
                    );
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
    fn static_container_hscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_ADAPTION
    }

    #[inline]
    fn static_container_vscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_ADAPTION
    }
}
