pub mod callbacks;
pub mod widget_ext;
pub mod widget_inner;
pub mod win_widget;

use self::{callbacks::Callbacks, widget_inner::WidgetInnerExt};
use crate::{
    application_window::ApplicationWindow,
    graphics::{
        box_shadow::ShadowRender,
        drawing_context::DrawingContext,
        element::{ElementImpl, HierachyZ},
        painter::Painter,
        render_difference::RenderDiffence,
        styles::InnerStyles,
    },
    layout::LayoutMgr,
    opti::tracker::Tracker,
    overlay::ReflectPartCovered,
    prelude::*,
    skia_safe,
};
use derivative::Derivative;
use log::error;
#[cfg(verbose_logging)]
use log::info;
use nohash_hasher::IntSet;
use std::{ptr::NonNull, slice::Iter};
use tlib::{
    bitflags::bitflags,
    emit,
    events::{InputMethodEvent, KeyEvent, MouseEvent, ReceiveCharacterEvent},
    figure::Color,
    namespace::{Align, BlendMode, Coordinate, Overflow},
    nonnull_mut, nonnull_ref,
    object::{ObjectImpl, ObjectSubclass},
    ptr_mut, signals,
    skia_safe::{region::RegionOp, ClipOp},
    typedef::SkiaRect,
};

pub type Transparency = u8;
pub type WidgetHnd = Option<NonNull<dyn WidgetImpl>>;

#[extends(Element)]
pub struct Widget {
    parent: WidgetHnd,

    child: Option<DynTr>,
    children_index: IntSet<ObjectId>,

    old_rect: FRect,
    old_image_rect: FRect,
    child_image_rect_union: FRect,
    child_overflow_rect: FRect,
    invalid_area: FRect,
    need_update_geometry: bool,
    redraw_region: CoordRegion,
    whole_styles_render: bool,

    #[derivative(Default(value = "true"))]
    repaint_when_resize: bool,

    initialized: bool,
    first_rendered: bool,
    #[derivative(Default(value = "false"))]
    rerender_difference: bool,
    redraw_shadow_box: bool,
    in_tree: bool,
    overflow: Overflow,

    margins: [i32; 4],
    paddings: [i32; 4],
    styles: InnerStyles,

    width_request: i32,
    height_request: i32,

    detecting_width: i32,
    detecting_height: i32,

    /// Control whether to occupy parent widget's space.
    ///
    /// This field only affects a container parent when a fixed child widget is overlaid..
    #[derivative(Default(value = "true"))]
    occupy_space: bool,

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

    resize_redraw: bool,
    manage_by_container: bool,

    callbacks: Callbacks,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EventBubble: u8 {
        const MOUSE_PRESSED = 1;
        const MOUSE_RELEASED = 1 << 1;
        const MOUSE_MOVE = 1 << 2;
        const MOUSE_WHEEL = 1 << 3;
        const MOUSE_OVER = 1 << 4;
        const MOUSE_OUT = 1 << 5;
        const KEY_PRESSED = 1 << 6;
        const KEY_RELEASED = 1 << 7;
    }
}

////////////////////////////////////// Widget Signals //////////////////////////////////////
pub trait WidgetSignals: ActionExt {
    signals! {
        WidgetSignals:

        /// Emit when widget's size changed.
        ///
        /// @param [`Size`]
        size_changed(Size);

        /// Emit when widget's geometry(size or position) changed.
        ///
        /// @param [`FRect`]
        geometry_changed(FRect);

        /// Emit when widget's receive mouse pressed event.
        ///
        /// @param [`MouseEvent`]
        mouse_pressed(&MouseEvent);

        /// Emit when widget's receive mouse released event.
        ///
        /// @param [`MouseEvent`]
        mouse_released(&MouseEvent);

        /// Emit when widget's receive mouse double click event.
        ///
        /// @param [`MouseEvent`]
        mouse_double_click(&MouseEvent);

        /// Emit when widget's receive mouse move event.
        ///
        /// @param [`MouseEvent`]
        mouse_move(&MouseEvent);

        /// Emit when widget's receive mouse wheel event.
        ///
        /// @param [`MouseEvent`]
        mouse_wheel(&MouseEvent);

        /// Emit when widget's receive mouse enter event.
        ///
        /// @see [`MouseEnterLeaveOverOutDesc`]
        ///
        /// @param [`MouseEvent`]
        mouse_enter(&MouseEvent);

        /// Emit when widget's receive mouse leave event.
        ///
        /// @see [`MouseEnterLeaveOverOutDesc`]
        ///
        /// @param [`MouseEvent`]
        mouse_leave(&MouseEvent);

        /// Emit when widget's receive mouse over event.
        ///
        /// @see [`MouseEnterLeaveOverOutDesc`]
        ///
        /// @param [`MouseEvent`]
        mouse_over(&MouseEvent);

        /// Emit when widget's receive mouse out event.
        ///
        /// @see [`MouseEnterLeaveOverOutDesc`]
        ///
        /// @param [`MouseEvent`]
        mouse_out(&MouseEvent);

        /// Emit when widget's receive key pressed event.
        ///
        /// @param [`KeyEvent`]
        key_pressed(&KeyEvent);

        /// Emit when widget's receive key released event.
        ///
        /// @param [`KeyEvent`]
        key_released(&KeyEvent);

        /// Emit when widget's receive character event.
        ///
        /// @param [`ReceiveCharacterEvent`]
        receive_character(&ReceiveCharacterEvent);

        /// Emit when widget's background changed.
        ///
        /// @param [`Color`]
        background_changed(Color);

        /// Emit when widget's visibility changed.
        ///
        /// @param [`bool`]
        visibility_changed(bool);

        /// Emit when widget's invalid area has changed.
        ///
        /// @param [`FRect`]
        invalid_area_changed(FRect);
    }
}

