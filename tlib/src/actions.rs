#![allow(unused_variables)]
/// The crate of actions system.
///
/// <examples>
///
/// ```
/// use tlib::prelude::*;
/// use tlib::actions::ActionHub;
/// use tlib::object::{ObjectImpl, ObjectSubclass};
/// use tlib::{signals, signal, emit};
///
/// #[extends_object]
/// #[derive(Default)]
/// pub struct Widget {}
///
/// impl ObjectSubclass for Widget {
///     const NAME: &'static str = "Widget";
///
///     type Type = Widget;
///
///     type ParentType = Object;
/// }
///
/// impl ObjectImpl for Widget {}
///
/// impl Widget {
///     signals! {
///         action_test();
///     }
/// }
///
/// fn main() {
///     // Not necessary in actual use. //
///     let mut action_hub = ActionHub::new();
///     action_hub.initialize();
///
///     let widget: Widget = Object::new(&[]);
///     widget.connect_action(widget.action_test(), |param| println!("Hello World!"));
///     emit!(widget.action_test());
/// }
/// ```
use crate::prelude::*;
use lazy_static::lazy_static;
use log::warn;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    ptr::null_mut,
    sync::{
        atomic::AtomicPtr,
        mpsc::{channel, Receiver, Sender},
        Once,
    },
};

static INIT: Once = Once::new();
thread_local! {static IS_MAIN_THREAD: RefCell<bool>  = RefCell::new(false)}
lazy_static! {
    pub static ref ACTION_HUB: AtomicPtr<ActionHub> = AtomicPtr::new(null_mut());
}

/// ActionHub hold all of the registered actions
pub struct ActionHub {
    map: RefCell<HashMap<u16, HashMap<String, HashMap<u16, Box<dyn Fn(Option<Value>)>>>>>,
    sender: Sender<(Signal, Option<Value>)>,
    receiver: Receiver<(Signal, Option<Value>)>,
}
impl ActionHub {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            map: RefCell::new(HashMap::new()),
            sender,
            receiver,
        }
    }

    /// Initialize the `ActionHub`, the instance should be managed by the caller.  
    ///
    /// The thread initialize the `ActionHub` should be the `main` thread.
    ///
    /// This function should only call once.
    pub fn initialize(&mut self) {
        INIT.call_once(|| {
            IS_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);
            ACTION_HUB.store(self as *mut ActionHub, std::sync::atomic::Ordering::SeqCst);
        });
    }

    pub fn process_multi_thread_actions(&self) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`process_multi_thread_actions()` should only call in the `main` thread.")
            }
        });

        while let Ok(action) = self.receiver.try_recv() {
            let signal = action.0;
            let param = action.1;
            if let Some(emiter_map) = self.map.borrow().get(&signal.emiter_id) {
                if let Some(actions) = emiter_map.get(signal.signal()) {
                    actions.iter().for_each(|(target_id, f)| f(param.clone()));
                } else {
                    warn!("Unconnected action: {}", signal.signal());
                }
            } else {
                warn!("Unconnected action: {}", signal.signal());
            }
        }
    }

    pub fn connect_action<T: ActionExt, F: Fn(Option<Value>) + 'static>(
        &self,
        signal: Signal,
        target: &T,
        f: F,
    ) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`connect_action()` should only call in the `main` thread.")
            }
        });

        let mut map_ref = self.map.borrow_mut();
        let emiter_map = map_ref.entry(signal.emiter_id).or_insert(HashMap::new());
        let target_map = emiter_map.entry(signal.signal).or_insert(HashMap::new());
        target_map.insert(target.object_id(), Box::new(f));
    }

    pub fn disconnect_target_action(&self, signal: Option<Signal>, target: Option<u16>) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`disconnect_action()` should only call in the `main` thread.")
            }
        });

        let mut map_ref = self.map.borrow_mut();
    }

    pub fn disconnect_all_action(&self, emiter_id: u16) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`disconnect_action()` should only call in the `main` thread.")
            }
        });

        let mut map_ref = self.map.borrow_mut();
        map_ref.remove(&emiter_id);
    }

    pub fn activate_action(&self, signal: Signal, param: Option<Value>) {
        IS_MAIN_THREAD.with(|is_main| {
            let name = signal.signal();
            if *is_main.borrow() {
                if let Some(emiter_map) = self.map.borrow().get(&signal.emiter_id) {
                    if let Some(actions) = emiter_map.get(name) {
                        actions.iter().for_each(|(target_id, f)| f(param.clone()));
                    } else {
                        warn!("Unconnected action: {}", name);
                    }
                } else {
                    warn!("Unconnected action: {}", name);
                }
            } else {
                self.sender
                    .send((signal, param))
                    .expect("`ActionHub` send actions from multi thread failed.");
            }
        })
    }
}
pub trait ActionExt: Sized + ObjectOperation {
    fn connect<T: ActionExt, F: Fn(Option<Value>) + 'static>(
        &self,
        signal: Signal,
        target: &T,
        f: F,
    ) {
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .connect_action(signal, target, f)
        }
    }

    fn disconnect(&self, signal: Option<Signal>, target: Option<u16>) {
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .disconnect_target_action(signal, target)
        }
    }

    fn disconnect_all(&self) {
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .disconnect_all_action(self.object_id())
        }
    }

    fn create_action_with_no_param(&self, signal: Signal) -> Action {
        Action::with_no_param(signal)
    }

    fn create_action_with_param<T: ToValue + 'static>(&self, signal: Signal, param: T) -> Action {
        Action::with_param(signal, param)
    }

    fn object_id(&self) -> u16 {
        self.id()
    }
}

