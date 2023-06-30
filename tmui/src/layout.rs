use crate::{
    container::{Container, ContainerImpl},
    prelude::*,
    widget::{WidgetImpl, WidgetSignals},
};
use log::debug;
use std::collections::VecDeque;
use tlib::figure::Size;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Composition {
    #[default]
    Default,
    Overlay,
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
        Self::child_position_probe(None, None, Some(widget))
    }

    pub(crate) fn child_size_probe(
        window_size: Size,
        parent_size: Size,
        widget: *mut dyn WidgetImpl,
    ) -> Size {
        let widget_ref = unsafe { widget.as_mut().unwrap() };
        let raw_child = widget_ref.get_raw_child();
        let size = widget_ref.size();
        let composition = widget_ref.composition();

        // Determine whether the widget is a container.
        let is_container = widget_ref.parent_type().is_a(Container::static_type());
        let container_ref = if is_container {
            cast_mut!(widget_ref as ContainerImpl)
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
            let mut resized = false;
            if parent_size.width() != 0 && parent_size.height() != 0 {
                if size.width() == 0 {
                    widget_ref.width_request(parent_size.width());
                    resized = true;
                }

                if size.height() == 0 {
                    widget_ref.height_request(parent_size.height());
                    resized = true;
                }
            } else {
                if size.width() == 0 {
                    widget_ref.width_request(window_size.width());
                    resized = true;
                }
                if size.height() == 0 {
                    widget_ref.height_request(window_size.height());
                    resized = true;
                }
            }

            if resized {
                emit!(widget_ref.size_changed(), widget_ref.rect().size())
            }
            widget_ref.image_rect().size()
        } else {
            let child_size = if is_container {
                let mut child_size = Size::default();
                match composition {
                    Composition::Overlay => {
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
                        children.unwrap().iter_mut().for_each(|child| {
                            Self::child_size_probe(window_size, size, *child);
                        });
                        child_size = widget_ref.size();
                        if child_size.width() == 0 || child_size.height() == 0 {
                            panic!(
                                "`{}` FixedContainer should specified the size, the width or height can't be 0.",
                                widget_ref.type_name()
                            );
                        }
                    }
                    _ => unimplemented!(),
                }
                child_size
            } else {
                Self::child_size_probe(window_size, size, widget_ref.get_raw_child_mut().unwrap())
            };
            let mut resized = false;
            if size.width() == 0 {
                widget_ref.width_request(child_size.width());
                resized = true;
            }
            if size.height() == 0 {
                widget_ref.height_request(child_size.height());
                resized = true;
            }

            if resized {
                emit!(widget_ref.size_changed(), widget_ref.rect().size())
            }
            widget_ref.image_rect().size()
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

            // Determine whether the widget is a container.
            let is_container = widget_ref.parent_type().is_a(Container::static_type());
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
        if parent.parent_type().is_a(Container::static_type()) && !manage_by_container {
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
