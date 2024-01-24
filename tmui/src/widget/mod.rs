pub mod widget_ext;
pub mod widget_inner;

use crate::{
    application_window::ApplicationWindow,
    graphics::{
        border::Border,
        drawing_context::DrawingContext,
        element::{ElementImpl, HierachyZ},
        painter::Painter,
        render_difference::RenderDiffence,
    },
    layout::LayoutManager,
    prelude::*,
    skia_safe, opti::tracker::Tracker,
};
use derivative::Derivative;
use log::error;
use std::{ptr::NonNull, slice::Iter};
use tlib::{
    bitflags::bitflags,
    emit,
    events::{InputMethodEvent, KeyEvent, MouseEvent, ReceiveCharacterEvent},
    figure::Color,
    namespace::{Align, BlendMode, Coordinate},
    object::{ObjectImpl, ObjectSubclass},
    signals,
    skia_safe::{region::RegionOp, ClipOp},
};

pub type Transparency = u8;

#[extends(Element)]
pub struct Widget {
    parent: Option<NonNull<dyn WidgetImpl>>,
    child: Option<Box<dyn WidgetImpl>>,
    child_ref: Option<NonNull<dyn WidgetImpl>>,

    old_image_rect: Rect,
    child_image_rect_union: Rect,
    child_overflow_rect: Rect,
    need_update_geometry: bool,

    #[derivative(Default(value = "true"))]
    repaint_when_resize: bool,

    initialized: bool,
    first_rendered: bool,
    rerender_difference: bool,

    #[derivative(Default(value = "Color::WHITE"))]
    background: Color,
    font: Font,
    font_family: String,
    margins: [i32; 4],
    paddings: [i32; 4],
    border: Border,

    width_request: i32,
    height_request: i32,

    detecting_width: i32,
    detecting_height: i32,

    /// Widget's width was fixed or not,
    /// `true` when user invoke [`width_request`](WidgetExt::width_request)
    fixed_width: bool,
    /// Widget's height was fixed or not,
    /// `true` when user invoke [`height_request`](WidgetExt::height_request)
    fixed_height: bool,
    /// Used in conjunction with the function [`hexpand`],
    /// if widget was width fixed and hexpanded, `the width ration = width / parent_width`
    fixed_width_ration: f32,
    /// Used in conjunction with the function [`vexpand`],
    /// if widget was height fixed and vexpanded, `the height ration = height / parent_height`
    fixed_height_ration: f32,

    /// Horizontal scalability, if `true` can cause child widget to expand horizontally
    /// with changes in the width of the parent widget.
    hexpand: bool,
    /// The scale factor on horizontal, ratio of child width to parent component,
    /// only effective when widget's `hexpand was true` and `fixed_width was false`.
    ///
    /// ### when parent was widget:
    /// `width ration = hscale / 1`
    ///
    /// ### when parent was coontainer:
    /// `width ration = hscale / parent_children_total_hscales`
    #[derivative(Default(value = "1."))]
    hscale: f32,
    /// Vertical scalability, if `true` can cause child widget to expand vertically
    /// height changes in the height of the parent widget.
    vexpand: bool,
    /// The scale factor on vertical, ratio of child height to parent component,
    /// only effective when widget's hexpand was true.
    ///
    /// ### when parent was widget:
    /// `height ration = vsclae / 1`
    ///
    /// ### when parent was coontainer:
    /// `height ration = vscale / parent_children_total_vscales`
    #[derivative(Default(value = "1."))]
    vscale: f32,

    /// 0: minimum size hint <br>
    /// 1: maximum size hint
    ///
    /// For minimum size: default minimum size is `None`, the widget's minimum size is `(0, 0)`. <br>
    /// For Maximun size: default maximum size is `None`, indicates that the size is infinitely large when possible.
    ///
    /// Container or layout management will determine how to adjust the size and layout of the component appropriately
    /// based on these information.
    ///
    /// If the widget was contained by the container, the size hint is only a reference condition for container size management,
    /// and the actual final size of the component is also obtained by referring to the layout information
    /// of all other subcomponents actually contained in the container.
    ///
    /// Container's layout adjust logic based on their [`strict_children_layout`](crate::container::Container::strict_children_layout) attribute. <br>
    /// `false`: <br>
    /// - Container layout will attempt to respect the `size_hint` of each subcomponent.
    ///   But when space is insufficient, it will compress these components,
    ///   which may result in them being smaller than the size specified by their `size_hint`.
    ///
    /// `true`: <br>
    /// - Container layout will strictly respect the `size_hint` of each subcomponent,
    ///   the parts beyond the size range will be hidden.
    size_hint: SizeHint,

