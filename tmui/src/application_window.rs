use crate::{
    graphics::{
        board::Board,
        figure::{Color, Size},
        painter::Painter,
    },
    layout::LayoutManager,
    platform::Message,
    prelude::*,
    widget::{WidgetImpl, WidgetSignals},
};
use lazy_static::lazy_static;
use log::debug;
use once_cell::sync::Lazy;
use skia_safe::Font;
use std::{
    collections::{HashMap, VecDeque},
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        mpsc::{SendError, Sender},
        Once,
    },
};
use tlib::{
    connect, emit,
    object::{ObjectImpl, ObjectSubclass},
};

static INIT: Once = Once::new();
lazy_static! {
    static ref BOARD: AtomicPtr<Board> = AtomicPtr::new(null_mut());
}

/// Store the [`Board`] as raw ptr.
pub(crate) fn store_board(
    board: &mut Board
) {
    INIT.call_once(move || {
        BOARD.store(board, Ordering::SeqCst);
    })
}

#[extends(Widget)]
#[derive(Default)]
pub struct ApplicationWindow {
    output_sender: Option<Sender<Message>>,
    activated: bool,
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
        connect!(self, size_changed(), self, when_size_change(Size));
        child_initialize(self, self.get_raw_child_mut());
        emit!(self.size_changed(), self.size());
    }
}

impl WidgetImpl for ApplicationWindow {
    fn paint(&mut self, mut _painter: Painter) {}
}

impl ApplicationWindow {
    pub fn new(width: i32, height: i32) -> ApplicationWindow {
        let window: ApplicationWindow = Object::new(&[("width", &width), ("height", &height)]);
        Self::windows_layouts().insert(window.id(), Box::new(LayoutManager::default()));
        window
    }

    pub(crate) fn windows_layouts() -> &'static mut HashMap<u16, Box<LayoutManager>> {
        static mut WINDOW_LAYOUTS: Lazy<HashMap<u16, Box<LayoutManager>>> = Lazy::new(|| {
            let m: HashMap<u16, Box<LayoutManager>> = HashMap::new();
            m
        });
        unsafe { &mut WINDOW_LAYOUTS }
    }

    pub fn send_message(&self, message: Message) -> Result<(), SendError<Message>> {
        self.output_sender
            .as_ref()
            .expect("`ApplicationWindow` did not register the output sender.")
            .send(message)
    }

    pub fn is_activate(&self) -> bool {
        self.activated
    }

    pub(crate) fn when_size_change(&mut self, size: Size) {
        Self::windows_layouts()
            .get_mut(&self.id())
            .unwrap()
            .set_window_size(size);
    }

    pub(crate) fn activate(&mut self) {
        self.activated = true;
    }

    pub(crate) fn register_window(&mut self, sender: Sender<Message>) {
        self.output_sender = Some(sender)
    }

    pub(crate) fn window_layout_change(&mut self) {
        Self::windows_layouts()
            .get_mut(&self.id())
            .unwrap()
            .layout_change(self)
    }
}

fn child_initialize(mut parent: *mut dyn WidgetImpl, mut child: Option<*mut dyn WidgetImpl>) {
    let board = unsafe { BOARD.load(Ordering::SeqCst).as_mut().unwrap() };
    let type_registry = TypeRegistry::instance();
    let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();
    while let Some(child_ptr) = child {
        let child_ref = unsafe { child_ptr.as_mut().unwrap() };
        child_ref.set_parent(parent);
        parent = child_ptr;

        board.add_element(child_ref.as_element());

        child_ref.initialize();
        child_ref.inner_type_register(type_registry);
        child_ref.type_register(type_registry);

        // Determine whether the widget is a container.
        let is_container = child_ref.parent_type().is_a(Container::static_type());
        let container_ref = if is_container {
            cast_mut!(child_ref as ContainerImpl)
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
            children.push_back(child_ref.get_raw_child_mut());
        }

        child = children.pop_front().take().unwrap();
    }
}
