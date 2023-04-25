use crate::{container::Container, graphics::figure::Size, prelude::*, widget::WidgetImpl};
use log::debug;
use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Composition {
    #[default]
    Default,
    Overlay,
    VerticalArrange,
    HorizontalArrange,
}

pub trait Layout {
    fn composition(&self) -> Composition;

    fn position_layout(&mut self, previous: &dyn WidgetImpl, parent: &dyn WidgetImpl);
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
        // Deal with the width first
        let child = widget.get_raw_child_mut();
        if let Some(child) = child {
            self.child_width_probe(self.window_size, widget.size(), child);
        }

        // Deal with the position
        self.child_position_probe(Some(widget), Some(widget), child)
    }

    pub(crate) fn child_width_probe(
        &self,
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
            if parent_size.width() != 0 && parent_size.height() != 0 {
                if size.width() == 0 {
                    widget_ref.width_request(parent_size.width());
                }
                if size.height() == 0 {
                    widget_ref.height_request(parent_size.height());
                }
            } else {
                if size.width() == 0 {
                    widget_ref.width_request(window_size.width());
                }
                if size.height() == 0 {
                    widget_ref.height_request(window_size.height());
                }
            }
            let image_rect = widget_ref.image_rect();
            return Size::new(image_rect.width(), image_rect.height());
        } else {
            let child_size = if is_container {
                let mut child_size = Size::default();
                match composition {
                    Composition::Overlay => {
                        children.unwrap().iter_mut().for_each(|child| {
                            child_size.max(self.child_width_probe(window_size, child_size, *child));
                        });
                    }
                    Composition::HorizontalArrange => {

                    }
                    Composition::VerticalArrange => {

                    }
                    _ => {
                        children.unwrap().iter_mut().for_each(|child| {
                            child_size =
                                child_size + self.child_width_probe(window_size, child_size, *child)
                        });
                    }
                }
                child_size
            } else {
                self.child_width_probe(window_size, size, widget_ref.get_raw_child_mut().unwrap())
            };
            if size.width() == 0 {
                widget_ref.width_request(child_size.width());
            }
            if size.height() == 0 {
                widget_ref.height_request(child_size.height());
            }
            return widget_ref.size();
        }
    }

    pub(crate) fn child_position_probe(
        &self,
        mut previous: Option<*const dyn WidgetImpl>,
        mut parent: Option<*const dyn WidgetImpl>,
        mut widget: Option<*mut dyn WidgetImpl>,
    ) {
        let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();
        while let Some(widget_ptr) = widget {
            let widget_ref = unsafe { widget_ptr.as_mut().unwrap() };
            let previous_ref = unsafe { previous.as_ref().unwrap().as_ref().unwrap() };
            let parent_ref = unsafe { parent.as_ref().unwrap().as_ref().unwrap() };

            // Deal with the widget's postion.
            widget_ref.position_layout(previous_ref, parent_ref);

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
            widget = children.pop_front().take().unwrap();
            previous = Some(widget_ptr);
            parent = if let Some(c) = widget.as_ref() {
                unsafe { c.as_ref().unwrap().get_raw_parent() }
            } else {
                None
            };
        }
    }
}
