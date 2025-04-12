use crate::{
    animation::mgr::AnimationMgr,
    application::FnRunAfter,
    container::ContainerLayoutEnum,
    graphics::{
        board::Board,
        element::{HierachyZ, TOP_Z_INDEX},
    },
    input::{dialog::TyInputDialog, focus_mgr::FocusMgr, ReflectInputEle},
    layout::LayoutMgr,
    loading::LoadingMgr,
    platform::{ipc_bridge::IpcBridge, PlatformType},
    prelude::*,
    primitive::{global_watch::GlobalWatchEvent, Message},
    runtime::{wed, window_context::OutputSender},
    tooltip::TooltipStrat,
    widget::{
        index_children,
        widget_inner::WidgetInnerExt,
        win_widget::{handle_win_widget_create, CrsWinMsgHnd, WinWidgetHnd},
        IterExecutorHnd, WidgetImpl, ZIndexStep,
    },
    window::win_builder::WindowBuilder,
};
use ahash::AHashMap;
use log::{debug, error, warn};
use nohash_hasher::IntMap;
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    ptr::{addr_of_mut, NonNull},
    sync::Once,
    thread::{self, ThreadId},
};
use tlib::{
    connect,
    events::{DeltaType, Event, EventType, MouseEvent},
    figure::Size,
    namespace::{KeyboardModifier, MouseButton},
    nonnull_mut, nonnull_ref,
    object::{ObjectImpl, ObjectSubclass},
    skia_safe::ClipOp,
    values::FromValue,
    winit::window::WindowId,
};

use self::animation::frame_animator::{FrameAnimatorMgr, ReflectFrameAnimator};

thread_local! {
    pub(crate) static WINDOW_ID: RefCell<ObjectId> = const { RefCell::new(0) };
    pub(crate) static INTIALIZE_PHASE: RefCell<bool> = const { RefCell::new(false) };
    static WINDOW_PREPARED_ONCE: Once = const { Once::new() };
}

const DEFAULT_WINDOW_BACKGROUND: Color = Color::WHITE;

#[extends(Widget)]
pub struct ApplicationWindow {
    raw_window_handle: Option<RawWindowHandle6>,
    parent_window: Option<WindowId>,
    winit_id: Option<WindowId>,
    platform_type: PlatformType,
    ipc_bridge: Option<Box<dyn IpcBridge>>,
    shared_widget_size_changed: bool,
    high_load_request: bool,
    defer_display: bool,
    /// Position include the tile bar on the screen coordinate.
    outer_position: Point,
    /// Position to the client area on the screen coordinate.
    client_position: Point,
    params: Option<AHashMap<String, Value>>,
    min_size: Option<Size>,

    board: Option<NonNull<Board>>,
    output_sender: Option<OutputSender>,
    layout_mgr: LayoutMgr,
    widgets: IntMap<ObjectId, WidgetHnd>,
    iter_executors: Vec<IterExecutorHnd>,
    shadow_mouse_watch: Vec<WidgetHnd>,
    crs_win_handlers: Vec<CrsWinMsgHnd>,

    focused_widget: ObjectId,
    focused_widget_mem: Vec<ObjectId>,
    pressed_widget: ObjectId,
    modal_widget: Option<ObjectId>,
    mouse_over_widget: WidgetHnd,
    mouse_enter_widgets: Vec<WidgetHnd>,

    run_after: Option<FnRunAfter>,
    run_afters: Vec<WidgetHnd>,
    watch_map: IntMap<GlobalWatchEvent, HashSet<ObjectId>>,
    overlaids: IntMap<ObjectId, WidgetHnd>,
    root_ancestors: Vec<ObjectId>,
    win_widgets: Vec<WinWidgetHnd>,

    #[cfg(not(win_dialog))]
    input_dialog: Option<Box<crate::input::dialog::InputDialog>>,
    #[cfg(win_dialog)]
    input_dialog: Option<Box<crate::input::dialog::CorrInputDialog>>,
    #[cfg(not(win_tooltip))]
    tooltip: Option<Box<crate::tooltip::Tooltip>>,
    #[cfg(win_tooltip)]
    tooltip: Option<Box<crate::tooltip::CorrTooltip>>,
}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";
}

