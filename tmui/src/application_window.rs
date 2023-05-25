use crate::skia_safe::Font;
use crate::{
    graphics::{board::Board, painter::Painter},
    layout::LayoutManager,
    platform::{window_context::OutputSender, Message},
    prelude::*,
    widget::{WidgetImpl, WidgetSignals},
};
use lazy_static::lazy_static;
use log::debug;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, VecDeque},
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Once,
    },
    thread::{self, ThreadId},
};
use tlib::figure::{Color, Size};
use tlib::{
    connect, emit,
    object::{ObjectImpl, ObjectSubclass},
};

static INIT: Once = Once::new();
lazy_static! {
    static ref BOARD: AtomicPtr<Board> = AtomicPtr::new(null_mut());
}

/// Store the [`Board`] as raw ptr.
pub(crate) fn store_board(board: &mut Board) {
    INIT.call_once(move || {
        BOARD.store(board, Ordering::SeqCst);
    })
}

#[extends(Widget)]
#[derive(Default)]
pub struct ApplicationWindow {
    output_sender: Option<OutputSender>,
    activated: bool,
}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";
}

impl ObjectImpl for ApplicationWindow {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_window_id(self.id());
        debug!(
            "`ApplicationWindow` construct: static_type: {}",
            Self::static_type().name()
        )
    }

    fn initialize(&mut self) {
        connect!(self, size_changed(), self, when_size_change(Size));
        child_initialize(self, self.get_raw_child_mut(), self.id());
        emit!(self.size_changed(), self.size());
    }
}

impl WidgetImpl for ApplicationWindow {
    fn paint(&mut self, mut _painter: Painter) {}
}

type ApplicationWindowContext = (
    ThreadId,
    Option<NonNull<ApplicationWindow>>,
    Box<LayoutManager>,
);

impl ApplicationWindow {
    pub fn new(width: i32, height: i32) -> Box<ApplicationWindow> {
        let thread_id = thread::current().id();
        let mut window: Box<ApplicationWindow> =
            Box::new(Object::new(&[("width", &width), ("height", &height)]));
        Self::windows().insert(
            window.id(),
            (
                thread_id,
                NonNull::new(window.as_mut()),
                Box::new(LayoutManager::default()),
            ),
        );
        window
    }

    #[inline]
    pub(crate) fn window_widgets() -> &'static mut HashMap<String, Option<NonNull<dyn WidgetImpl>>>
    {
        static mut WINDOW_WIDGETS: Lazy<HashMap<String, Option<NonNull<dyn WidgetImpl>>>> =
            Lazy::new(|| HashMap::new());
        unsafe { &mut WINDOW_WIDGETS }
    }

    /// SAFETY: `ApplicationWidnow` and `LayoutManager` can only get and execute in they own ui thread.
    #[inline]
    pub(crate) fn windows() -> &'static mut HashMap<u16, ApplicationWindowContext> {
        static mut WINDOWS: Lazy<HashMap<u16, ApplicationWindowContext>> =
            Lazy::new(|| HashMap::new());
        unsafe { &mut WINDOWS }
    }

    #[inline]
    pub(crate) fn layout_of<'a>(id: u16) -> &'a mut LayoutManager {
        let current_thread_id = thread::current().id();
        let (thread_id, _, layout) = Self::windows()
            .get_mut(&id)
            .expect(&format!("Unkonwn application window with id: {}", id));
        if current_thread_id != *thread_id {
            panic!("Execute `ApplicationWindow::layout_of()` in the wrong thrad.");
        }
        layout.as_mut()
    }

    #[inline]
    pub fn window_of<'a>(id: u16) -> &'a mut ApplicationWindow {
        let current_thread_id = thread::current().id();
        let (thread_id, window, _) = Self::windows()
            .get_mut(&id)
            .expect(&format!("Unkonwn application window with id: {}", id));
        if current_thread_id != *thread_id {
            panic!("Execute `ApplicationWindow::window_of()` in the wrong thrad.");
        }
        unsafe { window.unwrap().as_mut() }
    }

    #[inline]
    pub fn send_message_with_id(id: u16, message: Message) {
        Self::window_of(id).send_message(message)
    }

    #[inline]
    pub fn dispatch_event(&self) {}

    #[inline]
    pub fn send_message(&self, message: Message) {
        match self.output_sender {
            Some(OutputSender::Sender(ref sender)) => sender.send(message).unwrap(),
            Some(OutputSender::EventLoopProxy(ref sender)) => sender.send_event(message).unwrap(),
            None => panic!("`ApplicationWindow` did not register the output_sender."),
        }
    }

    #[inline]
    pub fn is_activate(&self) -> bool {
        self.activated
    }

    #[inline]
    pub fn window_layout_change(&mut self) {
        Self::layout_of(self.id()).layout_change(self)
    }

    #[inline]
    pub(crate) fn when_size_change(&mut self, size: Size) {
        Self::layout_of(self.id()).set_window_size(size);
    }

    #[inline]
    pub(crate) fn activate(&mut self) {
        self.activated = true;
    }

    #[inline]
    pub(crate) fn register_window(&mut self, sender: OutputSender) {
        self.output_sender = Some(sender)
    }
}

fn child_initialize(
    mut parent: *mut dyn WidgetImpl,
    mut child: Option<*mut dyn WidgetImpl>,
    window_id: u16,
) {
    let board = unsafe { BOARD.load(Ordering::SeqCst).as_mut().unwrap() };
    let type_registry = TypeRegistry::instance();
    let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();
    while let Some(child_ptr) = child {
        let child_ref = unsafe { child_ptr.as_mut().unwrap() };

        board.add_element(child_ref.as_element());
        ApplicationWindow::window_widgets().insert(child_ref.name(), NonNull::new(child_ptr));

        child_ref.inner_type_register(type_registry);
        child_ref.type_register(type_registry);
        child_ref.set_window_id(window_id);

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
            container_children.unwrap().iter_mut().for_each(|c| {
                c.set_parent(child_ptr);
                children.push_back(Some(*c))
            });
        } else {
            children.push_back(child_ref.get_raw_child_mut());
            if child_ref.get_raw_parent().is_none() {
                child_ref.set_parent(parent);
            }
            parent = child_ptr;
        }

        child_ref.initialize();

        child = children.pop_front().take().unwrap();
    }
}
