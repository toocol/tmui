#![allow(unused_variables)]
//! The crate of actions system in `tmui`.
//!
//! <examples>
//!
//! ```
//! use tlib::prelude::*;
//! use tlib::actions::ActionHub;
//! use tlib::object::{ObjectImpl, ObjectSubclass};
//! use tlib::{signals, signal, connect, emit};
//!
//! #[extends(Object)]
//! #[derive(Default)]
//! pub struct Widget {}
//!
//! impl ObjectSubclass for Widget {
//!     const NAME: &'static str = "Widget";
//!
//!     type Type = Widget;
//!
//!     type ParentType = Object;
//! }
//!
//! impl ObjectImpl for Widget {}
//!
//! impl Widget {
//!     signals! {
//!         action_test();
//!     }
//!
//!     fn slot(&self) {
//!     }
//! }
//!
//! fn main() {
//!     // Not necessary in actual use. //
//!     let mut action_hub = ActionHub::new();
//!     action_hub.initialize();
//!
//!     let mut widget: Widget = Object::new(&[]);
//!     connect!(widget, action_test(), widget, slot());
//!     emit!(widget.action_test());
//! }
//! ```
use crate::prelude::*;
use log::{debug, error};
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Once,
    },
};

static INIT: Once = Once::new();
pub static ACTIVATE: AtomicBool = AtomicBool::new(false);

thread_local! {static IS_MAIN_THREAD: RefCell<bool>  = RefCell::new(false)}

/// ActionHub hold all of the registered actions
pub struct ActionHub {
    map: Box<
        HashMap<
            u16,
            (
                HashSet<u16>,
                HashMap<String, HashMap<u16, Vec<Box<dyn Fn(&Option<Value>)>>>>,
            ),
        >,
    >,
    sender: Sender<(Signal, Option<Value>)>,
    receiver: Receiver<(Signal, Option<Value>)>,
}
impl ActionHub {
    pub fn new() -> Box<Self> {
        let (sender, receiver) = channel();
        Box::new(Self {
            map: Box::new(HashMap::new()),
            sender,
            receiver,
        })
    }

