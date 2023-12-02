use crate::{
    application::PLATFORM_CONTEXT,
    graphics::{board::Board, element::HierachyZ},
    layout::LayoutManager,
    platform::PlatformType,
    prelude::*,
    primitive::Message,
    runtime::window_context::OutputSender,
    widget::{WidgetImpl, WidgetSignals, ZIndexStep},
};
use log::debug;
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    ptr::NonNull,
    sync::{atomic::Ordering, Once},
    thread::{self, ThreadId},
};
use tlib::{
    connect, emit,
    events::{downcast_event, Event, EventType, KeyEvent, MouseEvent, ResizeEvent},
    figure::{Color, Size},
    nonnull_mut, nonnull_ref,
    object::{ObjectImpl, ObjectSubclass},
};

thread_local! {
    pub(crate) static WINDOW_ID: RefCell<ObjectId> = RefCell::new(0);
    pub(crate) static INTIALIZE_PHASE: RefCell<bool> = RefCell::new(false);
}

static INIT: Once = Once::new();

#[extends(Widget)]
pub struct ApplicationWindow {
    platform_type: PlatformType,
    board: Option<NonNull<Board>>,
    output_sender: Option<OutputSender>,
    layout_manager: LayoutManager,
    widgets: HashMap<String, Option<NonNull<dyn WidgetImpl>>>,
    run_afters: Vec<Option<NonNull<dyn WidgetImpl>>>,
    base_offset: Point,

    focused_widget: ObjectId,
    high_load_request: bool,
}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";
}

impl ObjectImpl for ApplicationWindow {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_rerender_difference(true)
    }

    fn initialize(&mut self) {
        INTIALIZE_PHASE.with(|p| *p.borrow_mut() = true);
        debug!("Initialize-phase start.");

        connect!(self, size_changed(), self, when_size_change(Size));
        let window_id = self.id();
        child_initialize(self.get_child_mut(), window_id);
        emit!(ApplicationWindow::initialize => self.size_changed(), self.size());

        if self.platform_type == PlatformType::Ipc {
            let platform_context =
                unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_mut().unwrap() };
            self.base_offset = platform_context.region().top_left();
        }

        self.set_initialized(true);
        INTIALIZE_PHASE.with(|p| *p.borrow_mut() = false);
        debug!("Initialize-phase end.");
    }
}

impl WidgetImpl for ApplicationWindow {
    #[inline]
    fn run_after(&mut self) {
        for widget in self.run_afters.iter_mut() {
            nonnull_mut!(widget).run_after()
        }
        self.run_afters.clear();
    }
}

type ApplicationWindowContext = (ThreadId, Option<NonNull<ApplicationWindow>>);

impl ApplicationWindow {
    #[inline]
    pub fn new(platform_type: PlatformType, width: i32, height: i32) -> Box<ApplicationWindow> {
        let thread_id = thread::current().id();
        let mut window: Box<ApplicationWindow> =
            Object::new(&[("width", &width), ("height", &height)]);
        window.platform_type = platform_type;
        window.set_window_id(window.id());
        WINDOW_ID.with(|id| *id.borrow_mut() = window.id());
        Self::windows().insert(window.id(), (thread_id, NonNull::new(window.as_mut())));
        window
    }