    #[derivative(Default(value = "EventBubble::empty()"))]
    event_bubble: EventBubble,
    /// When true, widget will propagate it's [`event_bubble`] setting to child automatically.
    propagate_event_bubble: bool,
    /// When true, widget will propagate it's [`mouse_tracking`] setting to child automatically.
    propagate_mouse_tracking: bool,

    #[derivative(Default(value = "true"))]
    strict_clip_widget: bool,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EventBubble: u8 {
        const MOUSE_PRESSED = 1;
        const MOUSE_RELEASED = 1 << 1;
        const MOUSE_MOVE = 1 << 2;
        const MOUSE_WHEEL = 1 << 3;
        const KEY_PRESSED = 1 << 4;
        const KEY_RELEASED = 1 << 5;
    }
}

////////////////////////////////////// Widget Signals //////////////////////////////////////
pub trait WidgetSignals: ActionExt {
    signals! {
        WidgetSignals:

        /// Emit when widget's size changed.
        ///
        /// @param [`Size`]
        size_changed();

        /// Emit when widget's geometry(size or position) changed.
        ///
        /// @param [`Rect`]
        geometry_changed();

        /// Emit when widget's receive mouse pressed event.
        ///
        /// @param [`MouseEvent`]
        mouse_pressed();

        /// Emit when widget's receive mouse released event.
        ///
        /// @param [`MouseEvent`]
        mouse_released();

        /// Emit when widget's receive mouse double click event.
        ///
        /// @param [`MouseEvent`]
        mouse_double_click();

        /// Emit when widget's receive mouse move event.
        ///
        /// @param [`MouseEvent`]
        mouse_move();

        /// Emit when widget's receive mouse wheel event.
        ///
        /// @param [`MouseEvent`]
        mouse_wheel();

        /// Emit when widget's receive mouse enter event.
        ///
        /// @param [`MouseEvent`]
        mouse_enter();

        /// Emit when widget's receive mouse leave event.
        ///
        /// @param [`MouseEvent`]
        mouse_leave();

        /// Emit when widget's receive key pressed event.
        ///
        /// @param [`KeyEvent`]
        key_pressed();

        /// Emit when widget's receive key released event.
        ///
        /// @param [`KeyEvent`]
        key_released();

        /// Emit when widget's receive character event.
        ///
        /// @param [`ReceiveCharacterEvent`]
        receive_character();

        /// Emit when widget's background changed.
        ///
        /// @param [`Color`]
        background_changed();
    }
}
impl<T: WidgetImpl + ActionExt> WidgetSignals for T {}
impl WidgetSignals for dyn WidgetImpl {}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl Widget {
    #[inline]
    pub fn child_internal<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        ApplicationWindow::initialize_dynamic_component(child.as_mut());

        self.child = Some(child);
        self.child_ref = None;
    }

    #[inline]
    pub fn child_ref_internal(&mut self, child: &mut dyn WidgetImpl) {
        ApplicationWindow::initialize_dynamic_component(child);

        self.child = None;
        self.child_ref = NonNull::new(child);
    }

    /// Notify all the child widget to invalidate.
    #[inline]
    fn notify_invalidate(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.update()
        }
    }

    /// Notify the child to change the visibility.
    #[inline]
    fn notify_visible(&mut self, visible: bool) {
        if let Some(child) = self.get_child_mut() {
            if visible {
                child.show()
            } else {
                child.hide()
            }
        }
    }

    /// Notify the child to change the zindex.
    #[inline]
    fn notify_zindex(&mut self, offset: u32) {
        if let Some(child) = self.get_child_mut() {
            child.set_z_index(child.z_index() + offset);
        }
    }

    /// Notify the child to rerender styles.
    #[inline]
    fn notify_rerender_styles(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.set_rerender_styles(true)
        }
    }

    #[inline]
    fn notify_minimized(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.set_minimized(true)
        }
    }

    #[inline]
    fn notify_propagate_update(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.propagate_update();
        }
    }

    #[inline]
    fn notify_propagate_update_rect(&mut self, rect: CoordRect) {
        if let Some(child) = self.get_child_mut() {
            child.propagate_update_rect(rect);
        }
    }

    #[inline]
    fn notify_propagate_update_styles_rect(&mut self, rect: CoordRect) {
        if let Some(child) = self.get_child_mut() {
            child.propagate_update_styles_rect(rect);
        }
    }

    #[inline]
    fn notify_propagate_animation_progressing(&mut self, is: bool) {
        if let Some(child) = self.get_child_mut() {
            child.propagate_animation_progressing(is)
        }
    }

    #[inline]
    fn notify_propagate_transparency(&mut self, transparency: Transparency) {
        if let Some(child) = self.get_child_mut() {
            child.propagate_set_transparency(transparency)
        }
    }
}

impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";
}

impl ObjectImpl for Widget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_halign(Align::default());
        self.set_valign(Align::default());

        self.show();
    }

    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value);

        match name {
            "width" => {
                let width = value.get::<i32>();
                self.set_fixed_width(width);
            }
            "height" => {
                let height = value.get::<i32>();
                self.set_fixed_height(height);
            }
            "invalidate" => {
                let invalidate = value.get::<bool>();
                if invalidate {
                    // Notify all the child widget to invalidate, preparing rerenderer after.
                    self.notify_invalidate();
                }
            }
            "visible" => {
                let visible = value.get::<bool>();
                self.notify_visible(visible)
            }
            "z_index" => {
                if !ApplicationWindow::window_of(self.window_id()).initialized() {
                    return;
                }
                let new_z_index = value.get::<u32>();
                self.notify_zindex(new_z_index - self.z_index());
            }
            "rerender_styles" => {
                let rerender = value.get::<bool>();
                if rerender {
                    self.notify_rerender_styles()
                }
            }
            "minimized" => {
                let minimized = value.get::<bool>();
                if minimized {
                    self.notify_minimized();
                }
            }
            "propagate_update" => {
                let propagate_update = value.get::<bool>();
                if propagate_update {
                    self.notify_propagate_update();
                }
            }
            "propagate_update_rect" => {
                let rect = value.get::<CoordRect>();
                self.notify_propagate_update_rect(rect);
            }
            "propagate_update_styles_rect" => {
                let rect = value.get::<CoordRect>();
                self.notify_propagate_update_styles_rect(rect);
            }
            "animation_progressing" => {
                let is = value.get::<bool>();
                self.notify_propagate_animation_progressing(is);
            }
            "propagate_transparency" => {
                let transparency = value.get::<Transparency>();
                self.notify_propagate_transparency(transparency);
            }
            _ => {}
        }
    }
}

impl WidgetImpl for Widget {}

/////////////////////////////////////////////////////////////////////////////////
/// Renderering function for Widget.
/////////////////////////////////////////////////////////////////////////////////
impl<T: WidgetImpl + WidgetExt> ElementImpl for T {
    fn on_renderer(&mut self, cr: &DrawingContext) {
        if !self.visible() && !self.is_animation_progressing() {
            return;
        }
        let mut geometry = self.rect();
        if geometry.width() == 0 || geometry.height() == 0 {
            return;
        }
        geometry.set_point(&(0, 0).into());

        let _track = Tracker::start(format!("single_render_{}", self.name()));

        let mut painter = Painter::new(cr.canvas(), self);

        // Shared widget porcessing:
        if let Some(shared_widget) = cast_mut!(self as SharedWidgetImpl) {
            shared_widget.pixels_render(&mut painter);
            return;
        }

        // The default paint blend mode is set to `Src`,
        // it should be set to `SrcOver` when the widget is undergoing animation progress.
        if self.is_animation_progressing() {
            painter.set_blend_mode(BlendMode::SrcOver);
        }

        painter.save();
        painter.clip_region(self.child_region(), ClipOp::Difference);
        if let Some(parent) = self.get_parent_ref() {
            if let Some(_) = cast!(parent as ContainerImpl) {
                painter.clip_rect_global(parent.contents_rect(None), ClipOp::Intersect);
            }
        }
        painter_clip(self, &mut painter, self.styles_redraw_region().iter());

        if !self.first_rendered() || self.rerender_styles() {
            // Draw the background color of the Widget.
            let mut background = self.background();
            background.set_transparency(self.transparency());

            if self.rerender_difference() && self.first_rendered() && !self.window().minimized() {
                let mut border_rect: FRect = self.rect_record().into();
                border_rect.set_point(&(0, 0).into());
                self.border_ref()
                    .clear_border(&mut painter, border_rect, background);

                self.render_difference(&mut painter, background);
            } else {
                painter.fill_rect(geometry, background);
            }

            self.set_rect_record(self.rect());
            self.set_image_rect_record(self.image_rect());

            // Draw the border of the Widget.
            self.border_ref().render(&mut painter, geometry.into());

            self.set_first_rendered();
            self.set_rerender_styles(false);
        }

        painter.reset();
        painter.set_font(self.font().to_skia_font());

        if self.is_strict_clip_widget() {
            painter.clip_rect(
                self.contents_rect(Some(Coordinate::Widget)),
                ClipOp::Intersect,
            );
        }

        painter_clip(self, &mut painter, self.redraw_region().iter());

        if let Some(loading) = cast_mut!(self as Loadable) {
            if loading.is_loading() {
                loading.render_loading(&mut painter);
            } else {
                self.paint(&mut painter);
            }
        } else {
            self.paint(&mut painter);
        }

        painter.restore();
    }
}

