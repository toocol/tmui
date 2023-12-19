use crate::{
    animation::manager::AnimationManager,
    graphics::{board::Board, element::HierachyZ},
    layout::LayoutManager,
    platform::{ipc_bridge::IpcBridge, PlatformType},
    prelude::*,
    primitive::Message,
    runtime::{wed, window_context::OutputSender},
    widget::{WidgetImpl, WidgetSignals, ZIndexStep},
};
use log::debug;
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    ptr::NonNull,
    sync::Once,
    thread::{self, ThreadId},
};
use tlib::{
    connect, emit,
    events::Event,
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
    ipc_bridge: Option<Box<dyn IpcBridge>>,

    board: Option<NonNull<Board>>,
    output_sender: Option<OutputSender>,
    layout_manager: LayoutManager,
    widgets: HashMap<String, Option<NonNull<dyn WidgetImpl>>>,
    run_afters: Vec<Option<NonNull<dyn WidgetImpl>>>,

    /// Effecting when application was under shared memory mode.
    base_offset: Point,

    focused_widget: ObjectId,
    pressed_widget: ObjectId,
    mouse_over_widget: Option<NonNull<dyn WidgetImpl>>,
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

        Self::widgets_of(self.id()).insert(self.name(), NonNull::new(self));

        connect!(self, size_changed(), self, when_size_change(Size));
        let window_id = self.id();
        child_initialize(self.get_child_mut(), window_id);
        emit!(ApplicationWindow::initialize => self.size_changed(), self.size());

        if self.platform_type == PlatformType::Ipc {
            self.base_offset = self.ipc_bridge.as_ref().unwrap().region().top_left();
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
    pub(crate) fn set_ipc_bridge(&mut self, ipc_bridge: Option<Box<dyn IpcBridge>>) {
        self.ipc_bridge = ipc_bridge
    }

    #[inline]
    pub(crate) fn ipc_bridge(&self) -> Option<&Box<dyn IpcBridge>> {
        self.ipc_bridge.as_ref()
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
    pub(crate) fn set_pressed_widget(&mut self, id: ObjectId) {
        self.pressed_widget = id
    }

    #[inline]
    pub(crate) fn pressed_widget(&self) -> ObjectId {
        self.pressed_widget
    }

    #[inline]
    pub(crate) fn mouse_over_widget(&self) -> Option<NonNull<dyn WidgetImpl>> {
        self.mouse_over_widget
    }

    #[inline]
    pub(crate) fn set_mouse_over_widget(&mut self, widget: Option<NonNull<dyn WidgetImpl>>) {
        self.mouse_over_widget = widget
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
    pub(crate) fn register_output(&mut self, sender: OutputSender) {
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
        wed::win_evt_dispatch(self, evt)
    }

    pub(crate) fn invalid_effected_widgets(&mut self, dirty_rect: Rect) {
        for (_, w) in Self::widgets_of(self.id()) {
            let widget = nonnull_mut!(w);

            let rect: tlib::skia_safe::IRect = widget.rect().into();
            let mut region = tlib::skia_safe::Region::new();
            region.op_rect(rect, tlib::skia_safe::region::RegionOp::Replace);
            region.op_region(
                &widget.child_region(),
                tlib::skia_safe::region::RegionOp::Difference,
            );

            let dirty_irect: tlib::skia_safe::IRect = dirty_rect.into();
            if region.intersects_rect(dirty_irect) {
                widget.set_rerender_styles(true);
                widget.update_rect(dirty_rect);
            }
        }
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

        child_initialize(Some(widget), window_id);
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
        board.add_element(child_ref.as_element());
        ApplicationWindow::widgets_of(window_id).insert(child_ref.name(), NonNull::new(child_ref));

        let parent = child_ref.get_parent_mut().unwrap();
        let zindex = parent.z_index() + parent.z_index_step();
        child_ref.set_z_index(zindex);

        child_ref.inner_type_register(type_registry);
        child_ref.type_register(type_registry);

        child_ref.set_initialized(true);
        child_ref.inner_initialize();
        child_ref.initialize();

        if let Some(snapshot) = cast_mut!(child_ref as Snapshot) {
            AnimationManager::with(|m| m.borrow_mut().add_snapshot(snapshot))
        }

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
            let c_child = child_ref.get_raw_child_mut();
            if c_child.is_some() {
                children.push_back(c_child);
            }
        }

        // Determine whether the widget is pupupable.
        if let Some(popupable) = cast_mut!(child_ref as Popupable) {
            if let Some(popup) = popupable.get_popup_mut() {
                children.push_back(Some(popup.as_widget_impl_mut()));
            }
        }

        child = children.pop_front().take().map_or(None, |widget| unsafe {
            match widget {
                None => None,
                Some(w) => w.as_mut(),
            }
        });
    }
}
