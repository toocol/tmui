use crate::{
    container::{Container, ContainerImpl, ReflectSizeUnifiedAdjust, ScaleMeasure},
    prelude::*,
    widget::{ScaleCalculate, WidgetImpl, WidgetSignals},
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
        manage_by_container: bool,
    );
}

pub trait ContainerLayout {
    fn static_composition() -> Composition;

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
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

pub(crate) trait SizeCalculation {
    /// Widget has child:
    ///
    /// Determine widget's size before calc child's size based on `expand`,`fixed`... <br>
    /// Mainly for processing expanded widgets. <br>
    /// Return the calculated size.
    fn pre_calc_size(&mut self, parent_size: Size) -> Size;

    /// Widget has child:
    fn calc_node_size(&mut self, child_size: Size);

    /// Widget has no child:
    fn calc_leaf_size(&mut self, window_size: Size, parent_size: Size);
}
impl SizeCalculation for dyn WidgetImpl {
    fn pre_calc_size(&mut self, parent_size: Size) -> Size {
        if self.id() == self.window_id() {
            return self.size();
        }
        let mut resized = false;

        if self.hexpand() && !self.fixed_width() {
            // Use `hscale` to determine widget's width:
            let parent = self.get_parent_ref().unwrap();
            let parent_hscale = if parent.super_type().is_a(Container::static_type()) {
                cast!(parent as ContainerImpl).unwrap().hscale_calculate()
            } else {
                parent.hscale_calculate()
            };

            if parent_hscale.is_adaption() {
                self.set_fixed_width(parent_size.width());
                resized = true;
            } else if !parent_hscale.is_dismiss() {
                let ration = self.hscale() / parent_hscale;
                self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                resized = true;
            }
        }

        if self.vexpand() && !self.fixed_height() {
            // Use `vscale` to determine widget's height:
            let parent = self.get_parent_ref().unwrap();
            let parent_vscale = if parent.super_type().is_a(Container::static_type()) {
                cast!(parent as ContainerImpl).unwrap().vscale_calculate()
            } else {
                parent.vscale_calculate()
            };

            if parent_vscale.is_adaption() {
                self.set_fixed_height(parent_size.height());
                resized = true;
            } else if !parent_vscale.is_dismiss() {
                let ration = self.vscale() / parent_vscale;
                self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                resized = true;
            }
        }

        if resized {
            debug!(
                "Widget {} resized in `pre_calc_size`, size: {:?}",
                self.name(),
                self.size()
            );
            emit!(SizeCalculation::pre_calc_size => self.size_changed(), self.size())
        }
        self.size()
    }

