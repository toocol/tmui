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
//! pub struct Widget {}
//!
//! impl ObjectSubclass for Widget {
//!     const NAME: &'static str = "Widget";
//! }
//!
//! impl ObjectImpl for Widget {}
//!
//! impl Widget {
//!     signals! {
//!         Widget:
//!
//!         action_test();
//!     }
//!
//!     fn slot(&self) {
//!     }
//! }
//!
//! fn test() {
//!     // Not necessary in actual use. //
//!     ActionHub::initialize();
//!
//!     let mut widget: Box<Widget> = Object::new(&[]);
//!     connect!(widget, action_test(), widget, slot());
//!     emit!(widget.action_test());
//! }
//! ```
use crate::prelude::*;
use log::debug;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::atomic::{AtomicPtr, Ordering}, ptr::null_mut,
};

type ActionsMap = Box<
    HashMap<
        ObjectId,
        (
            HashSet<ObjectId>,
            HashMap<String, HashMap<ObjectId, Vec<Box<dyn Fn(&Option<Value>)>>>>,
        ),
    >,
>;

thread_local! {
    static INSTANCE: RefCell<Box<ActionHub>> = RefCell::new(ActionHub::new());
    static INSTANCE_PTR: AtomicPtr<ActionHub> = AtomicPtr::new(null_mut());
}

/// ActionHub hold all of the registered actions
pub struct ActionHub {
    map: ActionsMap,
}
impl ActionHub {
    #[inline]
    pub fn new() -> Box<Self> {
        Box::new(Self {
            map: ActionsMap::default(),
        })
    }

    pub fn initialize() {
        INSTANCE.with(|ins| {
            INSTANCE_PTR.with(|ptr| {
                ptr.store(ins.borrow_mut().as_mut(), Ordering::Release)
            })
        });
    }

    #[inline]
    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        INSTANCE_PTR.with(|ptr| {
            let hub = unsafe { ptr.load(Ordering::Acquire).as_mut().unwrap() };
            f(hub)
        })
    }

    pub fn connect_action<F: Fn(&Option<Value>) + 'static>(
        &mut self,
        signal: Signal,
        target: ObjectId,
        f: F,
    ) {
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
        emiter: Option<ObjectId>,
        signal: Option<&str>,
        target: Option<ObjectId>,
    ) {
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
        let name = signal.signal();
        let from_type = signal.from_type();
        let emit_type = signal.emit_type();

        if let Some((_, emiter_map)) = self.map.get(&signal.emiter_id) {
            if let Some(actions) = emiter_map.get(name) {
                actions
                    .iter()
                    .for_each(|(target_id, fns)| fns.iter().for_each(|f| f(&param)));
            } else {
                debug!(
                    "Unconnected action: {}, from_type: {:?}, emit_type: {:?}",
                    name, from_type, emit_type
                );
            }
        } else {
            debug!(
                "Unconnected action: {}, from_type: {:?}, emit_type: {:?}",
                name, from_type, emit_type
            );
        }
    }
}

pub type FnHandleValue = Box<dyn Fn(&Option<Value>)>;

pub trait ActionExt: ObjectOperation {
    fn connect(&self, signal: Signal, target: ObjectId, f: FnHandleValue) {
        ActionHub::with(|hub| {
            hub.connect_action(signal, target, f)
        });
    }

    fn disconnect(&self, emiter: Option<ObjectId>, signal: Option<&str>, target: Option<ObjectId>) {
        ActionHub::with(|hub| {
            hub.disconnect_action(emiter, signal, target)
        });
    }

    fn disconnect_all(&self) {
        ActionHub::with(|hub| {
            hub.disconnect_action(Some(self.id()), None, None)
        });
    }

    fn create_action_with_no_param(&self, signal: Signal) -> Action {
        Action::with_no_param(signal)
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
    emiter_id: ObjectId,
    signal: String,
    from_type: Option<String>,
    emit_type: Option<String>,
}
impl Signal {
    #[inline]
    pub fn new(emiter_id: ObjectId, signal: String) -> Self {
        Self {
            emiter_id,
            signal,
            from_type: None,
            emit_type: None,
        }
    }

    #[inline]
    pub fn emiter_id(&self) -> ObjectId {
        self.emiter_id
    }

    #[inline]
    pub fn signal(&self) -> &String {
        &self.signal
    }

    #[inline]
    pub fn set_from_type(&mut self, from_type: String) {
        self.from_type = Some(from_type)
    }

    #[inline]
    pub fn from_type(&self) -> Option<&str> {
        self.from_type.as_deref()
    }

    #[inline]
    pub fn set_emit_type(&mut self, emit_type: String) {
        self.emit_type = Some(emit_type)
    }