impl ObjectImpl for ApplicationWindow {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_background(DEFAULT_WINDOW_BACKGROUND);
    }

    fn initialize(&mut self) {
        INTIALIZE_PHASE.with(|p| *p.borrow_mut() = true);
        debug!("Initialize-phase start.");

        if !self.border_ref().should_draw_radius() {
            self.set_render_difference(true);
        }

        Self::widgets_of(self.id()).insert(self.id(), NonNull::new(self));

        let window_id = self.id();
        self.root_ancestors.push(window_id);
        self.set_in_tree();
        child_initialize(self.get_child_mut(), window_id, true);
        if let Some(input_dialog) = self.input_dialog.as_mut() {
            child_initialize(Some(input_dialog.as_mut()), window_id, true);
        }
        if let Some(tooltip) = self.tooltip.as_mut() {
            child_initialize(Some(tooltip.as_mut()), window_id, true);
        }

        self.when_size_change(self.size());

        for w in self.win_widgets.iter_mut() {
            let win_widget = nonnull_mut!(w);
            let inner = cast!(win_widget as PopupImpl).is_none();
            handle_win_widget_create(win_widget, inner);
        }

        self.set_initialized(true);
        INTIALIZE_PHASE.with(|p| *p.borrow_mut() = false);

        debug!("Initialize-phase end.");
    }
}

impl WidgetImpl for ApplicationWindow {
    #[inline]
    fn run_after(&mut self) {
        if let Some(run_after) = self.run_after.take() {
            run_after(self)
        }
        for widget in self.run_afters.iter_mut() {
            let widget = nonnull_mut!(widget);
            widget.run_after();
            #[cfg(verbose_logging)]
            log::info!("[run_after] `{}` executed run after.", widget.name());
        }
        self.run_afters.clear();
    }
}

type ApplicationWindowContext = (ThreadId, Option<NonNull<ApplicationWindow>>);

impl ApplicationWindow {
    #[inline]
    pub fn new(platform_type: PlatformType, width: i32, height: i32) -> Box<ApplicationWindow> {
        let thread_id = thread::current().id();
        let mut window: Box<ApplicationWindow> = Object::new(&[]);
        window.set_fixed_width(width);
        window.set_fixed_height(height);
        window.platform_type = platform_type;
        window.set_window_id(window.id());
        WINDOW_ID.with(|id| *id.borrow_mut() = window.id());
        Self::windows().insert(window.id(), (thread_id, NonNull::new(window.as_mut())));
        window
    }

