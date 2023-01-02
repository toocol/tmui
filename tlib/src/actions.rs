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
/// impl ActionExt for Widget {}
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
    map: RefCell<HashMap<String, Vec<Box<dyn Fn(Option<Value>)>>>>,
    sender: Sender<(String, Option<Value>)>,
    receiver: Receiver<(String, Option<Value>)>,
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
    /// The thread initialize the `ActionHub` was the `main` thread.
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
            let name = action.0;
            let param = action.1;
            if let Some(actions) = self.map.borrow().get(&name) {
                actions.iter().for_each(|f| f(param.clone()));
            } else {
                println!("Unconnected action: {}", name.to_string());
            }
        }
    }

    pub fn connect_action<F: Fn(Option<Value>) + 'static>(&self, signal: Signal, f: F) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`connect_action()` should only call in the `main` thread.")
            }
        });

        let mut map_ref = self.map.borrow_mut();
        let vec = map_ref.entry(signal.signal().to_string()).or_insert(vec![]);
        vec.push(Box::new(f));
    }

    pub fn activate_action<S: ToString>(&self, name: S, param: Option<Value>) {
        IS_MAIN_THREAD.with(|is_main| {
            if *is_main.borrow() {
                if let Some(actions) = self.map.borrow().get(&name.to_string()) {
                    actions.iter().for_each(|f| f(param.clone()));
                } else {
                    println!("Unconnected action: {}", name.to_string());
                }
            } else {
                self.sender
                    .send((name.to_string(), param))
                    .expect("`ActionHub` send actions from multi thread failed.");
            }
        })
    }
}
pub trait ActionExt: Sized + ObjectOperation {
    fn connect_action<F: Fn(Option<Value>) + 'static>(&self, signal: Signal, f: F) {
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .connect_action(signal, f)
        }
    }

    fn create_action_with_no_param(&self, signal: Signal) -> Action {
        Action::with_no_param(signal)
    }

    fn create_action_with_param<T: ToValue + 'static>(&self, signal: Signal, param: T) -> Action {
        Action::with_param(signal, param)
    }

    fn action_address(&self) -> u64 {
        self.id()
    }
}

/// The struct represents the subject to emit specified actions.
///
/// `Signal` implements the `Send` + `Sync` trait, so it can be transfered between threads safly.
/// The `Siginal` emited in the different threads will be transfer to the `main` thread to process the action.
pub struct Signal {
    signal: String,
}
impl Signal {
    pub fn new(signal: String) -> Self {
        Self { signal }
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
                .activate_action($signal.signal(), None)
        };
    }};
    ( $signal:expr, $x:expr ) => {{
        let value = $x.to_value();
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .activate_action($signal.signal(), Some(value))
        };
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! signal {
    ( $object:expr, $name:expr ) => {{
        let mut signal = String::new();
        signal.push_str($object.action_address().to_string().as_str());
        signal.push('-');
        signal.push_str($name);
        Signal::new(signal)
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
            emit!(self.signal, param)
        } else {
            emit!(self.signal)
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

    impl ActionExt for Widget {}

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

        pub fn reg_action(&self) {
            self.connect_action(self.action_test(), |param| {
                println!("Process action");
                let param = param.unwrap().get::<(i32, String)>();
                assert_eq!(1, param.0);
                assert_eq!("desc", param.1);
            })
        }

        pub fn emit(&self) {
            let param = 1;
            let desc = "desc";
            emit!(self.action_test(), (param, desc));
        }
    }

    #[test]
    fn test_actions() {
        let mut action_hub = ActionHub::new();
        action_hub.initialize();

        let widget = Widget::new();
        widget.reg_action();
        let rc = Rc::new(widget);
        rc.emit();

        let mut join_vec = vec![];
        for _ in 0..5 {
            let signal = rc.action_test();
            join_vec.push(thread::spawn(move || emit!(signal, (1, "desc"))));
        }

        thread::sleep(Duration::from_millis(500));
        action_hub.process_multi_thread_actions();
    }

    #[test]
    fn test_signal() {
        let widget = Widget::new();
        let signal = signal!(&widget, "hello");
        println!("{}", signal)
    }
}