impl<T: WidgetImpl + ActionExt> WidgetSignals for T {}

impl WidgetSignals for dyn WidgetImpl {}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl Widget {
    #[inline]
    pub fn resize_batch(
        mut base: DynTr,
        widgets: &mut [&mut dyn WidgetImpl],
        width: Option<i32>,
        height: Option<i32>,
    ) {
        for w in widgets.iter_mut() {
            w.resize_ex(width, height, false);
        }
        ApplicationWindow::window().layout_change(base.bind_mut());
    }

    #[inline]
    pub fn _child_internal<T>(&mut self, mut child: Tr<T>)
    where
        T: WidgetImpl,
    {
        if child.get_parent_ref().is_none() {
            panic!("Do not call `child_internal()` directly, use `child()` instead.")
        }

        self.child = Some(child.clone().into());

        ApplicationWindow::initialize_dynamic_component(child.as_dyn_mut(), self.is_in_tree());
    }

    #[inline]
    pub fn _remove_child_internal(&mut self) {
        if let Some(child) = self.child.take() {
            let window = ApplicationWindow::window();
            window._add_removed_widget(child);
            window.layout_change(self);
        }
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
                if !child.visibility_check() {
                    return;
                }
                if let Some(iv) = cast!(child as IsolatedVisibility) {
                    if iv.auto_hide() {
                        return;
                    }
                }

                child.set_property("visible", true.to_value());
                child.set_render_styles(true);
            } else {
                child.set_property("visible", false.to_value());
            }
        }
    }

    /// Notify the child to change the zindex.
    #[inline]
    fn notify_zindex(&mut self, new_zindex: u64) {
        if let Some(child) = self.get_child_mut() {
            child.set_z_index(new_zindex);
        }
    }

    /// Notify the child to rerender styles.
    #[inline]
    fn notify_rerender_styles(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.set_render_styles(true)
        }
    }

    #[inline]
    fn notify_minimized(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.set_minimized(true)
        }
    }

    #[inline]
    fn notify_propagate_update_rect(&mut self, rect: CoordRect) {
        if let Some(child) = self.get_child_mut() {
            child.propagate_update_rect(rect);
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

#[inline]
pub fn index_children(mut widget: &mut dyn WidgetImpl) {
    loop {
        let ptr = widget.as_ptr_mut();

        let id = widget.id();
        let ids = widget.children_index();

        let parent = ptr_mut!(ptr).get_parent_mut();

        if let Some(p) = parent {
            p.children_index_mut().insert(id);
            p.children_index_mut().extend(ids);
            widget = p;
        } else {
            break;
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

        self.set_property("visible", true.to_value());
    }

    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value);

        match name {
            "invalidate" => {
                let (_, propagate) = value.get::<(bool, bool)>();
                if propagate {
                    // Notify all the child widget to invalidate, preparing rerenderer after.
                    self.notify_invalidate();
                }
            }
            "visible" => {
                let visible = value.get::<bool>();
                emit!(self, visibility_changed(visible));
                self.inner_visibility_changed(visible);
                self.on_visibility_changed(visible);
                self.notify_visible(visible)
            }
            "z_index" => {
                if !ApplicationWindow::window_of(self.window_id()).initialized() {
                    return;
                }
                let new_z_index = value.get::<u64>();
                self.notify_zindex(new_z_index);
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
            "propagate_update_rect" => {
                let rect = value.get::<CoordRect>();
                self.notify_propagate_update_rect(rect);
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
impl<T: WidgetImpl + WidgetExt + WidgetInnerExt + ShadowRender> ElementImpl for T {
    fn on_renderer(&mut self, cr: &DrawingContext) {
        #[cfg(verbose_logging)]
        let frame = cr.frame();

        if cast!(self as WinWidget).is_some() {
            return;
        }

        if !self.visible() && !self.is_animation_progressing() {
            #[cfg(verbose_logging)]
            #[rustfmt::skip]
            info!(
                "[on_renderer({}:{})] {} check return, visible={}, is_animation_progressing={}", frame.id(), frame.nth(), self.name(), self.visible(), self.is_animation_progressing()
            );
            if self.object_type().is_a(ApplicationWindow::static_type()) {
                let mut background = self.background();
                background.set_transparency(self.transparency());
                cr.canvas().clear(background);
            }
            return;
        }

        let mut geometry = self.rect();
        if !geometry.is_valid() {
            #[cfg(verbose_logging)]
            #[rustfmt::skip]
            info!(
                "[on_renderer({}:{})] {} check return, geometry is not valid, rect={:?}", frame.id(), frame.nth(), self.name(), geometry,
            );
            return;
        }
        geometry.set_point(&(0, 0).into());

        let _track = Tracker::start(format!("single_render_{}", self.name()));

        let window = ApplicationWindow::window();

        let name = &self.name();
        let mut painter = Painter::new(name, cr.canvas(), self);

        // Shared widget porcessing:
        if let Some(shared_widget) = cast_mut!(self as SharedWidgetImpl) {
            shared_widget.pixels_render(&mut painter);
            return;
        }

        // The default paint blend mode is set to `Src`,
        // it should be set to `SrcOver` when the widget is undergoing animation progress.
        painter.set_blend_mode(self.blend_mode());
        if self.is_animation_progressing() || self.background().a() != 255 {
            painter.set_blend_mode(BlendMode::SrcOver);
        }

        // Clip difference the children region:
        painter.save();

        if self.id() == self.window_id() && self.border_ref().should_draw_radius() {
            let rect = self.rect_f();
            painter.clip_round_rect_global(
                rect,
                self.border_ref().border_radius,
                ClipOp::Difference,
            );
            painter.fill_rect_global(rect, Color::TRANSPARENT);
            painter.restore();
        } else {
            window.clip_window(&mut painter);
        }

        if self.id() != self.window_id() {
            self.clip_child_region(&mut painter);
        }
        if let Some(parent) = self.get_parent_ref() {
            painter.clip_rect_global(parent.contents_rect(None), ClipOp::Intersect);
        }
        for &id in window.get_radius_widgets() {
            if self.descendant_of(id) {
                if let Some(w) = window.find_id(id) {
                    painter.clip_round_rect_global(
                        w.rect(),
                        w.border_ref().border_radius,
                        ClipOp::Intersect,
                    );
                }
            }
        }

        let window_resized = window.is_resize_redraw();
        if (self.whole_styles_render() || self.is_resize_redraw() || window_resized)
            && !self.is_animation_progressing()
        {
            self.widget_props_mut().redraw_region.clear();
        }

        #[cfg(verbose_logging)]
        #[rustfmt::skip]
        info!(
            "[on_renderer({}:{})] {}, z_index={}, redraw_region={:?}", frame.id(), frame.nth(), self.name(), self.z_index(), self.redraw_region()
        );

        handle_global_overlaid(self, &mut painter, window_resized);

        let cliped = painter_clip(self, &mut painter, self.redraw_region().iter());

        if self.redraw_shadow_box() {
            self.render_shadow(&mut painter);
            self.set_redraw_shadow_box(false);
            painter.restore();
            return;
        } else if !self.first_rendered() || self.render_styles() {
            let _track = Tracker::start(format!("single_render_{}_styles", self.name()));
            let mut background = if !self.is_animation_progressing() {
                self.opaque_background()
            } else {
                self.background()
            };
            if background != Color::TRANSPARENT {
                background.set_transparency(self.transparency());
            }
            if let Some(pc) = cast!(self as PartCovered) {
                if pc.is_covered() {
                    background = self.background();
                }
            }

            // Draw the background color of the Widget.
            if self.is_render_difference()
                && self.first_rendered()
                && !window.minimized()
                && !cliped
            {
                painter.save();
                self.clip_rect(&mut painter, ClipOp::Intersect);

                let mut border_rect: FRect = self.rect_record();
                border_rect.set_point(&(0, 0).into());
                self.border_ref()
                    .clear_border(&mut painter, border_rect, background);

                self.render_difference(&mut painter, background);
            } else {
                painter.save();
                if self.border_ref().should_draw_radius() {
                    painter.clip_round_rect_global(
                        self.visual_rect(),
                        self.border_ref().border_radius,
                        ClipOp::Intersect,
                    );
                }
                self.render_shadow(&mut painter);
                painter.restore();

                painter.save();
                self.clip_rect(&mut painter, ClipOp::Intersect);
                painter.fill_rect(geometry, background);
            }

            // Draw the border of the Widget.
            self.border_ref().render(&mut painter, geometry.into());

            self.set_first_rendered(true);
            self.set_render_styles(false);

            painter.restore();
        }

        painter.reset();
        painter.set_font(self.font().clone());

        if self.is_strict_clip_widget() {
            self.clip_rect(&mut painter, ClipOp::Intersect);
        }

        let should_paint = match cast_mut!(self as Loadable) {
            Some(loading) if loading.is_loading() => {
                loading.render_loading(&mut painter);
                false
            }
            _ => true,
        };

        if should_paint {
            let invalid_area = self.invalid_area();
            if invalid_area.is_valid() {
                self.clear_global(&mut painter, invalid_area);
            }

            let _track = Tracker::start(format!("single_render_{}_paint", self.name()));
            self.widget_props_mut().redraw_region.merge_all();
            self.paint(&mut painter);
        }

        painter.restore();
    }

    #[inline]
    fn after_renderer(&mut self) {
        self.set_rect_record(self.rect_f());
        self.set_image_rect_record(self.visual_image_rect());
        self.clear_regions();
        self.set_whole_styles_render(false);
        if self.window().id() != self.id() {
            self.set_resize_redraw(false);
        }
    }

    #[inline]
    fn when_update(&mut self) {
        self.clear_regions();
    }
}

#[inline]
pub(crate) fn painter_clip(
    widget: &dyn WidgetImpl,
    painter: &mut Painter,
    iter: Iter<CoordRect>,
) -> bool {
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
        painter.clip_region_global(region, ClipOp::Intersect);
    }
    op
}

fn handle_global_overlaid(
    widget: &mut dyn WidgetImpl,
    painter: &mut Painter,
    window_resized: bool,
) {
    for (&id, overlaid) in widget.window().overlaids_mut().iter_mut() {
        let overlaid = nonnull_mut!(overlaid);

        // Clip the overliad widget area.
        if widget.z_index() < overlaid.z_index() && !widget.descendant_of(id) && widget.id() != id {
            if cast!(overlaid as WinWidget).is_none() {
                painter.clip_rect_global(overlaid.rect(), ClipOp::Difference);
            }
        } else {
            continue;
        }

        if window_resized
            || !overlaid.first_rendered()
            || overlaid.render_styles()
            || !widget.render_styles()
            || !widget.rect_f().is_intersects(&overlaid.visual_rect())
            || widget.is_resize_redraw()
        {
            continue;
        }

        // Handle overlaid box shadow overflow.
        if !widget.redraw_region().is_empty() {
            let mut intersects = false;
            let vr = overlaid.visual_rect();
            for r in widget.redraw_region().iter() {
                if r.rect().is_intersects(&vr) {
                    intersects = true;
                    overlaid.update_rect(*r);
                }
            }
            if !intersects {
                continue;
            }
        } else {
            overlaid.update_rect(CoordRect::new(widget.rect(), Coordinate::World));
        }
        overlaid.set_redraw_shadow_box(true);
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
                if parent.as_ref()?.object_type().is_a(R::static_type()) {
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
                if child.as_ref()?.object_type().is_a(R::static_type()) {
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
                if parent.as_mut()?.object_type().is_a(R::static_type()) {
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
                if child.as_ref()?.object_type().is_a(R::static_type()) {
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
        if !self.visible() {
            return false;
        }
        if !self.rect().contains(point) {
            return false;
        }
        if self.invalid_area().contains_point(point) {
            return false;
        }

        for (&id, overlaid) in self.window().overlaids().iter() {
            let overlaid = nonnull_ref!(overlaid);
            if self.descendant_of(id) || self.id() == id || self.z_index() > overlaid.z_index() {
                continue;
            }
            if overlaid.rect().contains(point) {
                return false;
            }
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

////////////////////////////////////// ChildRegionAcquire //////////////////////////////////////
pub trait ChildRegionAcquire {
    fn child_region(&self) -> tlib::skia_safe::Region;
}

impl ChildRegionAcquire for Widget {
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

////////////////////////////////////// ChildRegionClip //////////////////////////////////////
pub(crate) trait ChildRegionClip {
    fn clip_child_region(&self, painter: &mut Painter);
}

impl<T: WidgetImpl> ChildRegionClip for T {
    #[inline]
    fn clip_child_region(&self, painter: &mut Painter) {
        let widget = self;
        if let Some(container) = cast!(widget as ContainerImpl) {
            for c in container.children() {
                if !c.visible() || !c.background().is_opaque() {
                    continue;
                }
                c.clip_rect(painter, ClipOp::Difference)
            }
        } else if let Some(c) = widget.get_child_ref() {
            if !c.visible() || !c.background().is_opaque() {
                return;
            }
            c.clip_rect(painter, ClipOp::Difference)
        }
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
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    fn inner_mouse_enter(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse leave event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    fn inner_mouse_leave(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse over event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    fn inner_mouse_over(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse out event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    fn inner_mouse_out(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive key pressed event.
    fn inner_key_pressed(&mut self, event: &KeyEvent);

    /// Invoke when widget's receive key released event.
    fn inner_key_released(&mut self, event: &KeyEvent);

    /// Invoke when widget's receive character event.
    fn inner_receive_character(&mut self, event: &ReceiveCharacterEvent);

    /// Invoke when widget's visibility has changed.
    fn inner_visibility_changed(&mut self, visible: bool);
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

        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().mouse_pressed {
            f(ptr_mut!(ptr), event)
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_pressed(event)
        }

        emit!(Widget::inner_mouse_pressed => self, mouse_pressed(event));

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_PRESSED) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = *event;
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

        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().mouse_released {
            f(ptr_mut!(ptr), event)
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_released(event)
        }

        emit!(Widget::inner_mouse_released => self, mouse_released(event));

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_RELEASED) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = *event;
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_released(&evt);
            parent.inner_mouse_released(&evt, true);
        }
    }

    #[inline]
    fn inner_mouse_move(&mut self, event: &MouseEvent) {
        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().mouse_move {
            f(ptr_mut!(ptr), event)
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_move(event)
        }

        emit!(Widget::inner_mouse_move => self, mouse_move(event));

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_MOVE) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = *event;
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_move(&evt);
            parent.inner_mouse_move(&evt);
        }
    }

    #[inline]
    fn inner_mouse_wheel(&mut self, event: &MouseEvent) {
        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().mouse_wheel {
            f(ptr_mut!(ptr), event)
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_wheel(event)
        }

        emit!(Widget::inner_mouse_wheel => self, mouse_wheel(event));

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);

        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_WHEEL) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = *event;
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_wheel(&evt);
            parent.inner_mouse_wheel(&evt);
        }
    }

    #[inline]
    fn inner_mouse_enter(&mut self, event: &MouseEvent) {
        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().mouse_enter {
            f(ptr_mut!(ptr))
        }
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_enter(event)
        }

        emit!(Widget::inner_mouse_enter => self, mouse_enter(event));
    }

    #[inline]
    fn inner_mouse_leave(&mut self, event: &MouseEvent) {
        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().mouse_leave {
            f(ptr_mut!(ptr))
        }
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_leave(event)
        }

        emit!(Widget::inner_mouse_leave => self, mouse_leave(event));
    }

    #[inline]
    fn inner_mouse_over(&mut self, event: &MouseEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_over(event)
        }

        emit!(Widget::inner_mouse_enter => self, mouse_over(event));

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);
        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_OVER) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = *event;
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_over(&evt);
            parent.inner_mouse_over(&evt);
        }
    }

    #[inline]
    fn inner_mouse_out(&mut self, event: &MouseEvent) {
        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_mouse_out(event)
        }

        emit!(Widget::inner_mouse_enter => self, mouse_out(event));

        let mut pos: Point = event.position().into();
        pos = self.map_to_global(&pos);
        if let Some(parent) = self.get_parent_mut() {
            if !parent.is_event_bubbled(EventBubble::MOUSE_OUT) {
                return;
            }

            pos = parent.map_to_widget(&pos);
            let mut evt = *event;
            evt.set_position((pos.x(), pos.y()));

            parent.on_mouse_out(&evt);
            parent.inner_mouse_out(&evt);
        }
    }

    #[inline]
    fn inner_key_pressed(&mut self, event: &KeyEvent) {
        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().key_pressed {
            f(ptr_mut!(ptr), event)
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_key_pressed(event)
        }

        emit!(Widget::inner_key_pressed => self, key_pressed(event));

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
        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().key_released {
            f(ptr_mut!(ptr), event)
        }

        if let Some(inner_customize_process) = cast_mut!(self as InnerCustomizeEventProcess) {
            inner_customize_process.inner_customize_key_released(event)
        }

        emit!(Widget::inner_key_released => self, key_released(event));

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

        emit!(Widget::inner_receive_character => self, receive_character(event));
    }

    #[inline]
    fn inner_visibility_changed(&mut self, visible: bool) {
        let ptr = self as *mut dyn WidgetImpl;
        if let Some(ref f) = self.callbacks().visibility_changed {
            f(ptr_mut!(ptr), visible);
        }
    }
}

#[reflect_trait]
#[allow(unused_variables)]
pub trait InnerCustomizeEventProcess {
    /// Invoke when widget's receive mouse pressed event.
    #[inline]
    fn inner_customize_mouse_pressed(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse released event.
    #[inline]
    fn inner_customize_mouse_released(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse move event.
    #[inline]
    fn inner_customize_mouse_move(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse wheel event.
    #[inline]
    fn inner_customize_mouse_wheel(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse enter event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn inner_customize_mouse_enter(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse leave event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn inner_customize_mouse_leave(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse over event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn inner_customize_mouse_over(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse out event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn inner_customize_mouse_out(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive key pressed event.
    #[inline]
    fn inner_customize_key_pressed(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive key released event.
    #[inline]
    fn inner_customize_key_released(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive character event.
    #[inline]
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
    + WidgetPropsAcquire
    + ElementImpl
    + ElementExt
    + ObjectOperation
    + ObjectType
    + ObjectImpl
    + SuperType
    + Layout
    + InnerRunAfter
    + InnerEventProcess
    + PointEffective
    + ChildRegionAcquire
    + ActionExt
    + WindowAcquire
{
    /// The widget can be focused or not, default value was false.
    ///
    /// The ability of widget to receive focus determines
    /// whether it can receive keyboard-related events.
    #[inline]
    fn enable_focus(&self) -> bool {
        false
    }

    #[inline]
    fn blend_mode(&self) -> BlendMode {
        BlendMode::default()
    }

    #[inline]
    fn notify_update(&mut self) {
        if !self.initialized() {
            return;
        }

        self.update()
    }

    /// Invoke this function when renderering.
    #[inline]
    fn paint(&mut self, painter: &mut Painter) {}

    /// Invoke when widget's font was changed.
    #[inline]
    fn font_changed(&mut self) {}

    /// `run_after()` will be invoked when application was started. <br>
    ///
    /// ### Should annotated macro `[run_after]` to enable this function.
    ///
    /// ### Should call `self.parent_run_after()` mannually if override this function.
    #[inline]
    fn run_after(&mut self) {}

    /// Invoke when widget's receive mouse pressed event.
    #[inline]
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse released event.
    #[inline]
    fn on_mouse_released(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse move event.
    ///
    /// The widget does not track mouse movement by default. If need, call function [`set_mouse_tracking`](WidgetExt::set_mouse_tracking)
    #[inline]
    fn on_mouse_move(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse wheel event.
    #[inline]
    fn on_mouse_wheel(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse enter event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn on_mouse_enter(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse leave event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn on_mouse_leave(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse over event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn on_mouse_over(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse out event.
    ///
    /// @see [`MouseEnterLeaveOverOutDesc`]
    #[inline]
    fn on_mouse_out(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive key pressed event.
    #[inline]
    fn on_key_pressed(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive key released event.
    #[inline]
    fn on_key_released(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive character event.
    #[inline]
    fn on_receive_character(&mut self, event: &ReceiveCharacterEvent) {}

    /// Invoke when widget's receive input method event.
    #[inline]
    fn on_input_method(&mut self, input_method: &InputMethodEvent) {}

    /// Invoke when widget getting focus.
    #[inline]
    fn on_get_focus(&mut self) {}

    /// Invoke when widget losing focus.
    #[inline]
    fn on_lose_focus(&mut self) {}

    /// Invoke when window minimized.
    #[inline]
    fn on_window_minimized(&mut self) {}

    /// Invoke when window maximized.
    #[inline]
    fn on_window_maximized(&mut self) {}

    /// Invoke when window restored.
    #[inline]
    fn on_window_restored(&mut self) {}

    /// Invoke when widget's visibility changed.
    #[inline]
    fn on_visibility_changed(&mut self, visible: bool) {}

    /// Check the visibility of widget.
    /// false: prevent the `show()` calling of widget.
    #[inline]
    fn visibility_check(&self) -> bool {
        true
    }
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
}

impl AsMutPtr for dyn WidgetImpl {}

pub trait ChildOp: WidgetImpl {
    /// @see [`Widget::_child_internal`](Widget::_child_internal) <br>
    /// Go to[`Function defination`](ChildOp::child) (Defined in [`ChildOp`])
    fn child<T: WidgetImpl>(&mut self, child: Tr<T>);

    /// @see [`Widget::_remove_child_internal`](Widget::_remove_child_internal) <br>
    /// Go to[`Function defination`](ChildOp::child) (Defined in [`ChildOp`])
    /// Remove current child.
    fn remove_child(&mut self);
}
impl ChildOp for Widget {
    #[inline]
    fn child<_T: WidgetImpl>(&mut self, mut child: Tr<_T>) {
        if self.super_type().is_a(Container::static_type()) {
            panic!("function `child()` was invalid in `Container`, use `add_child()` instead")
        }
        child.set_parent(self);
        self._child_internal(child)
    }

    #[inline]
    fn remove_child(&mut self) {
        if self.super_type().is_a(Container::static_type()) {
            panic!("function `remove_child()` was invalid in `Container`, use `remove_children()` instead")
        }
        self._remove_child_internal()
    }
}

////////////////////////////////////// Widget Layouts impl //////////////////////////////////////
impl<T: WidgetAcquire> Layout for T {
    #[inline]
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    #[inline]
    fn position_layout(&mut self, parent: Option<&dyn WidgetImpl>) {
        LayoutMgr::base_widget_position_layout(self, parent);
    }
}

impl Layout for Widget {
    #[inline]
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    #[inline]
    fn position_layout(&mut self, parent: Option<&dyn WidgetImpl>) {
        LayoutMgr::base_widget_position_layout(self, parent)
    }
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

////////////////////////////////////// IterExecutor //////////////////////////////////////
#[reflect_trait]
pub trait IterExecutor: WidgetImpl {
    /// This function will be executed in each iteration of the UI main thread loop.
    fn iter_execute(&mut self);
}

pub type IterExecutorHnd = Option<NonNull<dyn IterExecutor>>;

////////////////////////////////////// WidgetHndAsable //////////////////////////////////////
pub(crate) trait WidgetHndAsable: WidgetImpl + Sized {
    #[inline]
    fn as_hnd(&mut self) -> WidgetHnd {
        NonNull::new(self)
    }
}

impl<T: WidgetImpl + Sized> WidgetHndAsable for T {}

////////////////////////////////////// WidgetPropsAcquire //////////////////////////////////////
pub trait WidgetPropsAcquire {
    /// Get the ref of widget props.
    fn widget_props(&self) -> &Widget;

    /// Get the mutable ref of widget props.
    fn widget_props_mut(&mut self) -> &mut Widget;
}

impl WidgetPropsAcquire for Widget {
    #[inline]
    fn widget_props(&self) -> &Widget {
        self
    }

    #[inline]
    fn widget_props_mut(&mut self) -> &mut Widget {
        self
    }
}

////////////////////////////////////// WidgetFinder //////////////////////////////////////
pub trait WidgetFinder: WidgetImpl {
    #[inline]
    fn finds<T: WidgetImpl + StaticType>(&self) -> Vec<&T> {
        self.window().finds::<T>()
    }

    #[inline]
    fn finds_mut<T: WidgetImpl + StaticType>(&self) -> Vec<&mut T> {
        self.window().finds_mut::<T>()
    }

    #[inline]
    fn find_id<T: WidgetImpl + StaticType>(&self, id: ObjectId) -> Option<&T> {
        self.window()
            .find_id(id)
            .and_then(|w| w.downcast_ref::<T>())
    }

    #[inline]
    fn find_id_mut<T: WidgetImpl + StaticType>(&self, id: ObjectId) -> Option<&mut T> {
        self.window()
            .find_id_mut(id)
            .and_then(|w| w.downcast_mut::<T>())
    }

    #[inline]
    fn find_name<T: WidgetImpl + StaticType>(&self, name: &str) -> Option<&T> {
        self.window()
            .find_name(name)
            .and_then(|w| w.downcast_ref::<T>())
    }

    #[inline]
    fn find_name_mut<T: WidgetImpl + StaticType>(&self, name: &str) -> Option<&mut T> {
        self.window()
            .find_name_mut(name)
            .and_then(|w| w.downcast_mut::<T>())
    }

    /// Only affected when the parent is a container.
    #[inline]
    fn find_siblings<T: WidgetImpl + StaticType>(&self) -> Vec<&T> {
        let mut siblings = vec![];
        if let Some(parent) = self.get_parent_ref() {
            if parent.super_type().is_a(Container::static_type()) {
                let container = cast!(parent as ContainerImpl).unwrap();
                for c in container.children() {
                    if c.object_type().is_a(T::static_type()) {
                        siblings.push(c.downcast_ref::<T>().unwrap())
                    }
                }
            }
        }
        siblings
    }

    /// Only affected when the parent is a container.
    #[inline]
    fn find_siblings_mut<T: WidgetImpl + StaticType>(&mut self) -> Vec<&mut T> {
        let mut siblings = vec![];
        if let Some(parent) = self.get_parent_mut() {
            if parent.super_type().is_a(Container::static_type()) {
                let container = cast_mut!(parent as ContainerImpl).unwrap();
                for c in container.children_mut() {
                    if c.object_type().is_a(T::static_type()) {
                        siblings.push(c.downcast_mut::<T>().unwrap())
                    }
                }
            }
        }
        siblings
    }
}

impl<T: WidgetImpl> WidgetFinder for T {}

impl WidgetFinder for dyn WidgetImpl {}

////////////////////////////////////// RegionClear //////////////////////////////////////
pub trait RegionClear: WidgetImpl {
    /// The coordinate of given rect should be [`Coordinate::Widget`]
    #[inline]
    fn clear<T: Into<SkiaRect>>(&self, painter: &mut Painter, rect: T) {
        painter.fill_rect(rect, self.opaque_background());
    }

    /// The coordinate of given rect should be [`Coordinate::Global`]
    #[inline]
    fn clear_global<T: Into<SkiaRect>>(&self, painter: &mut Painter, rect: T) {
        painter.fill_rect_global(rect, self.opaque_background())
    }
}

impl<T: WidgetImpl> RegionClear for T {}

impl RegionClear for dyn WidgetImpl {}

////////////////////////////////////// IsolatedVisibility //////////////////////////////////////
#[reflect_trait]
pub trait IsolatedVisibility: WidgetImpl {
    fn auto_hide(&self) -> bool;

    fn set_auto_hide(&mut self, auto_hide: bool);

    fn shadow_rect(&self) -> FRect;

    fn set_shadow_rect(&mut self, rect: FRect);

    fn shadow_rect_mut(&mut self) -> &mut FRect;
}

////////////////////////////////////// InnerRunAfter //////////////////////////////////////
pub trait InnerRunAfter {
    fn inner_run_after(&mut self);
}
impl InnerRunAfter for Widget {
    fn inner_run_after(&mut self) {}
}

/// `MouseEnter`/`MouseLeave`:
/// - `MouseEnter` fires when the mouse pointer enters the bounds of an element,
///   and does not bubble up from child elements. It is triggered less frequently,
///   ideal for certain UI interactions where you only need to know if the mouse
///   has entered or left the boundary of an element, regardless of its children.
///
/// - `MouseLeave` fires when the mouse pointer leaves the bounds of an element,
///   but, importantly, it does not fire when the mouse moves into child elements
///   of the parent element. This makes it suitable for handling UI logic where
///   you want an event to trigger only once when the mouse completely leaves the
///   element including all its children.
///
/// `MouseOver`/`MouseOut`:
/// - `MouseOver` occurs when the mouse pointer enters the element or any of its
///   children. This event can bubbles, meaning if the mouse moves over a child
///   element, the parent will also detect a `MouseOver` when
///   enable event propagation (see [`WidgetExt::enable_bubble`], [`EventBubble`]).
///
/// - `MouseOut` is similar in that it triggers both when the mouse leaves the
///   element or moves into any of its child elements. Like `MouseOver`, this
///   event also can bubbles, which can lead to it firing multiple times during
///   complex UI interactions involving multiple nested elements.
pub struct MouseEnterLeaveOverOutDesc;