pub trait AsMutPtr {
    fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}
impl<T: ActionExt> AsMutPtr for T {}

/// The struct represents the subject to emit specified actions.
///
/// `Signal` implements the `Send` + `Sync` trait, so it can be transfered between threads safly.
/// The `Siginal` emited in the different threads will be transfer to the `main` thread to process the action.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Signal {
    emiter_id: u16,
    signal: String,
}
impl Signal {
    pub fn new(emiter_id: u16, signal: String) -> Self {
        Self { emiter_id, signal }
    }

    pub fn emiter_id(&self) -> u16 {
        self.emiter_id
    }

    pub fn signal(&self) -> &String {
        &self.signal
    }
}
impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Signal: [\"{}\"]", self.signal()).as_str())
    }
}
unsafe impl Send for Signal {}
unsafe impl Sync for Signal {}

#[inline]
pub fn ptr_address<T>(obj: &T) -> usize {
    obj as *const T as *const u8 as usize
}

/////////////////////////////////////////////// Macros ///////////////////////////////////////////////
#[allow(unused_macros)]
#[macro_export]
macro_rules! emit {
    ( $signal:expr ) => {{
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .activate_action($signal, None)
        };
    }};
    ( $signal:expr, $($x:expr),+ ) => {{
        let tuple = ($($x),+);
        let value = tuple.to_value();
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .activate_action($signal, Some(value))
        };
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! connect {
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident() ) => {
        let signal = $emiter.$signal();
        let target_ptr = $target.as_mut_ptr();
        $emiter.connect(signal, $target, move |param| {
            unsafe {
                let target = target_ptr.as_mut().expect("Target is None.");
                target.$slot()
            }
        })
    };
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident($param:ident) ) => {
        let signal = $emiter.$signal();
        let target_ptr = $target.as_mut_ptr();
        $emiter.connect(signal, $target, move |param| {
            unsafe {
                let val = param.expect("Param is None.");
                let target = target_ptr.as_mut().expect("Target is None.");
                let param = val.get::<$param>();
                target.$slot(param)
            }
        })
    };
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident($($param:ident:$index:tt),+) ) => {
        let signal = $emiter.$signal();
        let target_ptr = $target.as_mut_ptr();
        $emiter.connect(signal, $target, move |param| {
            unsafe {
                let val = param.expect("Param is None.");
                let target = target_ptr.as_mut().expect("Target is None.");
                let param = val.get::<($($param),+)>();
                target.$slot($(param.$index),+)
            }
        })
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! disconnect {
    ( $emiter:expr, null, null, null ) => {
        $emiter.disconnect_all();
    };
    ( $emiter:expr, $signal:ident(), null, null ) => {
        let signal = $emiter.$signal();
        $emiter.disconnect(Some(signal), None);
    };
    ( $emiter:expr, $signal:ident(), $target:expr, null ) => {
        let signal = $emiter.$signal();
        $emiter.disconnect(Some(signal), Some($target.object_id()));
    };
    ( $emiter:expr, null, $target:expr, null ) => {
        $emiter.disconnect(None, Some($target.object_id()));
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! signal {
    ( $object:expr, $name:expr ) => {{
        Signal::new($object.object_id(), $name.to_string())
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! signals {
    ( $($(#[$($attrss:tt)*])* $name:ident();)* ) => {
        $(
            $(#[$($attrss)*])*
            #[allow(dead_code)]
            fn $name(&self) -> Signal {
                signal!(self, stringify!($name))
            }
        )*
    };
}

/// Struct represents an action which can emit specified action.
pub struct Action {
    signal: Signal,
    param: Option<Box<dyn ToValue>>,
}
impl Action {
    pub fn with_no_param(signal: Signal) -> Self {
        Self {
            signal: signal,
            param: None,
        }
    }

    pub fn with_param<T: ToValue + 'static>(signal: Signal, param: T) -> Self {
        Self {
            signal: signal,
            param: Some(Box::new(param)),
        }
    }

    pub fn emit(&self) {
        if let Some(param) = self.param.as_ref() {
            emit!(self.signal.clone(), param)
        } else {
            emit!(self.signal.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, thread, time::Duration};

    use crate::{
        object::{ObjectImpl, ObjectSubclass},
        prelude::*,
    };

    use super::ActionHub;

    #[extends_object]
    #[derive(Default)]
    pub struct Widget {}

    impl ObjectSubclass for Widget {
        const NAME: &'static str = "Widget";

        type Type = Widget;

        type ParentType = Object;
    }

    impl ObjectImpl for Widget {}

    impl Widget {
        signals! {
            /// Signal: action to test.
            action_test();

            /// Signal: action to demo.
            action_demo();
        }

        pub fn new() -> Self {
            Object::new(&[])
        }

        pub fn reg_action(&mut self) {
            connect!(self, action_test(), self, slot_test(i32:0, String:1));
            connect!(self, action_demo(), self, slot_demo(i32));
        }

        pub fn slot_test(&self, p1: i32, p2: String) {
            println!("Process action test");
            assert_eq!(1, p1);
            assert_eq!("desc", p2);
        }

        pub fn slot_demo(&self, i: i32) {
            println!("Process action demo");
        }

        pub fn emit(&self) {
            let param = 1;
            let desc = "desc";
            emit!(self.action_test(), param, desc);
            emit!(self.action_demo(), param);
        }
    }

    #[test]
    fn test_actions() {
        let mut action_hub = ActionHub::new();
        action_hub.initialize();

        let mut widget = Widget::new();
        widget.reg_action();
        let rc = Rc::new(widget);
        rc.emit();

        let mut join_vec = vec![];
        for _ in 0..5 {
            let signal = rc.action_test();
            join_vec.push(thread::spawn(move || emit!(signal, 1, "desc")));
        }

        thread::sleep(Duration::from_millis(500));
        action_hub.process_multi_thread_actions();
        for join in join_vec {
            join.join().unwrap();
        }

        let action = Action::with_param(rc.action_test(), (1, "desc"));
        action.emit();

        disconnect!(rc, null, null, null);
        disconnect!(rc, null, rc, null);
        disconnect!(rc, action_test(), rc, null);
        disconnect!(rc, action_test(), null, null);
    }

    #[test]
    fn test_signal() {
        let widget = Widget::new();
        let signal = signal!(&widget, "hello");
        println!("{}", signal)
    }
}