    /// # Safety
    /// `ApplicationWidnow` and `LayoutManager` can only get and execute in they own ui thread.
    #[inline]
    pub(crate) fn windows() -> &'static mut IntMap<ObjectId, ApplicationWindowContext> {
        static mut WINDOWS: Lazy<IntMap<ObjectId, ApplicationWindowContext>> =
            Lazy::new(IntMap::default);
        unsafe { addr_of_mut!(WINDOWS).as_mut().unwrap() }
    }

    #[inline]
    pub(crate) fn widgets_of(id: ObjectId) -> &'static mut IntMap<ObjectId, WidgetHnd> {
        let window = Self::window_of(id);
        &mut window.widgets
    }

    #[inline]
    pub(crate) fn layout_of(id: ObjectId) -> &'static mut LayoutMgr {
        let window = Self::window_of(id);
        &mut window.layout_mgr
    }

    #[inline]
    pub fn run_afters_of(id: ObjectId) -> &'static mut Vec<WidgetHnd> {
        let window = Self::window_of(id);
        &mut window.run_afters
    }

    #[inline]
    pub fn window_of(id: ObjectId) -> &'static mut ApplicationWindow {
        let current_thread_id = thread::current().id();
        let (thread_id, window) = Self::windows()
            .get_mut(&id)
            .unwrap_or_else(|| panic!("Unkonwn application window with id: {}", id));
        if current_thread_id != *thread_id {
            panic!("Get `ApplicationWindow` in the wrong thread.");
        }
        nonnull_mut!(window)
    }

    /// Get the mutable reference of ApplicationWindow base on thread local window id.
    #[inline]
    pub fn window() -> &'static mut ApplicationWindow {
        let win_id = WINDOW_ID.with(|id| *id.borrow());
        Self::window_of(win_id)
    }

    #[inline]
    pub fn finds<'a, T>(&self) -> Vec<&'a T>
    where
        T: StaticType + WidgetImpl + 'static,
    {
        let mut finds = vec![];
        for (_, widget) in self.widgets.iter() {
            let widget = nonnull_ref!(widget);
            if widget.object_type().is_a(T::static_type()) {
                finds.push(widget.downcast_ref::<T>().unwrap())
            }
        }
        finds
    }

    #[inline]
    pub fn finds_mut<'a, T>(&mut self) -> Vec<&'a mut T>
    where
        T: StaticType + WidgetImpl + 'static,
    {
        let mut finds = vec![];
        for (_, widget) in self.widgets.iter_mut() {
            let widget = nonnull_mut!(widget);
            if widget.object_type().is_a(T::static_type()) {
                finds.push(widget.downcast_mut::<T>().unwrap())
            }
        }
        finds
    }

    #[inline]
    pub fn find_id(&self, id: ObjectId) -> Option<&dyn WidgetImpl> {
        self.widgets.get(&id).map(|w| nonnull_ref!(w))
    }

    #[inline]
    pub fn find_id_mut(&mut self, id: ObjectId) -> Option<&mut dyn WidgetImpl> {
        self.widgets.get_mut(&id).map(|w| nonnull_mut!(w))
    }

    #[inline]
    pub fn find_name(&self, name: &str) -> Option<&dyn WidgetImpl> {
        for (_, widget) in self.widgets.iter() {
            let widget = nonnull_ref!(widget);
            if widget.name().eq(name) {
                return Some(widget);
            }
        }
        None
    }

    #[inline]
    pub fn find_name_mut(&mut self, name: &str) -> Option<&mut dyn WidgetImpl> {
        for (_, widget) in self.widgets.iter_mut() {
            let widget = nonnull_mut!(widget);
            if widget.name().eq(name) {
                return Some(widget);
            }
        }
        None
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
    pub fn register_run_after<R: 'static + FnOnce(&mut Self) + Send>(&mut self, run_after: R) {
        self.run_after = Some(Box::new(run_after));
    }

    #[inline]
    pub fn platform_type(&self) -> PlatformType {
        self.platform_type
    }

    #[inline]
    pub fn create_window(&self, window_bld: WindowBuilder) {
        if self.platform_type == PlatformType::Ipc {
            error!("Can not create window on slave side of shared memory application.");
            return;
        }

        let mut window = window_bld.build();
        window.set_parent(self.winit_id.unwrap());

        if window.is_inner_window() {
            if let Some(rwh) = self.raw_window_handle {
                window.set_parent_window_rwh(rwh)
            }
        }

        self.send_message(Message::CreateWindow(self.winit_id().unwrap(), window));
    }

    #[inline]
    pub fn close(&self) {
        if self.platform_type == PlatformType::Ipc {
            error!("Can not close window on slave side of shared memory application.");
            return;
        }

        self.send_message(Message::WindowCloseRequest(self.winit_id.unwrap()))
    }

    #[inline]
    pub fn maximize(&self) {
        if self.platform_type == PlatformType::Ipc {
            error!("Can not maximize window on slave side of shared memory application.");
            return;
        }

        self.send_message(Message::WindowMaximizeRequest(self.winit_id.unwrap()))
    }

    #[inline]
    pub fn minimize(&self) {
        if self.platform_type == PlatformType::Ipc {
            error!("Can not minimize window on slave side of shared memory application.");
            return;
        }

        self.send_message(Message::WindowMinimizeRequest(self.winit_id.unwrap()))
    }

    #[inline]
    pub fn restore(&self) {
        if self.platform_type == PlatformType::Ipc {
            error!("Can not restore window on slave side of shared memory application.");
            return;
        }

        self.send_message(Message::WindowRestoreRequest(self.winit_id.unwrap()))
    }

    #[inline]
    pub fn window_layout_change(&mut self) {
        Self::layout_of(self.id()).layout_change(self, false)
    }

    #[inline]
    pub fn layout_change(&self, mut widget: &mut dyn WidgetImpl) {
        if !self.initialized() || !widget.initialized() {
            return;
        }

        // Layout changes should be based on its parent widget.
        if let Some(parent) = widget.get_raw_parent_mut() {
            let parent = unsafe { parent.as_mut().unwrap() };
            if !parent.initialized() {
                return;
            }
            widget = parent;
        }

        Self::layout_of(self.id()).layout_change(widget, false);

        widget.update();
    }

    /// If the window has parent window(created by other window),
    /// the parent window will execute the given function closure.
    pub fn call_response<F>(&self, f: F)
    where
        F: FnOnce(&mut ApplicationWindow) + 'static + Send + Sync,
    {
        if let Some(parent_window) = self.parent_window {
            self.send_message(Message::WindowResponse(parent_window, Box::new(f)))
        }
    }

    /// Get the outer position of window.
    #[inline]
    pub fn outer_position(&self) -> Point {
        self.outer_position
    }

    #[inline]
    pub fn map_to_outer(&self, pos: &Point) -> Point {
        Point::new(
            pos.x() + self.outer_position.x(),
            pos.y() + self.outer_position.y(),
        )
    }

    #[inline]
    pub fn map_to_outer_f(&self, pos: &FPoint) -> FPoint {
        FPoint::new(
            pos.x() + self.outer_position.x() as f32,
            pos.y() + self.outer_position.y() as f32,
        )
    }

    /// Get the outer position of window.
    #[inline]
    pub fn client_position(&self) -> Point {
        self.client_position
    }

    #[inline]
    pub fn map_to_client(&self, pos: &Point) -> Point {
        Point::new(
            pos.x() + self.client_position.x(),
            pos.y() + self.client_position.y(),
        )
    }

    #[inline]
    pub fn map_to_client_f(&self, pos: &FPoint) -> FPoint {
        FPoint::new(
            pos.x() + self.client_position.x() as f32,
            pos.y() + self.client_position.y() as f32,
        )
    }

    #[inline]
    pub fn request_win_position(&self, position: Point) {
        self.send_message(Message::WindowPositionRequest(
            self.winit_id.unwrap(),
            position,
        ));
    }

    #[inline]
    pub fn get_param<T: FromValue + StaticType>(&self, key: &str) -> Option<T> {
        self.params.as_ref()?.get(key).map(|p| p.get::<T>())
    }

    #[inline]
    pub fn get_min_size(&self) -> Option<Size> {
        self.min_size
    }

    /// Should set the parent of widget before use this function.
    pub fn initialize_dynamic_component(widget: &mut dyn WidgetImpl, ancestor_is_in_tree: bool) {
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
        if !ancestor_is_in_tree {
            return;
        }

        child_initialize(Some(widget), window_id, ancestor_is_in_tree);

        window.board().shuffle();

        if let Some(parent) = widget.get_parent_mut() {
            window.layout_change(parent);
        } else {
            window.layout_change(widget);
        }

        if let Some(win_widget) = cast_mut!(widget as WinWidget) {
            let inner = cast!(win_widget as PopupImpl).is_none();
            handle_win_widget_create(win_widget, inner);
        }
    }
}