    fn calc_node_size(&mut self, child_size: Size) {
        if self.id() == self.window_id() {
            return;
        }
        let size = self.size();
        let mut resized = false;

        if size.width() == 0 {
            self.set_fixed_width(child_size.width());
            resized = true;
        }
        if size.height() == 0 {
            self.set_fixed_height(child_size.height());
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
        let mut resized = false;

        if self.fixed_width() {
            if self.hexpand() {
                self.set_fixed_width(
                    (parent_size.width() as f32 * self.fixed_width_ration()) as i32,
                );
                resized = true;
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
                    resized = true;
                } else if !parent_hscale.is_dismiss() {
                    let ration = self.hscale() / parent_hscale;
                    self.set_fixed_width((parent_size.width() as f32 * ration) as i32);
                    resized = true;
                }
            } else {
                if parent_size.width() != 0 {
                    if size.width() == 0 {
                        self.set_fixed_width(parent_size.width());
                        resized = true;
                    }
                } else {
                    if size.width() == 0 {
                        self.set_fixed_width(window_size.width());
                        resized = true;
                    }
                }
            }
        }

        if self.fixed_height() {
            if self.vexpand() {
                self.set_fixed_height(
                    (parent_size.height() as f32 * self.fixed_height_ration()) as i32,
                );
                resized = true;
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
                    resized = true;
                } else if !parent_vscale.is_dismiss() {
                    let ration = self.vscale() / parent_vscale;
                    self.set_fixed_height((parent_size.height() as f32 * ration) as i32);
                    resized = true;
                }
            } else {
                if parent_size.height() != 0 {
                    if size.height() == 0 {
                        self.set_fixed_height(parent_size.height());
                        resized = true;
                    }
                } else {
                    if size.height() == 0 {
                        self.set_fixed_height(window_size.height());
                        resized = true;
                    }
                }
            }
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

    pub(crate) fn layout_change(&self, widget: &mut dyn WidgetImpl) {
        // Deal with the size first
        Self::child_size_probe(self.window_size, widget.size(), widget);

        // Deal with the position
        Self::child_position_probe(None, None, Some(widget));
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

        widget.update();
        widget.set_rerender_styles(true);
        widget.child_image_rect_union_mut().clear();

        let raw_child = widget.get_raw_child();
        let widget_ptr = widget.as_ptr_mut();
        let composition = widget.composition();

        // Determine whether the widget is a container.
        let is_container = widget.super_type().is_a(Container::static_type());
        let container_ref = if is_container {
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
            let size = ptr_mut!(widget_ptr).pre_calc_size(parent_size);

            let child_size = if is_container {
                let mut child_size = Size::default();
                match composition {
                    Composition::Stack => {
                        children.unwrap().iter_mut().for_each(|child| {
                            child_size.max(Self::child_size_probe(window_size, size, *child));
                        });
                    }
                    Composition::HorizontalArrange => {
                        children.unwrap().iter_mut().for_each(|child| {
                            let inner = Self::child_size_probe(window_size, size, *child);
                            child_size.set_height(child_size.height().max(inner.height()));
                            child_size.add_width(inner.width());
                        });
                    }
                    Composition::VerticalArrange => {
                        children.unwrap().iter_mut().for_each(|child| {
                            let inner = Self::child_size_probe(window_size, size, *child);
                            child_size.set_width(child_size.width().max(inner.width()));
                            child_size.add_height(inner.height());
                        });
                    }
                    Composition::FixedContainer => {
                        let widget = ptr_mut!(widget_ptr);
                        if let Some(unified) = cast_mut!(widget as SizeUnifiedAdjust) {
                            unified.size_unified_adjust();
                        }
                        children.unwrap().iter_mut().for_each(|child| {
                            Self::child_size_probe(window_size, size, *child);
                        });
                        child_size = widget.size();
                    }
                    _ => unimplemented!(),
                }
                child_size
            } else {
                Self::child_size_probe(window_size, size, widget.get_child_mut().unwrap())
            };

            widget.calc_node_size(child_size);
            widget.image_rect().size()
        }
    }

    pub(crate) fn child_position_probe(
        mut previous: Option<*const dyn WidgetImpl>,
        mut parent: Option<*const dyn WidgetImpl>,
        mut widget: Option<*mut dyn WidgetImpl>,
    ) {
        let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();
        while let Some(widget_ptr) = widget {
            let widget_ref = unsafe { widget_ptr.as_mut().unwrap() };
            debug!("Widget position probe: {}", widget_ref.name());
            let previous_ref = unsafe { previous.as_ref().and_then(|p| p.as_ref()) };
            let parent_ref = unsafe { parent.as_ref().and_then(|p| p.as_ref()) };

            // Deal with the widget's postion.
            widget_ref.position_layout(previous_ref, parent_ref, false);
            if widget_ref.need_update_geometry() {
                widget_ref.update_geometry()
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
                children.push_back(widget_ref.get_raw_child_mut());
            }
            widget = children.pop_front().take().map_or(None, |widget| widget);
            previous = Some(widget_ptr);
            parent = if let Some(c) = widget.as_ref() {
                unsafe { c.as_ref().unwrap().get_raw_parent() }
            } else {
                None
            };
        }
    }

    pub(crate) fn base_widget_position_layout(
        widget: &mut dyn WidgetImpl,
        _: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        if parent.is_none() {
            return;
        }
        let parent = parent.unwrap();
        if parent.super_type().is_a(Container::static_type()) && !manage_by_container {
            return;
        }
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