#[inline]
pub(crate) fn painter_clip(widget: &dyn WidgetImpl, painter: &mut Painter, iter: Iter<CoordRect>) {
    let mut region = skia_safe::Region::new();
    let mut op = false;
    for r in iter {
        let coord = r.coord();
        let mut r = r.rect();
        if coord != Coordinate::World {
            r.set_point(&widget.map_to_global_f(&r.point()))
        }

        let r: skia_safe::IRect = r.into();
        region.op_rect(r, RegionOp::Union);
        op = true;
    }
    if op {
        painter.clip_region(region, ClipOp::Intersect);
    }
}

pub trait WidgetAcquire: WidgetImpl + Default {}

////////////////////////////////////// WidgetGenericExt //////////////////////////////////////
/// The trait provide some functions include the generic types.
pub trait WidgetGenericExt {
    /// Go to[`Function defination`](WidgetGenericExt::get_parent) (Defined in [`WidgetGenericExt`])
    fn parent_ref<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;

    /// Go to[`Function defination`](WidgetGenericExt::get_child) (Defined in [`WidgetGenericExt`])
    fn child_ref<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;

    /// Go to[`Function defination`](WidgetGenericExt::get_parent) (Defined in [`WidgetGenericExt`])
    fn parent_mut<T: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut T>;

    /// Go to[`Function defination`](WidgetGenericExt::get_child) (Defined in [`WidgetGenericExt`])
    fn child_mut<T: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut T>;
}