    #[inline]
    pub fn emit_type(&self) -> Option<&str> {
        self.emit_type.as_deref()
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
    ( $emit_type:expr => $signal:expr ) => {{
        let mut signal = $signal;
        signal.set_emit_type(stringify!($emit_type).to_string());
        ActionHub::with(|hub| {
            hub.activate_action(signal, None);
        });
    }};
    ( $emit_type:expr => $signal:expr, $($x:expr),+ ) => {{
        let tuple = ($($x),+);
        let value = tuple.to_value();
        let mut signal = $signal;
        signal.set_emit_type(stringify!($emit_type).to_string());
        ActionHub::with(|hub| {
            hub.activate_action(signal, Some(value));
        });
    }};
    ( $signal:expr ) => {{
        ActionHub::with(|hub| {
            hub.activate_action($signal, None);
        });
    }};
    ( $signal:expr, $($x:expr),+ ) => {{
        let tuple = ($($x),+);
        let value = tuple.to_value();
        ActionHub::with(|hub| {
            hub.activate_action($signal, Some(value));
        });
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! connect {
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident() ) => {
        let target_ptr = $target.as_mut_ptr();
        let id = $target.id();
        let signal = $emiter.$signal();
        $emiter.connect(signal, id, Box::new(move |_| {
            let target = unsafe { target_ptr.as_mut().expect("Target is None.") };
            target.$slot()
        }))
    };
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident($param:ident) ) => {
        let target_ptr = $target.as_mut_ptr();
        let id = $target.id();
        let signal = $emiter.$signal();
        $emiter.connect(signal, id, Box::new(move |param| {
            let val = param.as_ref().expect("Param is None.");
            let target = unsafe { target_ptr.as_mut().expect("Target is None.") };
            let param = val.get::<$param>();
            target.$slot(param)
        }))
    };
    ( $emiter:expr, $signal:ident(), $target:expr, $slot:ident($($param:ident:$index:tt),+) ) => {
        let target_ptr = $target.as_mut_ptr();
        let id = $target.id();
        let signal = $emiter.$signal();
        $emiter.connect(signal, id, Box::new(move |param| {
            let val = param.as_ref().expect("Param is None.");
            let target = unsafe { target_ptr.as_mut().expect("Target is None.") };
            let param = val.get::<($($param),+)>();
            target.$slot($(param.$index),+)
        }))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! disconnect {
    ( null, null, $target:expr, null ) => {
        $target.disconnect(None, None, Some($target.id()));
    };
    ( $emiter:expr, null, null, null ) => {
        $emiter.disconnect_all();
    };
    ( $emiter:expr, $signal:ident(), null, null ) => {
        let signal = $emiter.$signal();
        $emiter.disconnect(Some($emiter.id()), Some(signal.signal()), None);
    };
    ( $emiter:expr, $signal:ident(), $target:expr, null ) => {
        let signal = $emiter.$signal();
        $emiter.disconnect(
            Some($emiter.id()),
            Some(signal.signal()),
            Some($target.id()),
        );
    };
    ( $emiter:expr, null, $target:expr, null ) => {
        $emiter.disconnect(Some($emiter.id()), None, Some($target.id()));
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! signal {
    ( $object:expr, $name:expr ) => {{
        Signal::new($object.id(), $name.to_string())
    }};
    ( $object:expr, $name:expr, $from_type:expr ) => {{
        let mut signal = Signal::new($object.id(), $name.to_string());
        signal.set_from_type($from_type.to_string());
        signal
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! signals {
    ( $from_type:ident : $($(#[$($attrss:tt)*])* $name:ident();)* ) => {
        $(
            $(#[$($attrss)*])*
            #[allow(dead_code)]
            fn $name(&self) -> Signal {
                signal!(self, stringify!($name), stringify!($from_type))
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
            signal,
            param: None,
        }
    }

    pub fn with_param<T: ToValue + 'static>(signal: Signal, param: T) -> Self {
        Self {
            signal,
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
        object::{ObjectImpl, ObjectSubclass},
        prelude::*,
    };
    use std::rc::Rc;

    #[extends(Object)]
    pub struct Widget {}

    impl ObjectSubclass for Widget {
        const NAME: &'static str = "Widget";
    }

    impl ObjectImpl for Widget {}

    impl Widget {
        signals! {
            Widget:

            /// Signal: action to test.
            action_test();

            /// Signal: action to demo.
            action_demo();

            /// Signal: action with no param.
            action_no_param();
        }

        pub fn new() -> Box<Self> {
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
        ActionHub::initialize();

        let mut widget = Widget::new();
        widget.reg_action();
        let rc = Rc::new(widget);
        rc.emit();

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
