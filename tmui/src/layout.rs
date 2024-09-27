use crate::{
    container::{Container, ContainerImpl, ReflectSizeUnifiedAdjust, ScaleMeasure, SpacingSize},
    opti::tracker::Tracker,
    prelude::*,
    widget::{widget_inner::WidgetInnerExt, ScaleCalculate, WidgetImpl, WidgetSignals},
};
use log::debug;
use std::collections::VecDeque;
use tlib::{figure::Size, ptr_mut};

pub const ZINDEX_STEP: u32 = 1000;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Composition {
    #[default]
    Default,
    Overlay,
    Stack,
    VerticalArrange,
    HorizontalArrange,
    FixedContainer,
}

pub trait Layout {
    fn composition(&self) -> Composition;

    fn position_layout(&mut self, parent: Option<&dyn WidgetImpl>);
}

pub trait ContainerLayout {
    fn static_composition<T: WidgetImpl + ContainerImpl>(widget: &T) -> Composition;

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        parent: Option<&dyn WidgetImpl>,
    );
}

#[reflect_trait]
pub trait ContentAlignment {
    /// Container will mmanage the contents' position if the homogeneous was `true`.
    /// Otherwise the contents' position will layouted by their own alignment.
    fn homogeneous(&self) -> bool;

    fn set_homogeneous(&mut self, homogeneous: bool);

    fn content_halign(&self) -> Align;

    fn content_valign(&self) -> Align;

    fn set_content_halign(&mut self, halign: Align);

    fn set_content_valign(&mut self, valign: Align);
}

trait RemainSize {
    fn remain_size(&self) -> Size;
}
impl RemainSize for dyn WidgetImpl {
    fn remain_size(&self) -> Size {
        if let Some(container) = cast!(self as ContainerImpl) {
            let mut size = container.borderless_size();
            let composition = container.composition();
            for c in container.children() {
                let (mut cw, mut ch) = c.size().into();

                if composition == Composition::HorizontalArrange
                    && c.fixed_width()
                    && c.is_occupy_space()
                    && c.visible()
                {
                    if cw == 0 {
                        cw = c.get_width_request();
                    }
                    size.set_width(size.width() - cw);
                }

                if composition == Composition::VerticalArrange
                    && c.fixed_height()
                    && c.is_occupy_space()
                    && c.visible()
                {
                    if ch == 0 {
                        ch = c.get_height_request();
                    }
                    size.set_height(size.height() - ch);
                }
            }

            if size.width() < 0 {
                size.set_width(0);
            }
            if size.height() < 0 {
                size.set_height(0);
            }

            size
        } else {
            self.borderless_size()
        }
    }
}

pub(crate) trait SizeCalculation: WidgetImpl {
    /// Widget has child:
    ///
    /// Determine widget's size before calc child's size based on `expand`,`fixed`... <br>
    /// Mainly for processing expanded widgets. <br>
    ///
    /// @return: (actual_size, remain_size)
    fn pre_calc_size(&mut self, window_size: Size, parent_size: Size) -> (Size, Size);

    /// Widget has child:
    fn calc_node_size(&mut self, child_size: Size);

    /// Widget has no child:
    fn calc_leaf_size(&mut self, window_size: Size, parent_size: Size);

    /// Checking `size_hint` of widget to adjust size.
    fn check_size_hint(&mut self);

    #[inline]
    fn parent_spacing_size(&self) -> Option<SpacingSize> {
        let parent = self.get_parent_ref();
        if let Some(p) = parent {
            cast!(p as SpacingCapable).and_then(|s| Some(s.spacing_size()))
        } else {
            None
        }
    }
}
#[allow(clippy::collapsible_else_if)]
impl SizeCalculation for dyn WidgetImpl {
    fn pre_calc_size(&mut self, window_size: Size, mut parent_size: Size) -> (Size, Size) {
        if self.id() == self.window_id() || cast!(self as Overlaid).is_some() {
            return (self.borderless_size(), self.remain_size());
        }
        let size = self.size();
        let mut resized = false;
        let parent_spacing_size = self.parent_spacing_size();

        if self.fixed_width() {
            if self.hexpand() {
                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }

                if self.fixed_width_ration() <= 0. {
                    let ration = self.get_width_request() as f32 / parent_size.width() as f32;
                    self.set_fixed_width_ration(ration);
                }

                self.set_fixed_width(
                    (parent_size.width() as f32 * self.fixed_width_ration()) as i32,
                );
            } else {
                if self.get_width_request() != 0 {
                    self.set_fixed_width(self.get_width_request())
                }
            }
        } else {
            if self.hexpand() {
                // Use `hscale` to determine widget's width:
                let parent = self.get_parent_ref().unwrap();
                let parent_hscale = if parent.super_type().is_a(Container::static_type()) {
                    cast!(parent as ContainerImpl).unwrap().hscale_calculate()
                } else {
                    parent.hscale_calculate()
                };

                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }
                if parent_hscale.is_adaption() {
                    let ration = self.hscale().min(1.) / 1.;
                    self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                } else if !parent_hscale.is_dismiss() {
                    let ration = self.hscale() / parent_hscale;
                    self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_width() != 0 {
                    self.set_fixed_width(self.detecting_width())
                }
            }
        }
        let width = self.rect_f().width();
        if let Some(iv) = cast_mut!(self as IsolatedVisibility) {
            iv.shadow_rect_mut().set_width(width);
        }
        if !self.visible() {
            self.set_fixed_width(0)
        }