impl ApplicationWindow {
    #[inline]
    pub(crate) fn shared_widget_size_changed(&self) -> bool {
        self.shared_widget_size_changed
    }

    #[inline]
    pub(crate) fn set_shared_widget_size_changed(&mut self, changed: bool) {
        self.shared_widget_size_changed = changed;
    }

    #[inline]
    pub(crate) fn winit_id(&self) -> Option<WindowId> {
        self.winit_id
    }

    #[inline]
    pub(crate) fn set_winit_id(&mut self, id: WindowId) {
        self.winit_id = Some(id)
    }

    #[inline]
    pub(crate) fn set_ipc_bridge(&mut self, ipc_bridge: Option<Box<dyn IpcBridge>>) {
        self.ipc_bridge = ipc_bridge
    }

    #[inline]
    pub(crate) fn ipc_bridge(&self) -> Option<&dyn IpcBridge> {
        self.ipc_bridge.as_deref()
    }

    #[inline]
    pub(crate) fn set_focused_widget(&mut self, id: ObjectId) {
        if self.focused_widget != 0 && self.focused_widget != id {
            if let Some(widget) = self.find_id_mut(self.focused_widget) {
                widget.on_lose_focus();
            }
        }
        if id != 0 {
            if let Some(widget) = self.find_id_mut(id) {
                widget.on_get_focus();

                if cast!(widget as InputEle).is_some() {
                    FocusMgr::with(|m| {
                        m.borrow_mut()
                            .set_currrent(widget.root_ancestor(), Some(id))
                    })
                }
            } else {
                return;
            }
        } else if let Some(widget) = self.find_id_mut(self.focused_widget) {
            FocusMgr::with(|m| m.borrow_mut().set_currrent(widget.root_ancestor(), None))
        }

        self.focused_widget = id
    }

    /// Let the focused widget lose focus temporarily.
    pub(crate) fn temp_lose_focus(&mut self) {
        if self.focused_widget != 0 {
            self.focused_widget_mem.push(self.focused_widget);
            self.set_focused_widget(0);
        }
    }