impl<T: WidgetImpl> WidgetGenericExt for T {
    fn parent_ref<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
        let raw_parent = self.get_raw_parent();
        match raw_parent {
            Some(parent) => unsafe {
                if parent.as_ref().is_none() {
                    return None;
                }
                if parent
                    .as_ref()
                    .unwrap()
                    .object_type()
                    .is_a(R::static_type())
                {
                    (parent as *const R).as_ref()
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn child_ref<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
        let raw_child = self.get_raw_child();
        match raw_child {
            Some(child) => unsafe {
                if child.as_ref().is_none() {
                    return None;
                }
                if child.as_ref().unwrap().object_type().is_a(R::static_type()) {
                    (child as *const R).as_ref()
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn parent_mut<R: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut R> {
        let raw_parent = self.get_raw_parent_mut();
        match raw_parent {
            Some(parent) => unsafe {
                if parent.as_ref().is_none() {
                    return None;
                }
                if parent
                    .as_mut()
                    .unwrap()
                    .object_type()
                    .is_a(R::static_type())
                {
                    (parent as *mut R).as_mut()
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn child_mut<R: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut R> {
        let raw_child = self.get_raw_child_mut();
        match raw_child {
            Some(child) => unsafe {
                if child.as_ref().is_none() {
                    return None;
                }
                if child.as_ref().unwrap().object_type().is_a(R::static_type()) {
                    (child as *mut R).as_mut()
                } else {
                    None
                }
            },
            None => None,
        }
    }
}

////////////////////////////////////// PointEffective //////////////////////////////////////
pub trait PointEffective {
    /// Is the detection point effective.
    fn point_effective(&self, point: &Point) -> bool;
}
impl PointEffective for Widget {
    fn point_effective(&self, point: &Point) -> bool {
        let self_rect = self.rect();
        if !self_rect.contains(point) {
            return false;
        }

        if let Some(child) = self.get_child_ref() {
            if !child.visible() {
                return true;
            }
            return !child.rect().contains(point);
        }
        true
    }
}

////////////////////////////////////// ChildRegionAcquirer //////////////////////////////////////
pub trait ChildRegionAcquirer {
    fn child_region(&self) -> tlib::skia_safe::Region;
}
impl ChildRegionAcquirer for Widget {
    fn child_region(&self) -> tlib::skia_safe::Region {
        let mut region = tlib::skia_safe::Region::new();
        if let Some(child) = self.get_child_ref() {
            if child.visible() || child.is_animation_progressing() {
                let rect: tlib::skia_safe::IRect = child.rect().into();
                region.op_rect(rect, RegionOp::Replace);
            }
        }
        region
    }
}

////////////////////////////////////// InnerEventProcess //////////////////////////////////////
pub trait InnerEventProcess {
    /// Invoke when widget's receive mouse pressed event.
    fn inner_mouse_pressed(&mut self, event: &MouseEvent, bubbled: bool);

    /// Invoke when widget's receive mouse released event.
    fn inner_mouse_released(&mut self, event: &MouseEvent, bubbled: bool);

    /// Invoke when widget's receive mouse move event.
    fn inner_mouse_move(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse wheel event.
    fn inner_mouse_wheel(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse enter event.
    fn inner_mouse_enter(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse leave event.
    fn inner_mouse_leave(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive key pressed event.
    fn inner_key_pressed(&mut self, event: &KeyEvent);

    /// Invoke when widget's receive key released event.
    fn inner_key_released(&mut self, event: &KeyEvent);

    /// Invoke when widget's receive character event.
    fn inner_receive_character(&mut self, event: &ReceiveCharacterEvent);
}
impl<T: WidgetImpl + WidgetSignals> InnerEventProcess for T {
    #[inline]
    fn inner_mouse_pressed(&mut self, event: &MouseEvent, bubbled: bool) {
        if !bubbled {
            if self.enable_focus() {
                self.set_focus(true)
            }
            self.window().set_pressed_widget(self.id());
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_pressed(event)
        }

        emit!(Widget::inner_mouse_pressed => self.mouse_pressed(), event);

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_PRESSED) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = event.clone();
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_pressed(&evt);
            parent.inner_mouse_pressed(&evt, true);
        }
    }

    #[inline]
    fn inner_mouse_released(&mut self, event: &MouseEvent, bubbled: bool) {
        if !bubbled {
            self.window().set_pressed_widget(0);
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_released(event)
        }

        emit!(Widget::inner_mouse_released => self.mouse_released(), event);

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_RELEASED) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = event.clone();
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_released(&evt);
            parent.inner_mouse_released(&evt, true);
        }
    }

    #[inline]
    fn inner_mouse_move(&mut self, event: &MouseEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_move(event)
        }

        emit!(Widget::inner_mouse_move => self.mouse_move(), event);

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_MOVE) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = event.clone();
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_move(&evt);
            parent.inner_mouse_move(&evt);
        }
    }

    #[inline]
    fn inner_mouse_wheel(&mut self, event: &MouseEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_wheel(event)
        }

        emit!(Widget::inner_mouse_wheel => self.mouse_wheel(), event);

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_WHEEL) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = event.clone();
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_wheel(&evt);
            parent.inner_mouse_wheel(&evt);
        }
    }

    #[inline]
    fn inner_mouse_enter(&mut self, event: &MouseEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_enter(event)
        }

        emit!(Widget::inner_mouse_enter => self.mouse_enter(), event);
    }

    #[inline]
    fn inner_mouse_leave(&mut self, event: &MouseEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_leave(event)
        }

        emit!(Widget::inner_mouse_leave => self.mouse_leave(), event);
    }

    #[inline]
    fn inner_key_pressed(&mut self, event: &KeyEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_key_pressed(event)
        }

        emit!(Widget::inner_key_pressed => self.key_pressed(), event);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::KEY_PRESSED) {
                return;
            }

            parent.on_key_pressed(event);
            parent.inner_key_pressed(event);
        }
    }

    #[inline]
    fn inner_key_released(&mut self, event: &KeyEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_key_released(event)
        }

        emit!(Widget::inner_key_released => self.key_released(), event);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::KEY_RELEASED) {
                return;
            }

            parent.on_key_released(event);
            parent.inner_key_released(event);
        }
    }

    #[inline]
    fn inner_receive_character(&mut self, event: &ReceiveCharacterEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_receive_character(event)
        }

        emit!(Widget::inner_receive_character => self.receive_character(), event);
    }
}

