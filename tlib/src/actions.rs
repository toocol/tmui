use crate::prelude::*;
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    collections::HashMap,
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
/// Initialize the `ActionHub`, the instance should be managed by the caller.  
///
/// The thread initialize the `ActionHub` was the `main` thread.
///
/// This function should only call once.
pub fn initialize_action_hub(action_hub: &mut ActionHub) {
    INIT.call_once(|| {
        IS_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);
        ACTION_HUB.store(
            action_hub as *mut ActionHub,
            std::sync::atomic::Ordering::SeqCst,
        );
    });
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

    pub fn process_multi_thread_actions(&self) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`process_multi_thread_actions()` should only call in the `main` thread.")
            }
        });

        while let Ok(action) = self.receiver.try_recv() {
            let name = action.0;
            let param = action.1;
            if let Some(actions) = self.map.borrow().get(&name.to_string()) {
                actions.iter().for_each(|f| f(param.clone()));
            } else {
                println!("Unconnected action: {}", name.to_string());
            }
        }
    }

    pub fn connect_action<S: ToString, F: Fn(Option<Value>) + 'static>(&self, name: S, f: F) {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("`connect_action()` should only call in the `main` thread.")
            }
        });

        let mut map_ref = self.map.borrow_mut();
        let vec = map_ref.entry(name.to_string()).or_insert(vec![]);
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
pub trait ActionHubExt {
    fn connect_action<S: ToString, F: Fn(Option<Value>) + 'static>(&self, name: S, f: F) {
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .connect_action(name, f)
        }
    }
}

///////////////////////////////////// Macros
#[allow(unused_macros)]
#[macro_export]
macro_rules! emit {
    ( $name:expr ) => {{
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .activate_action($name, None)
        };
    }};
    ( $name:expr, $x:expr ) => {{
        let value = $x.to_value();
        let action_hub = ACTION_HUB.load(std::sync::atomic::Ordering::SeqCst);
        unsafe {
            action_hub
                .as_ref()
                .expect("`ActionHub` was not initialized, or already dead.")
                .activate_action($name, Some(value))
        };
    }};
}

/// Struct represents an action which can emit specified action.
pub struct Action {
    name: String,
    param: Option<Box<dyn ToValue>>,
}
impl Action {
    pub fn new<T: ToValue + 'static>(name: String, param: Option<T>) -> Self {
        let mut param = param;
        let param: Option<Box<dyn ToValue>> = if param.is_none() {
            None
        } else {
            Some(Box::new(param.take().unwrap()))
        };
        Self { name, param }
    }

    pub fn emit(&self) {
        if let Some(param) = self.param.as_ref() {
            emit!(&self.name, param)
        } else {
            emit!(&self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::prelude::*;

    use super::{initialize_action_hub, ActionHub};

    pub struct Widget;
    impl ActionHubExt for Widget {}
    impl Widget {
        pub const ACTION: &'static str = "action-test";

        pub fn new() -> Self {
            Self {}
        }

        pub fn reg_action(&self) {
            self.connect_action(Self::ACTION, |param| {
                println!("Process action");
                let param = param.unwrap().get::<(i32, String)>();
                assert_eq!(1, param.0);
                assert_eq!("desc", param.1);
            })
        }

        pub fn emit(&self) {
            let param = 1;
            let desc = "desc";
            emit!(Self::ACTION, (param, desc));
        }
    }

    #[test]
    fn test_actions() {
        let mut action_hub = ActionHub::new();
        initialize_action_hub(&mut action_hub);

        let widget = Widget::new();
        widget.reg_action();
        widget.emit();

        let mut join_vec = vec![];
        for _ in 0..5 {
            join_vec.push(thread::spawn(|| {
                let widget = Widget::new();
                widget.emit();
            }));
        }

        thread::sleep(Duration::from_millis(500));
        action_hub.process_multi_thread_actions();

        for h in join_vec {
            h.join().unwrap()
        }
    }
}
