use crate::{
    container::{Container, ContainerImpl, ReflectSizeUnifiedAdjust, ScaleMeasure},
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

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
    );
}

pub trait ContainerLayout {
    fn static_composition<T: WidgetImpl + ContainerImpl>(widget: &T) -> Composition;

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: Option<&dyn WidgetImpl>,
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
            let mut size = container.size();
            for c in container.children() {
                let cs = c.size();
                if c.fixed_width() {
                    size.set_width(size.width() - cs.width());
                }
                if c.fixed_height() {
                    size.set_height(size.height() - cs.height());
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
            self.size()
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
    fn parent_size_exclude_spacing(&self) -> Option<Size> {
        let parent = self.get_parent_ref();
        if let Some(p) = parent {
            cast!(p as SpacingCapable).and_then(|s| Some(s.size_exclude_spacing()))
        } else {
            None
        }
    }
}
impl SizeCalculation for dyn WidgetImpl {
    fn pre_calc_size(&mut self, window_size: Size, parent_size: Size) -> (Size, Size) {
        if self.id() == self.window_id() || cast!(self as Overlaid).is_some() {
            return (self.size(), self.remain_size());
        }
        let size = self.size();
        let mut resized = false;
        let parent_size_exclude_spacing = self.parent_size_exclude_spacing();

        if self.fixed_width() {
            if self.hexpand() {
                let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                if self.fixed_width_ration() == 0. {
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

                if parent_hscale.is_adaption() {
                    self.set_fixed_width(parent_size.width());
                } else if !parent_hscale.is_dismiss() {
                    let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                    let ration = self.hscale() / parent_hscale;
                    self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_width() != 0 {
                    self.set_fixed_width(self.detecting_width())
                }
            }
        }

        if self.fixed_height() {
            if self.vexpand() {
                let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                if self.fixed_height_ration() == 0. {
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

                if parent_vscale.is_adaption() {
                    self.set_fixed_height(parent_size.height());
                } else if !parent_vscale.is_dismiss() {
                    let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                    let ration = self.vscale() / parent_vscale;
                    self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_height() != 0 {
                    self.set_fixed_height(self.detecting_height())
                }
            }
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
            emit!(SizeCalculation::pre_calc_size => self.size_changed(), self.size())
        }

        (self.size(), self.remain_size())
    }

    fn calc_node_size(&mut self, child_size: Size) {
        if self.id() == self.window_id() {
            return;
        }
        let size = self.size();
        let mut resized = false;

        if size.width() == 0 && child_size.width() != 0 {
            self.set_fixed_width(child_size.width());
        }
        if size.height() == 0 && child_size.height() != 0 {
            self.set_fixed_height(child_size.height());
        }

        self.check_size_hint();

        if child_size.width() > self.size().width() || child_size.height() > self.size().height() {
            if let Some(unified) = cast_mut!(self as SizeUnifiedAdjust) {
                unified.size_unified_adjust();
            }
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
            emit!(SizeCalculation::calc_node_size => self.size_changed(), self.size())
        }
    }

    fn calc_leaf_size(&mut self, window_size: Size, parent_size: Size) {
        if self.id() == self.window_id() {
            return;
        }
        let size = self.size();
        let parent_size_exclude_spacing = self.parent_size_exclude_spacing();
        let mut resized = false;

        if self.fixed_width() {
            if self.hexpand() {
                let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                if self.fixed_width_ration() == 0. {
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

                if parent_hscale.is_adaption() {
                    self.set_fixed_width(parent_size.width());
                } else if !parent_hscale.is_dismiss() {
                    let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                    let ration = self.hscale() / parent_hscale;
                    self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_width() != 0 {
                    self.set_fixed_width(self.detecting_width())
                }
            }
        }

        if self.fixed_height() {
            if self.vexpand() {
                let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                if self.fixed_height_ration() == 0. {
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

                if parent_vscale.is_adaption() {
                    self.set_fixed_height(parent_size.height());
                } else if !parent_vscale.is_dismiss() {
                    let parent_size = parent_size_exclude_spacing.or(Some(parent_size)).unwrap();

                    let ration = self.vscale() / parent_vscale;
                    self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                }
            } else {
                if self.detecting_height() != 0 {
                    self.set_fixed_height(self.detecting_height())
                }
            }
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
            emit!(SizeCalculation::calc_leaf_size => self.size_changed(), self.size())
        }
    }

    fn check_size_hint(&mut self) {
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
pub(crate) struct LayoutManager {
    window_size: Size,
}

impl LayoutManager {
    pub(crate) fn set_window_size(&mut self, new_size: Size) {
        debug!("`LayoutManager` set window size: {:?}", new_size);
        self.window_size = new_size;
    }

    pub(crate) fn layout_change(&self, widget: &mut dyn WidgetImpl, is_animation: bool) {
        let _track = Tracker::start(format!("layout_change_{}", widget.name()));

        if !is_animation {
            // Deal with the size first
            Self::child_size_probe(self.window_size, widget.size(), widget);
        }

        // Deal with the position
        Self::child_position_probe(None, widget.get_raw_parent_mut(), Some(widget));
    }

    pub(crate) fn child_size_probe(
        window_size: Size,
        parent_size: Size,
        widget: &mut dyn WidgetImpl,
    ) -> Size {
        debug!(
            "Widget {} size probe, parent_size: {:?}",
            widget.name(),
            parent_size
        );

        if widget.repaint_when_resize() {
            if widget.first_rendered() {
                widget.set_resize_redraw(true)
            }
            widget.update();
            widget.set_rerender_styles(true);
        }
        widget.child_image_rect_union_mut().clear();
        widget.child_overflow_rect_mut().clear();

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
        let children = if container_ref.is_some() {
            Some(container_ref.unwrap().children_mut())
        } else {
            None
        };

        let container_no_children = children.is_none() || children.as_ref().unwrap().len() == 0;
        if raw_child.is_none() && container_no_children {
            widget.calc_leaf_size(window_size, parent_size);

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
                                if idx != 0 && spacing != 0 {
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
                                if idx != 0 && spacing != 0 {
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
            widget.image_rect().size()
        }
    }

    pub(crate) fn child_position_probe(
        mut previous: Option<*const dyn WidgetImpl>,
        mut parent: Option<*mut dyn WidgetImpl>,
        mut widget: Option<*mut dyn WidgetImpl>,
    ) {
        let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();
        while let Some(widget_ptr) = widget {
            let widget_ref = unsafe { widget_ptr.as_mut().unwrap() };
            let previous_ref = unsafe { previous.as_ref().and_then(|p| p.as_ref()) };
            let parent_ref = unsafe { parent.as_ref().and_then(|p| p.as_ref()) };

            // Deal with the widget's postion.
            widget_ref.position_layout(previous_ref, parent_ref);
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
                ptr_mut!(p).child_overflow_rect_mut().or(&widget_ref.rect());
            }

            // Emit `geometry_changed()` when widget's position or size has changed.
            let new_rect = widget_ref.rect();
            if widget_ref.rect_record() != new_rect {
                emit!(widget_ref.geometry_changed(), new_rect)
            }

            // Determine whether the widget is a container.
            let is_container = widget_ref.super_type().is_a(Container::static_type());
            let container_ref = if is_container {
                cast_mut!(widget_ref as ContainerImpl)
            } else {
                None
            };
            let container_children = if container_ref.is_some() {
                Some(container_ref.unwrap().children_mut())
            } else {
                None
            };

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
            widget = children.pop_front().map_or(None, |widget| widget);
            previous = Some(widget_ptr);
            parent = if let Some(c) = widget.as_ref() {
                unsafe { c.as_mut().unwrap().get_raw_parent_mut() }
            } else {
                None
            };
        }
    }

    pub(crate) fn base_widget_position_layout(
        widget: &mut dyn WidgetImpl,
        _: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
    ) {
        if parent.is_none() || cast!(widget as Overlaid).is_some() {
            return;
        }
        if widget.is_manage_by_container() {
            return;
        }

        let parent = parent.unwrap();
        let widget_rect = widget.rect();
        let parent_rect = parent.rect();

        let halign = widget.get_property("halign").unwrap().get::<Align>();
        let valign = widget.get_property("valign").unwrap().get::<Align>();

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
