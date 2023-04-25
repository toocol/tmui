use crate::{
    graphics::{
        board::Board,
        figure::{Color, Size},
        painter::Painter,
    },
    platform::Message,
    prelude::*,
    widget::WidgetImpl,
};
use lazy_static::lazy_static;
use log::debug;
use skia_safe::Font;
use std::{
    collections::VecDeque,
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        mpsc::{Sender, SendError},
        Once,
    },
};
use tlib::object::{ObjectImpl, ObjectSubclass};

static INIT: Once = Once::new();
lazy_static! {
    static ref BOARD: AtomicPtr<Board> = AtomicPtr::new(null_mut());
}

/// Store the [`Board`] as raw ptr.
pub fn store_board(board: &mut Board) {
    INIT.call_once(move || {
        BOARD.store(board as *mut Board, Ordering::SeqCst);
    })
}

#[extends(Widget)]
#[derive(Default)]
pub struct ApplicationWindow {
    output_sender: Option<Sender<Message>>,
}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";

    type Type = ApplicationWindow;

    type ParentType = Object;
}

impl ObjectImpl for ApplicationWindow {
    fn construct(&mut self) {
        self.parent_construct();
        debug!(
            "`ApplicationWindow` construct: static_type: {}",
            Self::static_type().name()
        )
    }

    fn initialize(&mut self) {
        child_initialize(self, self.get_raw_child_mut())
    }
}

impl WidgetImpl for ApplicationWindow {
    fn paint(&mut self, mut _painter: Painter) {}
}

impl ApplicationWindow {
    pub fn new(width: i32, height: i32) -> ApplicationWindow {
        Object::new(&[("width", &width), ("height", &height)])
    }

    pub fn register_window(&mut self, sender: Sender<Message>) {
        self.output_sender = Some(sender)
    }

    pub fn send_message(&self, message: Message) -> Result<(), SendError<Message>> {
        self.output_sender
            .as_ref()
            .expect("`ApplicationWindow` did not register the output sender.")
            .send(message)
    }

    pub fn size_probe(&mut self) {
        let child = self.get_raw_child_mut();
        if let Some(child) = child {
            child_width_probe(self.size(), self.size(), child);
        }
    }

    pub fn position_probe(&mut self) {
        let child = self.get_raw_child_mut();
        child_position_probe(Some(self), Some(self), child)
    }
}

fn child_initialize(mut parent: *mut dyn WidgetImpl, mut child: Option<*mut dyn WidgetImpl>) {
    let board = unsafe { BOARD.load(Ordering::SeqCst).as_mut().unwrap() };
    let type_registry = TypeRegistry::instance();
    while let Some(child_ptr) = child {
        let child_ref = unsafe { child_ptr.as_mut().unwrap() };
        child_ref.set_parent(parent);
        parent = child_ptr;

        board.add_element(child_ref.as_element());

        child_ref.initialize();
        child_ref.inner_type_register(type_registry);
        child_ref.type_register(type_registry);
        child = child_ref.get_raw_child_mut();
    }
}

fn child_width_probe(window_size: Size, parent_size: Size, widget: *mut dyn WidgetImpl) -> Size {
    let widget_ref = unsafe { widget.as_mut().unwrap() };
    let raw_child = widget_ref.get_raw_child();
    let size = widget_ref.size();

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
            let mut size = Size::default();
            children
                .unwrap()
                .iter_mut()
                .for_each(|child| size = size + child_width_probe(window_size, size, *child));
            size
        } else {
            child_width_probe(window_size, size, widget_ref.get_raw_child_mut().unwrap())
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

fn child_position_probe(
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