    /// Get the singleton instance of `ActionHub`
    pub fn instance() -> &'static mut Self {
        static mut ACTION_HUB: Lazy<Box<ActionHub>> = Lazy::new(|| ActionHub::new());
        unsafe { &mut ACTION_HUB }
    }

    /// Initialize the `ActionHub`, the instance should be managed by the caller.
    ///
    /// The thread initialize the `ActionHub` should be the `main` thread.
    ///
    /// This function should only call once.
    pub fn initialize(self: &mut Box<Self>) {
        INIT.call_once(|| {
            IS_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);
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
            if let Some((_, emiter_map)) = self.map.get(&signal.emiter_id) {
                if let Some(actions) = emiter_map.get(signal.signal()) {
                    actions
                        .iter()
                        .for_each(|(target_id, fns)| fns.iter().for_each(|f| f(&param)));
                } else {
                    debug!("Unconnected action: {}", signal.signal());
                }
            } else {
                debug!("Unconnected action: {}", signal.signal());
            }
        }
    }

    pub fn connect_action<F: Fn(&Option<Value>) + 'static>(
        &mut self,
        signal: Signal,
        target: u16,
        f: F,
    ) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`connect_action()` should only call in the `main` thread.")
            }
        });
        if !ACTIVATE.load(Ordering::SeqCst) {
            error!("Signal/Slot should connect in `ObjectImpl::initialize()`, or be connected after application was activated(after UI building).");
            return;
        }

        let map_ref = self.map.as_mut();
        let (target_set, signal_map) = map_ref
            .entry(signal.emiter_id)
            .or_insert((HashSet::new(), HashMap::new()));
        let actions = signal_map
            .entry(signal.signal)
            .or_insert(HashMap::new())
            .entry(target)
            .or_insert(vec![]);
        target_set.insert(target);
        actions.push(Box::new(f));
    }

    pub fn disconnect_action(
        &mut self,
        emiter: Option<u16>,
        signal: Option<&str>,
        target: Option<u16>,
    ) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`disconnect_action()` should only call in the `main` thread.")
            }
        });

        let map_ref = self.map.as_mut();

        // When disconnect with specified signal(`signal.is_some()`),
        // we assume that other signals also hold the action function of the target id,
        // so it is not necessary to remove the target id from the target set.
        // (This can be solved by counting the target, but it doesn't seem necessary at present.)
        //
        // Bechmarks: `connected 10000 widgets with 100 slots/widget and 1 signal,
        //             with the most time-consuming way to disconnect
        //             (emiter.is_none() && signal.is_none() && target.is_some())`
        // No-Target-Set time:   [495.17 µs 506.13 µs 517.70 µs]
        // With-Target-Set time: [190.71 µs 193.56 µs 196.83 µs]
        //
        // This is just that there is only one signal for each connection.
        // If there are multiple signals, the performance improvement will be more obvious.
        if emiter.is_none() && signal.is_none() && target.is_some() {
            for (_, (target_set, signal_map)) in map_ref.iter_mut() {
                let target = target.as_ref().unwrap();
                if !target_set.contains(target) {
                    continue;
                }
                for (_, target_map) in signal_map.iter_mut() {
                    target_map.remove(target);
                }
                target_set.remove(target);
            }
        } else if emiter.is_some() && signal.is_none() && target.is_none() {
            map_ref.remove(emiter.as_ref().unwrap());
        } else if emiter.is_some() && signal.is_some() && target.is_none() {
            if let Some((_, signal_map)) = map_ref.get_mut(emiter.as_ref().unwrap()) {
                signal_map.remove(signal.unwrap());
            }
        } else if emiter.is_some() && signal.is_some() && target.is_some() {
            if let Some((_, signal_map)) = map_ref.get_mut(emiter.as_ref().unwrap()) {
                if let Some(target_map) = signal_map.get_mut(signal.unwrap()) {
                    target_map.remove(target.as_ref().unwrap());
                }
            }
        } else if emiter.is_some() && signal.is_none() && target.is_some() {
            if let Some((target_set, signal_map)) = map_ref.get_mut(emiter.as_ref().unwrap()) {
                let target = target.as_ref().unwrap();
                if !target_set.contains(target) {
                    return;
                }
                for (_, target_map) in signal_map.iter_mut() {
                    target_map.remove(target);
                }
                target_set.remove(target);
            }
        }
    }

    pub fn activate_action(&self, signal: Signal, param: Option<Value>) {
        IS_MAIN_THREAD.with(|is_main| {
            let name = signal.signal();
            if *is_main.borrow() {
                if let Some((_, emiter_map)) = self.map.get(&signal.emiter_id) {
                    if let Some(actions) = emiter_map.get(name) {
                        actions
                            .iter()
                            .for_each(|(target_id, fns)| fns.iter().for_each(|f| f(&param)));
                    } else {
                        debug!("Unconnected action: {}", name);
                    }
                } else {
                    debug!("Unconnected action: {}", name);
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
    fn connect<F: Fn(&Option<Value>) + 'static>(&self, signal: Signal, target: u16, f: F) {
        ActionHub::instance().connect_action(signal, target, f)
    }

    fn disconnect(&self, emiter: Option<u16>, signal: Option<&str>, target: Option<u16>) {
        ActionHub::instance().disconnect_action(emiter, signal, target)
    }

    fn disconnect_all(&self) {
        ActionHub::instance().disconnect_action(Some(self.object_id()), None, None)
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
        ActionHub::instance().activate_action($signal, None);
    }};
    ( $signal:expr, $($x:expr),+ ) => {{
        let tuple = ($($x),+);
        let value = tuple.to_value();
        ActionHub::instance().activate_action($signal, Some(value));
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! connect {
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident() ) => {
        let target_ptr = $target.as_mut_ptr();
        let id = $target.object_id();
        let signal = $emiter.$signal();
        $emiter.connect(signal, id, move |_| {
            let target = unsafe { target_ptr.as_mut().expect("Target is None.") };
            target.$slot()
        })
    };
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident($param:ident) ) => {
        let target_ptr = $target.as_mut_ptr();
        let id = $target.object_id();
        let signal = $emiter.$signal();
        $emiter.connect(signal, id, move |param| {
            let val = param.as_ref().expect("Param is None.");
            let target = unsafe { target_ptr.as_mut().expect("Target is None.") };
            let param = val.get::<$param>();
            target.$slot(param)
        })
    };
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident($($param:ident:$index:tt),+) ) => {
        let target_ptr = $target.as_mut_ptr();
        let id = $target.object_id();
        let signal = $emiter.$signal();
        $emiter.connect(signal, id, move |param| {
            let val = param.as_ref().expect("Param is None.");
            let target = unsafe { target_ptr.as_mut().expect("Target is None.") };
            let param = val.get::<($($param),+)>();
            target.$slot($(param.$index),+)
        })
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! disconnect {
    ( null, null, $target:expr, null ) => {
        $target.disconnect(None, None, Some($target.object_id()));
    };
    ( $emiter:expr, null, null, null ) => {
        $emiter.disconnect_all();
    };
    ( $emiter:expr, $signal:ident(), null, null ) => {
        let signal = $emiter.$signal();
        $emiter.disconnect(Some($emiter.object_id()), Some(signal.signal()), None);
    };
    ( $emiter:expr, $signal:ident(), $target:expr, null ) => {
        let signal = $emiter.$signal();
        $emiter.disconnect(
            Some($emiter.object_id()),
            Some(signal.signal()),
            Some($target.object_id()),
        );
    };
    ( $emiter:expr, null, $target:expr, null ) => {
        $emiter.disconnect(Some($emiter.object_id()), None, Some($target.object_id()));
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
            /// #### Signal/Slot should connect in `ObjectImpl::initialize()` function, or be connected after application was activated(after UI building).
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
    use super::ActionHub;
    use crate::{
        actions::ACTIVATE,
        object::{ObjectImpl, ObjectSubclass},
        prelude::*,
    };
    use std::{rc::Rc, sync::atomic::Ordering, thread, time::Duration};

    #[extends(Object)]
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

            /// Signal: action with no param.
            action_no_param();
        }

        pub fn new() -> Self {
            Object::new(&[])
        }

        pub fn reg_action(&mut self) {
            connect!(self, action_test(), self, slot_test(i32:0, String:1));
            connect!(self, action_demo(), self, slot_demo(i32));
            connect!(self, action_no_param(), self, slot_no_param());
        }

        pub fn slot_test(&self, p1: i32, p2: String) {
            println!("Process action test");
            assert_eq!(1, p1);
            assert_eq!("desc", p2);
        }

        pub fn slot_demo(&self, i: i32) {
            assert_eq!(i, 1);
            println!("Process action demo");
        }

        pub fn slot_no_param(&self) {
            println!("Process action no param");
        }

        pub fn emit(&self) {
            let param = 1;
            let desc = "desc";
            emit!(self.action_test(), param, desc);
            emit!(self.action_demo(), param);
            emit!(self.action_no_param());
        }
    }

    #[test]
    fn test_actions() {
        let mut action_hub = ActionHub::new();
        action_hub.initialize();
        ACTIVATE.store(true, Ordering::SeqCst);

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

        disconnect!(null, null, rc, null);
        disconnect!(rc, null, null, null);
        disconnect!(rc, null, rc, null);
        disconnect!(rc, action_test(), rc, null);
        disconnect!(rc, action_test(), null, null);
        rc.emit();
    }

    #[test]
    fn test_signal() {
        let widget = Widget::new();
        let signal = signal!(&widget, "hello");
        println!("{}", signal)
    }
}