    /// Restore the previous focused widget.
    pub(crate) fn restore_focus(&mut self) {
        if let Some(focused_widget) = self.focused_widget_mem.pop() {
            self.set_focused_widget(focused_widget);
        }
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
    pub(crate) fn mouse_over_widget(&self) -> WidgetHnd {
        self.mouse_over_widget
    }

    #[inline]
    pub(crate) fn set_mouse_over_widget(&mut self, widget: WidgetHnd) {
        self.mouse_over_widget = widget
    }

    #[inline]
    pub(crate) fn send_message(&self, message: Message) {
        match self.output_sender {
            Some(OutputSender::Sender(ref sender)) => sender.send(message).unwrap(),
            Some(OutputSender::EventLoopProxy(ref sender)) => sender.send_event(message).unwrap(),
            None => panic!("`ApplicationWindow` did not register the output_sender."),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn register_global_watch(&mut self, id: ObjectId, ty: GlobalWatchEvent) {
        self.watch_map.entry(ty).or_default().insert(id);
    }

    #[inline]
    pub(crate) fn handle_global_watch<F: Fn(&mut dyn GlobalWatch) -> bool>(
        &mut self,
        ty: GlobalWatchEvent,
        f: F,
    ) -> bool {
        let mut prevent = false;
        if let Some(ids) = self.watch_map.get(&ty) {
            for &id in ids.clone().iter() {
                if let Some(widget) = self.find_id_mut(id) {
                    if !widget.visible() {
                        continue;
                    }
                    let type_name = widget.type_name();

                    let flag = f(cast_mut!(widget as GlobalWatch).unwrap_or_else(|| {
                        panic!("Widget `{}` has not impl `GlobalWatchImpl`.", type_name)
                    }));
                    if flag {
                        prevent = true;
                    }
                }
            }
        }
        prevent
    }

    #[inline]
    pub(crate) fn handle_overlaids_global_mouse_click(&mut self, evt: &MouseEvent) -> bool {
        let mut prevent = false;
        for (_, overlaid) in self.overlaids.iter_mut() {
            let overlaid = nonnull_mut!(overlaid);
            if let Some(popup) = cast_mut!(overlaid as PopupImpl) {
                if !popup.hide_on_click() {
                    continue;
                }
                if popup.handle_global_mouse_pressed(evt) {
                    prevent = true
                }
            }
        }
        prevent
    }

    #[inline]
    pub(crate) fn animation_layout_change(&self, widget: &mut dyn WidgetImpl) {
        Self::layout_of(self.id()).layout_change(widget, true)
    }

    #[inline]
    pub(crate) fn when_size_change(&mut self, size: Size) {
        emit!(self, size_changed(size));
        Self::layout_of(self.id()).set_window_size(size);
        self.window_layout_change();

        for (_, widget) in self.overlaids.iter_mut() {
            nonnull_mut!(widget).update_render_styles();
        }
    }

    #[inline]
    pub(crate) fn register_output(&mut self, sender: OutputSender) {
        self.output_sender = Some(sender)
    }

    #[inline]
    pub(crate) fn set_board(&mut self, board: &mut Board) {
        self.board = NonNull::new(board);
    }

    #[inline]
    pub(crate) fn board(&mut self) -> &mut Board {
        nonnull_mut!(self.board)
    }

    #[inline]
    pub(crate) fn dispatch_event(&mut self, evt: Event) -> Option<Event> {
        wed::win_evt_dispatch(self, evt)
    }

    #[inline]
    pub(crate) fn iter_execute(&mut self) {
        self.iter_executors
            .iter_mut()
            .for_each(|hnd| nonnull_mut!(hnd).iter_execute());

        self.crs_win_handlers
            .iter_mut()
            .for_each(|hnd| nonnull_mut!(hnd).handle_inner());
    }

    #[inline]
    pub(crate) fn overlaids(&self) -> &IntMap<ObjectId, WidgetHnd> {
        &self.overlaids
    }

    #[inline]
    pub(crate) fn overlaids_mut(&mut self) -> &mut IntMap<ObjectId, WidgetHnd> {
        &mut self.overlaids
    }

    #[inline]
    pub(crate) fn set_modal_widget(&mut self, id: Option<ObjectId>) {
        // Trigger the mouse leave method for all entered widgets
        // when a modal widget is present.
        self.mouse_enter_widgets.retain_mut(|w| {
            let widget = nonnull_mut!(w);
            let mouse_leave = MouseEvent::new(
                EventType::MouseLeave,
                (0, 0),
                MouseButton::NoButton,
                KeyboardModifier::NoModifier,
                0,
                Point::default(),
                DeltaType::default(),
            );
            widget.inner_mouse_leave(&mouse_leave);
            widget.on_mouse_leave(&mouse_leave);

            false
        });

        self.modal_widget = id;
    }

    #[inline]
    pub(crate) fn modal_widget(&mut self) -> Option<&mut dyn WidgetImpl> {
        if let Some(ref id) = self.modal_widget {
            self.widgets.get_mut(id).map(|v| nonnull_mut!(v))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn set_parent_window(&mut self, parent: WindowId) {
        self.parent_window = Some(parent)
    }

    #[inline]
    pub(crate) fn set_raw_window_handle(&mut self, rwh: RawWindowHandle6) {
        self.raw_window_handle = Some(rwh)
    }

    #[inline]
    pub(crate) fn set_min_size(&mut self, size: Option<Size>) {
        self.min_size = size;
    }

    #[inline]
    pub(crate) fn check_mouse_enter(
        &mut self,
        widget: &mut dyn WidgetImpl,
        point: &Point,
        evt: &MouseEvent,
    ) {
        if !widget.rect().contains(point) {
            if let Some(iv) = cast!(widget as IsolatedVisibility) {
                if !iv.shadow_rect().contains_point(point) {
                    return;
                }
            } else {
                return;
            }
        }
        for (&id, overlaid) in self.overlaids.iter() {
            let overlaid = nonnull_ref!(overlaid);
            if widget.id() == id || widget.z_index() > overlaid.z_index() {
                continue;
            }
            if overlaid.rect().contains(point) {
                return;
            }
        }

        let hnd = NonNull::new(widget);
        if !self.mouse_enter_widgets.contains(&hnd) {
            self.mouse_enter_widgets.push(hnd);

            let widget_position = widget.map_to_widget(point);
            let mouse_enter = MouseEvent::new(
                EventType::MouseEnter,
                (widget_position.x(), widget_position.y()),
                evt.mouse_button(),
                evt.modifier(),
                evt.n_press(),
                evt.delta(),
                evt.delta_type(),
            );
            widget.inner_mouse_enter(&mouse_enter);
            widget.on_mouse_enter(&mouse_enter);
        }
    }

    #[inline]
    pub(crate) fn check_mouse_leave(&mut self, point: &Point, evt: &MouseEvent) {
        self.mouse_enter_widgets.retain_mut(|w| {
            let widget = nonnull_mut!(w);
            let mut efct = widget.rect().contains(point);
            for (&id, overlaid) in self.overlaids.iter() {
                let overlaid = nonnull_ref!(overlaid);
                if widget.id() == id || widget.z_index() > overlaid.z_index() {
                    continue;
                }
                if overlaid.rect().contains(point) {
                    efct = false;
                }
            }

            if !efct {
                let widget_position = widget.map_to_widget(point);
                let mouse_leave = MouseEvent::new(
                    EventType::MouseLeave,
                    (widget_position.x(), widget_position.y()),
                    evt.mouse_button(),
                    evt.modifier(),
                    evt.n_press(),
                    evt.delta(),
                    evt.delta_type(),
                );
                widget.inner_mouse_leave(&mouse_leave);
                widget.on_mouse_leave(&mouse_leave);
            }

            efct
        });
    }

    /// The coordinate of `dirty_rect` must be [`World`](tlib::namespace::Coordinate::World).
    ///
    /// @param id: the id of the widget that affected the others.
    pub(crate) fn invalid_effected_widgets(&mut self, dirty_rect: FRect, id: ObjectId) {
        if !self.initialized() {
            return;
        }

        let find = self.find_id(id);
        if find.is_none() {
            warn!(
                "[invalid_effected_widgets()] find widget by id failed, id = {}",
                id
            );
            return;
        }

        let z_index = find.unwrap().z_index();
        for w in self.widgets.values_mut() {
            let widget = nonnull_mut!(w);
            if widget.id() == id || widget.descendant_of(id) || widget.z_index() > z_index {
                continue;
            }

            let rect: tlib::skia_safe::IRect = widget.rect().into();
            let mut region = tlib::skia_safe::Region::new();
            region.op_rect(rect, tlib::skia_safe::region::RegionOp::Replace);
            region.op_region(
                &widget.child_region(),
                tlib::skia_safe::region::RegionOp::Difference,
            );

            let dirty_irect: tlib::skia_safe::IRect = dirty_rect.into();
            if region.intersects_rect(dirty_irect) {
                widget.set_render_styles(true);
                widget.update_rect(CoordRect::new(dirty_rect, Coordinate::World));
            }
        }
    }

    #[inline]
    pub(crate) fn shadow_mouse_watch(&mut self) -> &mut Vec<WidgetHnd> {
        &mut self.shadow_mouse_watch
    }

    #[inline]
    pub(crate) fn add_shadow_mouse_watch(&mut self, widget: &mut dyn WidgetImpl) {
        self.shadow_mouse_watch.push(NonNull::new(widget))
    }

    #[inline]
    pub(crate) fn set_outer_position(&mut self, position: Point) {
        self.outer_position = position
    }

    #[inline]
    pub(crate) fn set_client_position(&mut self, position: Point) {
        self.client_position = position
    }

    #[inline]
    pub(crate) fn root_ancestors(&self) -> &[ObjectId] {
        &self.root_ancestors
    }

    #[inline]
    pub(crate) fn set_params(&mut self, params: Option<AHashMap<String, Value>>) {
        self.params = params
    }

    #[inline]
    pub(crate) fn input_dialog(&mut self) -> &mut TyInputDialog {
        if self.input_dialog.is_none() {
            #[cfg(not(win_dialog))]
            let mut input_dialog = crate::input::dialog::InputDialog::new();

            #[cfg(win_dialog)]
            let mut input_dialog = crate::input::dialog::CorrInputDialog::new();

            Self::initialize_dynamic_component(input_dialog.as_mut(), true);
            self.input_dialog = Some(input_dialog);
        }

        self.input_dialog.as_mut().unwrap()
    }

    #[inline]
    pub(crate) fn tooltip_visible(&self) -> bool {
        if let Some(ref tooltip) = self.tooltip {
            tooltip.visible()
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn tooltip(&mut self, tooltip_strat: TooltipStrat) {
        if self.tooltip.is_none() {
            #[cfg(not(win_tooltip))]
            let mut tooltip = crate::tooltip::Tooltip::new();

            #[cfg(win_tooltip)]
            let mut tooltip = crate::tooltip::CorrTooltip::new();

            Self::initialize_dynamic_component(tooltip.as_mut(), true);
            self.tooltip = Some(tooltip);
        }

        let tooltip = self.tooltip.as_mut().unwrap();

        match tooltip_strat {
            TooltipStrat::Show(text, position, size, styles) => {
                #[cfg(not(win_tooltip))]
                {
                    tooltip.set_fixed_x(position.x());
                    tooltip.set_fixed_y(position.y());
                    tooltip.set_props(text, size, styles);
                    tooltip.calc_relative_position();
                    tooltip.show();
                    ApplicationWindow::window().layout_change(tooltip.as_widget_impl_mut());
                }

                #[cfg(win_tooltip)]
                {
                    tooltip.set_fixed_x(position.x());
                    tooltip.set_fixed_y(position.y());

                    if let Some(width) = size.width() {
                        tooltip.width_request(width)
                    }
                    if let Some(height) = size.height() {
                        tooltip.height_request(height)
                    }

                    tooltip.send_cross_win_msg(crate::tooltip::TooltipCrsMsg::Show(
                        text.to_string(),
                        size,
                        styles,
                    ));
                    tooltip.calc_relative_position();
                    tooltip.show();
                    ApplicationWindow::window().layout_change(tooltip.as_widget_impl_mut());
                }
            }
            TooltipStrat::Hide => tooltip.hide(),
            TooltipStrat::HideOnWindowReisze(on) => tooltip.set_hide_on_win_change(on),
        }
    }

    #[inline]
    pub(crate) fn set_defer_display(&mut self, defer_display: bool) {
        self.defer_display = defer_display
    }

    #[inline]
    pub(crate) fn window_prepared(&self) {
        if self.defer_display {
            WINDOW_PREPARED_ONCE.with(|once| {
                once.call_once(|| {
                    self.send_message(Message::WindowVisibilityRequest(
                        self.winit_id().unwrap(),
                        true,
                    ));
                })
            });
        }
    }

    #[inline]
    pub(crate) fn handle_win_widget_geometry_changed(&self, _: FRect) {
        if !self.initialized() {
            return;
        }
        let id = self.get_signal_source().unwrap();

        let w = self.find_id(id).unwrap();
        let mut rect = w.visual_rect();
        let is_popup = cast!(w as PopupImpl).is_some();
        if is_popup {
            let outer = self.map_to_client_f(&rect.top_left());
            rect.set_point(&outer);
        }

        debug!(
            "Window correspondent widget `{}` geometry changed {:?}, rect record {:?}",
            w.name(),
            rect,
            w.rect_record()
        );
        self.send_message(Message::WinWidgetGeometryChangedRequest(id, rect.into()))
    }

    #[inline]
    pub(crate) fn handle_win_widget_visibility_changed(&self, visible: bool) {
        if !self.initialized() {
            return;
        }
        let id = self.get_signal_source().unwrap();
        self.send_message(Message::WinWidgetVisibilityChangedRequest(id, visible))
    }

    #[inline]
    pub(crate) fn clip_window(&self, painter: &mut Painter) {
        if !self.border_ref().should_draw_radius() {
            return;
        }
        let rect = self.rect_f();
        painter.clip_round_rect_global(rect, self.border_ref().border_radius, ClipOp::Intersect);
    }
}

/// Get window id in current ui thread.
#[inline]
pub fn window_id() -> ObjectId {
    WINDOW_ID.with(|id| *id.borrow())
}

fn child_initialize(
    mut child: Option<&mut dyn WidgetImpl>,
    window_id: ObjectId,
    ancestor_is_in_tree: bool,
) {
    let window = ApplicationWindow::window_of(window_id);
    let type_registry = TypeRegistry::instance();

    let mut children: VecDeque<Option<*mut dyn WidgetImpl>> = VecDeque::new();

    while let Some(child_ref) = child {
        #[cfg(verbose_logging)]
        log::info!(
            "[child_initialize] Initialize the widget {}.",
            child_ref.name()
        );

        window.board().add_element(child_ref.as_element());
        window
            .widgets
            .insert(child_ref.id(), NonNull::new(child_ref));
        index_children(child_ref);

        child_ref.inner_type_register(type_registry);
        child_ref.type_register(type_registry);

        if let Some(pop) = cast!(child_ref as PopupImpl) {
            let supervisor = pop.supervisor();
            window.root_ancestors.push(pop.id());
            child_ref.set_z_index(supervisor.z_index() + TOP_Z_INDEX);
        } else if let Some(parent) = child_ref.get_parent_mut() {
            let zindex = parent.z_index() + parent.z_index_step();
            child_ref.set_z_index(zindex);
        }

        if let Some(parent) = child_ref.get_parent_ref() {
            let is_passing_event_bubble = parent.is_propagate_event_bubble();
            let is_passing_mouse_tracking = parent.is_propagate_mouse_tracking();
            let is_manage_by_container = {
                let container = cast!(parent as ContainerImpl);
                match container {
                    Some(c) => c.container_layout() != ContainerLayoutEnum::Stack,
                    None => false,
                }
            };
            let mouse_tracking = parent.mouse_tracking();

            if is_passing_event_bubble {
                let event_bubble = parent.event_bubble();
                child_ref.set_event_bubble(event_bubble);
                child_ref.set_propagate_event_bubble(true);
            }

            if is_passing_mouse_tracking {
                child_ref.set_mouse_tracking(mouse_tracking);
                child_ref.set_propagate_mouse_tracking(true);
            }

            if is_manage_by_container {
                child_ref.set_manage_by_container(true);
            }
        }

        if ancestor_is_in_tree {
            child_ref.set_in_tree();
        }
        child_ref.inner_initialize();
        child_ref.initialize();
        child_ref.set_initialized(true);

        if let Some(snapshot) = cast_mut!(child_ref as Snapshot) {
            AnimationMgr::with(|m| m.borrow_mut().add_snapshot(snapshot))
        }
        if let Some(loading) = cast_mut!(child_ref as Loadable) {
            LoadingMgr::with(|m| m.borrow_mut().add_loading(loading))
        }
        if let Some(watch) = cast_mut!(child_ref as GlobalWatch) {
            watch.register_global_watch();
        }
        if let Some(executor) = cast_mut!(child_ref as IterExecutor) {
            ApplicationWindow::window_of(window_id)
                .iter_executors
                .push(NonNull::new(executor))
        }
        if let Some(frame_animator) = cast_mut!(child_ref as FrameAnimator) {
            FrameAnimatorMgr::with(|m| m.borrow_mut().add_frame_animator(frame_animator))
        }
        if let Some(input_ele) = cast_mut!(child_ref as InputEle) {
            FocusMgr::with(|m| m.borrow_mut().add(input_ele.root_ancestor(), input_ele))
        }
        if let Some(win_widget) = cast_mut!(child_ref as WinWidget) {
            window.win_widgets.push(NonNull::new(win_widget));
            connect!(
                win_widget,
                geometry_changed(),
                window,
                handle_win_widget_geometry_changed(FRect)
            );
            connect!(
                win_widget,
                visibility_changed(),
                window,
                handle_win_widget_visibility_changed(bool)
            );
        }
        if let Some(crs_win_hnd) = cast_mut!(child_ref as CrossWinMsgHandlerInner) {
            window.crs_win_handlers.push(NonNull::new(crs_win_hnd));
        }

        // Determine whether the widget is a container.
        let is_container = child_ref.super_type().is_a(Container::static_type());
        let container_ref = if is_container {
            cast_mut!(child_ref as ContainerImpl)
        } else {
            None
        };
        let container_children = container_ref.map(|cf| cf.children_mut());

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

        child = children.pop_front().and_then(|widget| unsafe {
            match widget {
                None => None,
                Some(w) => w.as_mut(),
            }
        });
    }
}