        if self.fixed_height() {
            if self.vexpand() {
                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }

                if self.fixed_height_ration() <= 0. {
                    let ration = self.get_height_request() as f32 / parent_size.height() as f32;
                    self.set_fixed_height_ration(ration);
                }

                self.set_fixed_height(
                    (parent_size.height() as f32 * self.fixed_height_ration()) as i32,
                );
            } else {
                if self.get_height_request() != 0 {
                    self.set_fixed_height(self.get_height_request())
                }
            }
        } else {
            if self.vexpand() {
                // Use `vscale` to determine widget's height:
                let parent = self.get_parent_ref().unwrap();
                let parent_vscale = if parent.super_type().is_a(Container::static_type()) {
                    cast!(parent as ContainerImpl).unwrap().vscale_calculate()
                } else {
                    parent.vscale_calculate()
                };

                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }
                if parent_vscale.is_adaption() {
                    let ration = self.vscale().min(1.) / 1.;
                    self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                } else if !parent_vscale.is_dismiss() {
                    let ration = self.vscale() / parent_vscale;
                    self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_height() != 0 {
                    self.set_fixed_height(self.detecting_height())
                }
            }
        }
        let height = self.rect_f().height();
        if let Some(iv) = cast_mut!(self as IsolatedVisibility) {
            iv.shadow_rect_mut().set_height(height);
        }
        if !self.visible() {
            self.set_fixed_height(0)
        }

        self.check_size_hint();

        if window_size.width() == 0 && self.size().width() != 0 {
            self.set_fixed_width(0);
        }
        if window_size.height() == 0 && self.size().height() != 0 {
            self.set_fixed_height(0);
        }

        if self.size() != size {
            resized = true;
        }

        if resized {
            debug!(
                "Widget {} resized in `pre_calc_size`, size: {:?}",
                self.name(),
                self.size()
            );
        }

        (self.borderless_size(), self.remain_size())
    }

    fn calc_node_size(&mut self, child_size: Size) {
        if self.id() == self.window_id() {
            return;
        }
        let size = self.size();
        let visible = self.visible();
        let mut resized = false;

        if (size.width() == 0 && child_size.width() != 0 && visible)
            || (!self.hexpand() && !self.fixed_width())
        {
            self.set_fixed_width(child_size.width());
        }
        if (size.height() == 0 && child_size.height() != 0 && visible)
            || (!self.vexpand() && !self.fixed_height())
        {
            self.set_fixed_height(child_size.height());
        }

        self.check_size_hint();

        let borderless_size = self.borderless_size();
        if visible
            && (child_size.width() > borderless_size.width()
                || child_size.height() > borderless_size.height())
        {
            if let Some(unified) = cast_mut!(self as SizeUnifiedAdjust) {
                unified.size_unified_adjust();
            }

            self.handle_child_overflow_hidden(child_size);
        }

        if self.size() != size {
            resized = true;
        }

        if resized {
            debug!(
                "Widget {} resized in `calc_node_size`, size: {:?}",
                self.name(),
                self.size()
            );
        }
    }

    fn calc_leaf_size(&mut self, window_size: Size, mut parent_size: Size) {
        if self.id() == self.window_id() {
            return;
        }
        let size = self.size();
        let parent_spacing_size = self.parent_spacing_size();
        let mut resized = false;

        if self.fixed_width() {
            if self.hexpand() {
                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }

                if self.fixed_width_ration() <= 0. {
                    let ration = self.get_width_request() as f32 / parent_size.width() as f32;
                    self.set_fixed_width_ration(ration);
                }

                self.set_fixed_width(
                    (parent_size.width() as f32 * self.fixed_width_ration()) as i32,
                );
            } else {
                self.set_fixed_width(self.get_width_request());
            }
        } else {
            if self.hexpand() {
                // Use `hscale` to determine widget's width:
                let parent = self.get_parent_ref().unwrap();
                let parent_hscale = if parent.super_type().is_a(Container::static_type()) {
                    cast!(parent as ContainerImpl).unwrap().hscale_calculate()
                } else {
                    parent.hscale_calculate()
                };

                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }
                if parent_hscale.is_adaption() {
                    let ration = self.hscale().min(1.) / 1.;
                    self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                } else if !parent_hscale.is_dismiss() {
                    let ration = self.hscale() / parent_hscale;
                    self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_width() != 0 {
                    self.set_fixed_width(self.detecting_width())
                }
            }
        }
        let width = self.rect_f().width();
        if let Some(iv) = cast_mut!(self as IsolatedVisibility) {
            iv.shadow_rect_mut().set_width(width);
        }
        if !self.visible() {
            self.set_fixed_width(0)
        }

        if self.fixed_height() {
            if self.vexpand() {
                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }

                if self.fixed_height_ration() <= 0. {
                    let ration = self.get_height_request() as f32 / parent_size.height() as f32;
                    self.set_fixed_height_ration(ration);
                }

                self.set_fixed_height(
                    (parent_size.height() as f32 * self.fixed_height_ration()) as i32,
                );
            } else {
                self.set_fixed_height(self.get_height_request())
            }
        } else {
            if self.vexpand() {
                // Use `vscale` to determine widget's height:
                let parent = self.get_parent_ref().unwrap();
                let parent_vscale = if parent.super_type().is_a(Container::static_type()) {
                    cast!(parent as ContainerImpl).unwrap().vscale_calculate()
                } else {
                    parent.vscale_calculate()
                };

                if let Some(ref spacing_size) = parent_spacing_size {
                    spacing_size.remove_spacing_from(&mut parent_size)
                }
                if parent_vscale.is_adaption() {
                    let ration = self.vscale().min(1.) / 1.;
                    self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                } else if !parent_vscale.is_dismiss() {
                    let ration = self.vscale() / parent_vscale;
                    self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_height() != 0 {
                    self.set_fixed_height(self.detecting_height())
                }
            }
        }
        let height = self.rect_f().height();
        if let Some(iv) = cast_mut!(self as IsolatedVisibility) {
            iv.shadow_rect_mut().set_height(height);
        }
        if !self.visible() {
            self.set_fixed_height(0)
        }

        self.check_size_hint();

        if window_size.width() == 0 && size.width() != 0 {
            self.set_fixed_width(0);
        }
        if window_size.height() == 0 && size.height() != 0 {
            self.set_fixed_height(0);
        }

        if self.size() != size {
            resized = true;
        }

        if resized {
            debug!(
                "Widget {} resized in `calc_leaf_size`, size: {:?}",
                self.name(),
                self.size()
            );
        }
    }

    fn check_size_hint(&mut self) {
        if !self.visible() {
            return;
        }
        let size_hint = self.size_hint();

        let size = self.size();
        if let Some(min_width) = size_hint.min_width() {
            if size.width() < min_width {
                self.set_fixed_width(min_width)
            }
        }
        if let Some(min_height) = size_hint.min_height() {
            if size.height() < min_height {
                self.set_fixed_height(min_height)
            }
        }

        let size = self.size();
        if let Some(max_width) = size_hint.max_width() {
            if size.width() > max_width {
                self.set_fixed_width(max_width)
            }
        }
        if let Some(max_height) = size_hint.max_height() {
            if size.height() > max_height {
                self.set_fixed_height(max_height)
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct LayoutMgr {
    window_size: Size,
}

impl LayoutMgr {
    pub(crate) fn set_window_size(&mut self, new_size: Size) {
        debug!("`LayoutManager` set window size: {:?}", new_size);
        self.window_size = new_size;
    }

    pub(crate) fn layout_change(&self, widget: &mut dyn WidgetImpl, is_animation: bool) {
        let _track = Tracker::start(format!("layout_change_{}", widget.name()));

        // If the widget was not under animation-progressing, deal with the size first:
        if !is_animation {
            let parent_size = widget
                .get_parent_ref()
                .map(|p| p.remain_size())
                .unwrap_or(self.window_size);

            Self::child_size_probe(self.window_size, parent_size, widget);
        }

        // Deal with the position
        Self::child_position_probe(widget.get_raw_parent_mut(), Some(widget));
    }

    pub(crate) fn child_size_probe(
        window_size: Size,
        parent_size: Size,
        widget: &mut dyn WidgetImpl,
    ) -> Size {
        debug!(
            "Widget {} size probe, parent_size: {:?}, visible: {}",
            widget.name(),
            parent_size,
            widget.visible(),
        );

        if widget.repaint_when_resize() {
            widget.update();
            widget.set_render_styles(true);
        }
        widget.child_image_rect_union_mut().clear();
        widget.child_overflow_rect_mut().clear();

        let size = widget.size();
        let raw_child = widget.get_raw_child();
        let widget_ptr = widget.as_ptr_mut();
        let composition = widget.composition();

        let mut spacing = 0;

        // Determine whether the widget is a container.
        let is_container = widget.super_type().is_a(Container::static_type());
        let container_ref = if is_container {
            if let Some(spacing_widget) = cast!(widget as SpacingCapable) {
                spacing = spacing_widget.get_spacing() as i32;
            };

            cast_mut!(widget as ContainerImpl)
        } else {
            None
        };
        let children = container_ref.map(|c| c.children_mut());

        let container_no_children = children.is_none() || children.as_ref().unwrap().is_empty();
        if raw_child.is_none() && container_no_children {
            widget.calc_leaf_size(window_size, parent_size);

            if widget.size() != size {
                emit!(LayoutManager::child_size_probe => widget.size_changed(), widget.size())
            }
            widget.image_rect().size()
        } else {
            let (actual_size, remain_size) =
                ptr_mut!(widget_ptr).pre_calc_size(window_size, parent_size);

            let child_size = if is_container {
                let mut child_size = Size::default();
                match composition {
                    Composition::Stack => {
                        children.unwrap().iter_mut().for_each(|child| {
                            child_size.max(Self::child_size_probe(
                                window_size,
                                remain_size,
                                *child,
                            ));
                        });
                    }
                    Composition::HorizontalArrange => {
                        children
                            .unwrap()
                            .iter_mut()
                            .enumerate()
                            .for_each(|(idx, child)| {
                                let parent_size = if child.hexpand() && child.fixed_width() {
                                    actual_size
                                } else {
                                    remain_size
                                };

                                let inner =
                                    Self::child_size_probe(window_size, parent_size, *child);

                                child_size.set_height(child_size.height().max(inner.height()));
                                child_size.add_width(inner.width());
                                if idx != 0 && spacing != 0 && inner.width() != 0 {
                                    child_size.add_width(spacing);
                                }
                            });
                    }
                    Composition::VerticalArrange => {
                        children
                            .unwrap()
                            .iter_mut()
                            .enumerate()
                            .for_each(|(idx, child)| {
                                let parent_size = if child.vexpand() && child.fixed_height() {
                                    actual_size
                                } else {
                                    remain_size
                                };

                                let inner =
                                    Self::child_size_probe(window_size, parent_size, *child);

                                child_size.set_width(child_size.width().max(inner.width()));
                                child_size.add_height(inner.height());
                                if idx != 0 && spacing != 0 && inner.height() != 0 {
                                    child_size.add_height(spacing);
                                }
                            });
                    }
                    Composition::FixedContainer => {
                        let widget = ptr_mut!(widget_ptr);
                        if let Some(unified) = cast_mut!(widget as SizeUnifiedAdjust) {
                            unified.size_unified_adjust();
                        }
                        children.unwrap().iter_mut().for_each(|child| {
                            Self::child_size_probe(window_size, remain_size, *child);
                        });
                        child_size = widget.size();
                    }
                    _ => unimplemented!(),
                }
                child_size
            } else {
                Self::child_size_probe(window_size, remain_size, widget.get_child_mut().unwrap())
            };

            widget.calc_node_size(child_size);

            if widget.size() != size {
                if widget.repaint_when_resize() && widget.first_rendered() {
                    widget.set_resize_redraw(true)
                }

                emit!(LayoutManager::child_size_probe => widget.size_changed(), widget.size())
            }
            widget.image_rect().size()
        }
    }

    pub(crate) fn child_position_probe(
        mut parent: Option<*mut dyn WidgetImpl>,
        mut widget: Option<*mut dyn WidgetImpl>,
    ) {
        let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();
        while let Some(widget_ptr) = widget {
            let widget_ref = unsafe { widget_ptr.as_mut().unwrap() };
            let parent_ref = unsafe { parent.as_ref().and_then(|p| p.as_ref()) };

            // Deal with the widget's postion.
            widget_ref.position_layout(parent_ref);

            let r = widget_ref.rect_f();
            if let Some(iv) = cast_mut!(widget_ref as IsolatedVisibility) {
                if !iv.is_animation_progressing() {
                    let shadow_rect = iv.shadow_rect_mut();
                    shadow_rect.set_x(r.x());
                    shadow_rect.set_y(r.y());
                }
            }
            if r != widget_ref.rect_record() {
                widget_ref.set_resize_redraw(true);
            }

            debug!(
                "Widget position probe: {}, position: {:?}, is_manage_by_container: {}",
                widget_ref.name(),
                widget_ref.rect(),
                widget_ref.is_manage_by_container()
            );

            if widget_ref.need_update_geometry() {
                widget_ref.update_geometry()
            }
            if let Some(p) = parent {
                ptr_mut!(p)
                    .child_overflow_rect_mut()
                    .or(&widget_ref.rect_f());
            }

            // Emit `geometry_changed()` when widget's position or size has changed.
            let new_rect = widget_ref.rect_f();
            if widget_ref.rect_record() != new_rect {
                emit!(widget_ref.geometry_changed(), new_rect);
            }

            // Determine whether the widget is a container.
            let is_container = widget_ref.super_type().is_a(Container::static_type());
            let container_ref = if is_container {
                cast_mut!(widget_ref as ContainerImpl)
            } else {
                None
            };
            let container_children = container_ref.map(|c| c.children_mut());

            if is_container {
                container_children
                    .unwrap()
                    .iter_mut()
                    .for_each(|c| children.push_back(Some(*c)));
            } else {
                let crm = widget_ref.get_raw_child_mut();
                if crm.is_some() {
                    children.push_back(crm);
                }
            }
            if let Some(popupable) = cast_mut!(widget_ref as Popupable) {
                if let Some(popup) = popupable.get_popup_mut() {
                    children.push_back(Some(popup.as_widget_impl_mut() as *mut dyn WidgetImpl));
                }
            };

            widget = children.pop_front().and_then(|widget| widget);
            parent = if let Some(c) = widget.as_ref() {
                unsafe { c.as_mut().unwrap().get_raw_parent_mut() }
            } else {
                None
            };
        }
    }

    #[inline]
    pub(crate) fn base_widget_position_layout(
        widget: &mut dyn WidgetImpl,
        parent: Option<&dyn WidgetImpl>,
    ) {
        if parent.is_none() || cast!(widget as Overlaid).is_some() {
            if let Some(popup) = cast_mut!(widget as PopupImpl) {
                if popup.is_animation_progressing() {
                    return;
                }
                popup.layout_relative_position();
            }

            return;
        }
        if widget.is_manage_by_container() {
            return;
        }

        Self::base_widget_position_layout_inner(widget, parent)
    }

    pub(crate) fn base_widget_position_layout_inner(
        widget: &mut dyn WidgetImpl,
        parent: Option<&dyn WidgetImpl>,
    ) {
        let parent = parent.unwrap();
        let widget_rect = widget.rect();
        let parent_rect = parent.borderless_rect();

        let halign = widget.halign();
        let valign = widget.valign();

        match halign {
            Align::Start => widget.set_fixed_x(parent_rect.x() + widget.margin_left()),
            Align::Center => {
                let offset = (parent_rect.width() - widget_rect.width()) / 2 + widget.margin_left();
                widget.set_fixed_x(parent_rect.x() + offset)
            }
            Align::End => {
                let offset = parent_rect.width() - widget_rect.width() + widget.margin_left();
                widget.set_fixed_x(parent_rect.x() + offset)
            }
        }

        match valign {
            Align::Start => widget.set_fixed_y(parent_rect.y() + widget.margin_top()),
            Align::Center => {
                let offset =
                    (parent_rect.height() - widget_rect.height()) / 2 + widget.margin_top();
                widget.set_fixed_y(parent_rect.y() + offset)
            }
            Align::End => {
                let offset = parent_rect.height() - widget_rect.height() + widget.margin_top();
                widget.set_fixed_y(parent_rect.y() + offset)
            }
        }
    }
}
