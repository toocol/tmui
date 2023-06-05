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
use tlib::events::{to_key_event, to_mouse_event};
use tlib::{
    connect, emit,
    events::{Event, EventType},
    figure::{Color, Size},
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
        debug!(
            "`ApplicationWindow` construct: static_type: {}",
            Self::static_type().name()
        )
    }

    fn initialize(&mut self) {
        connect!(self, size_changed(), self, when_size_change(Size));
        self.set_window_id(self.id());
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
    HashMap<String, Option<NonNull<dyn WidgetImpl>>>,
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
                HashMap::new(),
            ),
        );
        window
    }

    /// SAFETY: `ApplicationWidnow` and `LayoutManager` can only get and execute in they own ui thread.
    #[inline]
    pub(crate) fn windows() -> &'static mut HashMap<u16, ApplicationWindowContext> {
        static mut WINDOWS: Lazy<HashMap<u16, ApplicationWindowContext>> =
            Lazy::new(|| HashMap::new());
        unsafe { &mut WINDOWS }
    }

    #[inline]
    pub(crate) fn widgets_of(
        id: u16,
    ) -> &'static mut HashMap<String, Option<NonNull<dyn WidgetImpl>>> {
        let current_thread_id = thread::current().id();
        let (thread_id, _, _, map) = Self::windows()
            .get_mut(&id)
            .expect(&format!("Unkonwn application window with id: {}", id));
        if current_thread_id != *thread_id {
            panic!("Execute `ApplicationWindow::layout_of()` in the wrong thrad.");
        }
        map
    }

    #[inline]
    pub(crate) fn layout_of(id: u16) -> &'static mut LayoutManager {
        let current_thread_id = thread::current().id();
        let (thread_id, _, layout, _) = Self::windows()
            .get_mut(&id)
            .expect(&format!("Unkonwn application window with id: {}", id));
        if current_thread_id != *thread_id {
            panic!("Execute `ApplicationWindow::layout_of()` in the wrong thrad.");
        }
        layout.as_mut()
    }

    #[inline]
    pub fn window_of(id: u16) -> &'static mut ApplicationWindow {
        let current_thread_id = thread::current().id();
        let (thread_id, window, _, _) = Self::windows()
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
    pub fn layout_change(&self, widget: &mut dyn WidgetImpl) {
        // If the given widget's layout has changed, need pass ref of widget's parent.
        let parent = unsafe {
            widget
                .get_raw_parent_mut()
                .as_mut()
                .unwrap()
                .as_mut()
                .unwrap()
        };
        Self::layout_of(self.id()).layout_change(parent)
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

    #[inline]
    pub(crate) fn dispatch_event(&self, evt: Event) {
        match evt.type_() {
            // Mouse pressed.
            EventType::MouseButtonPress => {
                let mut evt = to_mouse_event(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = unsafe { widget_opt.as_mut().unwrap().as_mut() };

                    if widget.point_effective(&pos) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_pressed(evt.as_ref());
                        widget.on_mouse_pressed(evt.as_ref());
                    }
                }
            }

            // Mouse released.
            EventType::MouseButtonRelease => {
                let mut evt = to_mouse_event(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = unsafe { widget_opt.as_mut().unwrap().as_mut() };

                    if widget.point_effective(&pos) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_released(evt.as_ref());
                        widget.on_mouse_released(evt.as_ref());
                    }
                }
            }

            // Mouse double clicked.
            EventType::MouseButtonDoubleClick => {
                let mut evt = to_mouse_event(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = unsafe { widget_opt.as_mut().unwrap().as_mut() };

                    if widget.point_effective(&pos) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_double_click(evt.as_ref());
                        widget.on_mouse_double_click(evt.as_ref());
                    }
                }
            }

            // Mouse moved.
            EventType::MouseMove => {
                let mut evt = to_mouse_event(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = unsafe { widget_opt.as_mut().unwrap().as_mut() };

                    if widget.point_effective(&evt.position().into()) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_move(evt.as_ref());
                        widget.on_mouse_move(evt.as_ref());
                    }
                }
            }

            // Mouse wheeled.
            EventType::MouseWhell => {
                let mut evt = to_mouse_event(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = unsafe { widget_opt.as_mut().unwrap().as_mut() };

                    if widget.point_effective(&evt.position().into()) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_wheel(evt.as_ref());
                        widget.on_mouse_wheel(evt.as_ref());
                    }
                }
            }

            EventType::MouseEnter => {}

            EventType::MouseLeave => {}

            // Key pressed.
            EventType::KeyPress => {
                let evt = to_key_event(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = unsafe { widget_opt.as_mut().unwrap().as_mut() };

                    if widget.is_focus() {
                        widget.inner_key_pressed(&evt);
                        widget.on_key_pressed(&evt);
                    }
                }
            }

            // Key released.
            EventType::KeyRelease => {
                let evt = to_key_event(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = unsafe { widget_opt.as_mut().unwrap().as_mut() };

                    if widget.is_focus() {
                        widget.inner_key_released(&evt);
                        widget.on_key_released(&evt);
                    }
                }
            }

            EventType::FocusIn => {}
            EventType::FocusOut => {}
            EventType::Resize => {}
            EventType::Moved => {}
            EventType::DroppedFile => {}
            EventType::HoveredFile => {}
            EventType::HoveredFileCancelled => {}
            EventType::ReceivedCharacter => {}
            EventType::InputMethod => {}
            EventType::None => {}
        }
    }

    pub fn initialize_dynamic_component(parent: &mut dyn WidgetImpl, widget: &mut dyn WidgetImpl) {
        let window_id = parent.window_id();
        // window_id was 0 means there was no need to initialize the widget, it's created before ApplicationWindow's initialization,
        // widget will be initialized later in function `child_initialize()`.
        if window_id == 0 {
            return;
        }
        if widget.initialized() {
            return;
        }

        // Just check the thread was right or not:
        let _ = Self::window_of(window_id);

        widget.set_parent(parent);
        let board = unsafe { BOARD.load(Ordering::SeqCst).as_mut().unwrap() };
        board.add_element(widget.as_element());
        ApplicationWindow::widgets_of(window_id).insert(widget.name(), NonNull::new(widget));

        let type_registry = TypeRegistry::instance();
        widget.inner_type_register(type_registry);
        widget.type_register(type_registry);
        widget.set_window_id(window_id);

        widget.initialize();
        widget.set_initialized(true);
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
        ApplicationWindow::widgets_of(window_id).insert(child_ref.name(), NonNull::new(child_ptr));

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
        }

        if child_ref.get_raw_parent().is_none() {
            child_ref.set_parent(parent);
        }

        parent = child_ptr;

        child_ref.initialize();
        child_ref.set_initialized(true);

        child = children.pop_front().take().map_or(None, |widget| widget);
    }
}