    /// SAFETY: `ApplicationWidnow` and `LayoutManager` can only get and execute in they own ui thread.
    #[inline]
    pub(crate) fn windows() -> &'static mut Box<HashMap<ObjectId, ApplicationWindowContext>> {
        static mut WINDOWS: Lazy<Box<HashMap<ObjectId, ApplicationWindowContext>>> =
            Lazy::new(|| Box::new(HashMap::new()));
        unsafe { &mut WINDOWS }
    }

    #[inline]
    pub(crate) fn widgets_of(
        id: ObjectId,
    ) -> &'static mut HashMap<String, Option<NonNull<dyn WidgetImpl>>> {
        let window = Self::window_of(id);
        &mut window.widgets
    }

    #[inline]
    pub(crate) fn layout_of(id: ObjectId) -> &'static mut LayoutManager {
        let window = Self::window_of(id);
        &mut window.layout_manager
    }

    #[inline]
    pub fn run_afters_of(id: ObjectId) -> &'static mut Vec<Option<NonNull<dyn WidgetImpl>>> {
        let window = Self::window_of(id);
        &mut window.run_afters
    }

    #[inline]
    pub fn window_of(id: ObjectId) -> &'static mut ApplicationWindow {
        let current_thread_id = thread::current().id();
        let (thread_id, window) = Self::windows()
            .get_mut(&id)
            .expect(&format!("Unkonwn application window with id: {}", id));
        if current_thread_id != *thread_id {
            panic!("Get `ApplicationWindow` in the wrong thread.");
        }
        nonnull_mut!(window)
    }

    #[inline]
    pub fn send_message_with_id(id: ObjectId, message: Message) {
        Self::window_of(id).send_message(message)
    }

    #[inline]
    pub fn finds<'a, T>(id: ObjectId) -> Vec<&'a T>
    where
        T: StaticType + WidgetImpl + 'static,
    {
        let mut finds = vec![];
        for (_, widget) in Self::widgets_of(id).iter() {
            let widget = nonnull_ref!(widget);
            if widget.object_type().is_a(T::static_type()) {
                finds.push(widget.downcast_ref::<T>().unwrap())
            }
        }
        finds
    }

    #[inline]
    pub fn finds_mut<'a, T>(id: ObjectId) -> Vec<&'a mut T>
    where
        T: StaticType + WidgetImpl + 'static,
    {
        let mut finds = vec![];
        for (_, widget) in Self::widgets_of(id).iter_mut() {
            let widget = nonnull_mut!(widget);
            if widget.object_type().is_a(T::static_type()) {
                finds.push(widget.downcast_mut::<T>().unwrap())
            }
        }
        finds
    }

    #[inline]
    pub(crate) fn is_initialize_phase() -> bool {
        INTIALIZE_PHASE.with(|p| *p.borrow())
    }

    #[inline]
    pub fn high_load_request(&mut self, high_load: bool) {
        self.high_load_request = high_load
    }

    #[inline]
    pub fn is_high_load_requested(&self) -> bool {
        self.high_load_request
    }

    #[inline]
    pub(crate) fn base_offset(&self) -> Point {
        self.base_offset
    }

    #[inline]
    pub(crate) fn set_focused_widget(&mut self, id: ObjectId) {
        self.focused_widget = id
    }

    #[inline]
    pub(crate) fn focused_widget(&self) -> ObjectId {
        self.focused_widget
    }

    #[inline]
    pub fn platform_type(&self) -> PlatformType {
        self.platform_type
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
    pub fn window_layout_change(&mut self) {
        Self::layout_of(self.id()).layout_change(self)
    }

    #[inline]
    pub fn layout_change(&self, widget: &mut dyn WidgetImpl) {
        Self::layout_of(self.id()).layout_change(widget)
    }

    #[inline]
    pub(crate) fn when_size_change(&mut self, size: Size) {
        Self::layout_of(self.id()).set_window_size(size);
        self.window_layout_change();
    }

    #[inline]
    pub(crate) fn register_window(&mut self, sender: OutputSender) {
        self.output_sender = Some(sender)
    }

    #[inline]
    pub(crate) fn set_board(&mut self, board: &mut Board) {
        INIT.call_once(move || {
            self.board = NonNull::new(board);
        });
    }

    #[inline]
    pub(crate) fn board(&mut self) -> &mut Board {
        nonnull_mut!(self.board)
    }

    #[inline]
    pub(crate) fn dispatch_event(&mut self, evt: Event) -> Option<Event> {
        let mut event: Option<Event> = None;
        match evt.event_type() {
            // Window Resize.
            EventType::Resize => {
                let evt = downcast_event::<ResizeEvent>(evt).unwrap();
                self.resize(Some(evt.width()), Some(evt.height()));
                event = Some(evt);
            }

            // Mouse pressed.
            EventType::MouseButtonPress => {
                let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = nonnull_mut!(widget_opt);

                    if widget.point_effective(&pos) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_pressed(evt.as_ref());
                        widget.on_mouse_pressed(evt.as_ref());

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                }
            }

            // Mouse released.
            EventType::MouseButtonRelease => {
                let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = nonnull_mut!(widget_opt);

                    if widget.point_effective(&pos) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_released(evt.as_ref());
                        widget.on_mouse_released(evt.as_ref());

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                }
            }

            // Mouse moved.
            EventType::MouseMove => {
                let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = nonnull_mut!(widget_opt);

                    if widget.point_effective(&evt.position().into()) {
                        if !widget.mouse_tracking() {
                            break;
                        }
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_move(evt.as_ref());
                        widget.on_mouse_move(evt.as_ref());

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                }
            }

            // Mouse wheeled.
            EventType::MouseWhell => {
                let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());
                let pos = evt.position().into();

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = nonnull_mut!(widget_opt);

                    if widget.point_effective(&evt.position().into()) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));
                        widget.inner_mouse_wheel(evt.as_ref());
                        widget.on_mouse_wheel(evt.as_ref());

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                }
            }

            EventType::MouseEnter => event = Some(evt),

            EventType::MouseLeave => event = Some(evt),

            // Key pressed.
            EventType::KeyPress => {
                let evt = downcast_event::<KeyEvent>(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = nonnull_mut!(widget_opt);

                    if widget.id() == self.focused_widget {
                        widget.inner_key_pressed(&evt);
                        widget.on_key_pressed(&evt);

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                }
            }

            // Key released.
            EventType::KeyRelease => {
                let evt = downcast_event::<KeyEvent>(evt).unwrap();
                let widgets_map = Self::widgets_of(self.id());

                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = nonnull_mut!(widget_opt);

                    if widget.id() == self.focused_widget {
                        widget.inner_key_released(&evt);
                        widget.on_key_released(&evt);

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                }
            }

            EventType::FocusIn => event = Some(evt),
            EventType::FocusOut => event = Some(evt),
            EventType::Moved => {}
            EventType::DroppedFile => {}
            EventType::HoveredFile => {}
            EventType::HoveredFileCancelled => {}
            EventType::ReceivedCharacter => {}
            EventType::InputMethod => {}
            EventType::None => {}
        }

        event
    }

    /// Should set the parent of widget before use this function.
    pub fn initialize_dynamic_component(widget: &mut dyn WidgetImpl) {
        INTIALIZE_PHASE.with(|p| {
            if *p.borrow() {
                panic!(
                    "`{}` Can not add ui component in function `ObjectImpl::initialize()`.",
                    widget.name()
                )
            }
        });

        if widget.initialized() {
            return;
        }

        let window_id = widget.window_id();
        if window_id == 0 {
            return;
        }
        let window = Self::window_of(window_id);
        // There was no need to initialize the widget, it's created before ApplicationWindow's initialization,
        // widget will be initialized later in function `child_initialize()`.
        if !window.initialized() {
            return;
        }

        let parent = widget.get_parent_mut().unwrap();
        let zindex = parent.z_index() + parent.z_index_step();
        widget.set_z_index(zindex);

        let board = window.board();
        board.add_element(widget.as_element());
        ApplicationWindow::widgets_of(window_id).insert(widget.name(), NonNull::new(widget));

        let type_registry = TypeRegistry::instance();
        widget.inner_type_register(type_registry);
        widget.type_register(type_registry);

        widget.set_initialized(true);
        widget.initialize();
    }
}

/// Get window id in current ui thread.
#[inline]
pub fn current_window_id() -> ObjectId {
    WINDOW_ID.with(|id| *id.borrow())
}

fn child_initialize(mut child: Option<&mut dyn WidgetImpl>, window_id: ObjectId) {
    let board = ApplicationWindow::window_of(window_id).board();
    let type_registry = TypeRegistry::instance();
    let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();
    while let Some(child_ref) = child {
        let parent = child_ref.get_parent_mut().unwrap();
        let zindex = parent.z_index() + parent.z_index_step();
        child_ref.set_z_index(zindex);

        board.add_element(child_ref.as_element());
        ApplicationWindow::widgets_of(window_id).insert(child_ref.name(), NonNull::new(child_ref));

        child_ref.inner_type_register(type_registry);
        child_ref.type_register(type_registry);

        child_ref.set_initialized(true);
        child_ref.inner_initialize();
        child_ref.initialize();

        // Determine whether the widget is a container.
        let is_container = child_ref.super_type().is_a(Container::static_type());
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

        child = children.pop_front().take().map_or(None, |widget| unsafe {
            match widget {
                None => None,
                Some(w) => w.as_mut(),
            }
        });
    }
}