#[reflect_trait]
#[allow(unused_variables)]
pub trait InnerCustomizeEventProcess {
    /// Invoke when widget's receive mouse pressed event.
    fn inner_customize_mouse_pressed(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse released event.
    fn inner_customize_mouse_released(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse move event.
    fn inner_customize_mouse_move(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse wheel event.
    fn inner_customize_mouse_wheel(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse enter event.
    fn inner_customize_mouse_enter(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse leave event.
    fn inner_customize_mouse_leave(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive key pressed event.
    fn inner_customize_key_pressed(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive key released event.
    fn inner_customize_key_released(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive character event.
    fn inner_customize_receive_character(&mut self, event: &ReceiveCharacterEvent) {}
}

////////////////////////////////////// WidgetImpl //////////////////////////////////////
/// Every struct modified by proc-macro [`extends_widget`] should impl this trait manually.
/// WidgetImpl's `paint()` function Will be proxy executated by [`ElementImpl::on_renderer`] method .
#[allow(unused_variables)]
#[allow(unused_mut)]
#[reflect_trait]
pub trait WidgetImpl:
    WidgetExt
    + ElementImpl
    + ElementExt
    + ObjectOperation
    + ObjectType
    + ObjectImpl
    + SuperType
    + Layout
    + InnerEventProcess
    + PointEffective
    + ChildRegionAcquirer
    + ActionExt
{
    /// The widget can be focused or not, default value was false.
    fn enable_focus(&self) -> bool {
        false
    }

    /// Invoke this function when renderering.
    fn paint(&mut self, painter: &mut Painter) {}

    /// Invoke when widget's font was changed.
    fn font_changed(&mut self) {}

    /// `run_after()` will be invoked when application was started. <br>
    ///
    /// ### Should annotated macro `[run_after]` to enable this function.
    ///
    /// ### Should call `self.parent_run_after()` mannually if override this function.
    fn run_after(&mut self) {
        self.parent_run_after();
    }

    /// Invoke when widget's receive mouse pressed event.
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse released event.
    fn on_mouse_released(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse move event.
    ///
    /// The widget does not track mouse movement by default. If need, call function [`set_mouse_tracking`](WidgetExt::set_mouse_tracking)
    fn on_mouse_move(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse wheel event.
    fn on_mouse_wheel(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse enter event.
    fn on_mouse_enter(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse leave event.
    fn on_mouse_leave(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive key pressed event.
    fn on_key_pressed(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive key released event.
    fn on_key_released(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive character event.
    fn on_receive_character(&mut self, event: &ReceiveCharacterEvent) {}

    /// Invoke when widget's receive input method event.
    fn on_input_method(&mut self, input_method: &InputMethodEvent) {}
}

impl dyn WidgetImpl {
    #[inline]
    pub fn as_ptr(&self) -> *const Self {
        self
    }

    #[inline]
    pub fn as_ptr_mut(&mut self) -> *mut Self {
        self
    }

    #[inline]
    pub fn is<T: StaticType + 'static>(&self) -> bool {
        self.object_type().is_a(T::static_type()) && self.as_any().is::<T>()
    }

    #[inline]
    pub fn downcast_ref<T: StaticType + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            self.as_any().downcast_ref::<T>()
        } else {
            error!(
                "Downcast widget type mismatched, require {}, get {}",
                self.object_type().name(),
                T::static_type().name()
            );
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: StaticType + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            self.as_any_mut().downcast_mut::<T>()
        } else {
            error!(
                "Downcast widget type mismatched, require {}, get {}",
                self.object_type().name(),
                T::static_type().name()
            );
            None
        }
    }

    #[inline]
    pub fn downcast<T: StaticType + 'static>(self: Box<Self>) -> Option<Box<T>> {
        let require = self.object_type().name();
        match self.as_any_boxed().downcast::<T>() {
            Ok(v) => Some(v),
            _ => {
                error!(
                    "Downcast widget type mismatched, require {}, get {}",
                    require,
                    T::static_type().name()
                );
                None
            }
        }
    }
}

pub trait WidgetImplExt: WidgetImpl {
    /// @see [`Widget::child_internal`](Widget) <br>
    /// Go to[`Function defination`](WidgetImplExt::child) (Defined in [`WidgetImplExt`])
    fn child<T: WidgetImpl>(&mut self, child: Box<T>);

    /// @see [`Widget::child_ref_internal`](Widget) <br>
    /// Go to[`Function defination`](WidgetImplExt::child_ref) (Defined in [`WidgetImplExt`])
    fn _child_ref(&mut self, child: *mut dyn WidgetImpl);
}

////////////////////////////////////// Widget Layouts impl //////////////////////////////////////
impl<T: WidgetAcquire> Layout for T {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(self, previous, parent, manage_by_container)
    }
}

impl Layout for Widget {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    fn position_layout(&mut self, _: Option<&dyn WidgetImpl>, _: Option<&dyn WidgetImpl>, _: bool) {
    }
}

////////////////////////////////////// ZInddexStep //////////////////////////////////////
pub(crate) trait ZIndexStep {
    /// Get current widget's z-index step, starts from 1, `auto-increacement`.
    fn z_index_step(&mut self) -> u32;
}
macro_rules! z_index_step_impl {
    () => {
        #[inline]
        fn z_index_step(&mut self) -> u32 {
            let step = match self.get_property("z_index_step") {
                Some(val) => val.get(),
                None => 1,
            };
            self.set_property("z_index_step", (step + 1).to_value());
            step
        }
    };
}
impl<T: WidgetImpl> ZIndexStep for T {
    z_index_step_impl!();
}
impl ZIndexStep for dyn WidgetImpl {
    z_index_step_impl!();
}

////////////////////////////////////// ScaleCalculate //////////////////////////////////////
pub(crate) trait ScaleCalculate {
    #[inline]
    fn hscale_calculate(&self) -> f32 {
        1.
    }

    #[inline]
    fn vscale_calculate(&self) -> f32 {
        1.
    }
}

impl ScaleCalculate for dyn WidgetImpl {}

////////////////////////////////////// WindowAcquire //////////////////////////////////////
pub trait WindowAcquire {
    fn window(&self) -> &'static mut ApplicationWindow;
}
impl<T: WidgetImpl> WindowAcquire for T {
    #[inline]
    fn window(&self) -> &'static mut ApplicationWindow {
        ApplicationWindow::window_of(self.window_id())
    }
}
impl WindowAcquire for dyn WidgetImpl {
    #[inline]
    fn window(&self) -> &'static mut ApplicationWindow {
        ApplicationWindow::window_of(self.window_id())
    }
}

#[cfg(test)]
mod tests {
    use super::WidgetImpl;
    use crate::{prelude::*, skia_safe, widget::WidgetGenericExt};
    use tlib::{
        object::{ObjectImpl, ObjectSubclass},
        skia_safe::region::RegionOp,
    };

    #[extends(Widget)]
    struct SubWidget {}

    impl ObjectSubclass for SubWidget {
        const NAME: &'static str = "SubWidget";
    }

    impl ObjectImpl for SubWidget {}

    impl WidgetImpl for SubWidget {}

    #[extends(Widget)]
    struct ChildWidget {}

    impl ObjectSubclass for ChildWidget {
        const NAME: &'static str = "ChildWidget";
    }

    impl ObjectImpl for ChildWidget {}

    impl WidgetImpl for ChildWidget {}

    #[test]
    fn test_sub_widget() {
        let mut widget: Box<SubWidget> = Object::new(&[("width", &&120), ("height", &&80)]);
        assert_eq!(120, widget.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, widget.get_property("height").unwrap().get::<i32>());

        let child: Box<ChildWidget> = Object::new(&[("width", &&120), ("height", &&80)]);
        let child_id = child.id();

        widget.child(child);

        let child_ref = widget.child_ref::<ChildWidget>().unwrap();
        assert_eq!(child_ref.id(), child_id);
        assert_eq!(120, child_ref.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, child_ref.get_property("height").unwrap().get::<i32>());
    }

    #[test]
    fn test_skia_region() {
        let mut region = skia_safe::Region::new();
        let rect1 = skia_safe::IRect::new(0, 0, 100, 100);
        let rect2 = skia_safe::IRect::new(0, 0, 400, 400);
        region.op_rect(rect1, RegionOp::Union);
        region.op_rect(rect2, RegionOp::Difference);
    }
}
