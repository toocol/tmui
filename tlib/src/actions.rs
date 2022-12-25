use crate::prelude::*;
use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

lazy_static! {
    pub static ref ACTION_HUB: Mutex<ActionHub> = Mutex::new(ActionHub::new());
}

/// ActionHub hold all of the registered actions
pub struct ActionHub {
    map: RefCell<HashMap<String, Vec<Box<dyn Fn(Option<Value>) + Send + Sync>>>>,
}
impl ActionHub {
    pub fn new() -> Self {
        Self {
            map: RefCell::new(HashMap::new()),
        }
    }

    pub fn connect_action<S: ToString, F: Fn(Option<Value>) + Send + Sync + 'static>(
        &self,
        name: S,
        f: F,
    ) {
        let mut map_ref = self.map.borrow_mut();
        let vec = map_ref.entry(name.to_string()).or_insert(vec![]);
        vec.push(Box::new(f));
    }

    pub fn activate_action<S: ToString>(&self, name: S, param: Option<Value>) {
        if let Some(actions) = self.map.borrow().get(&name.to_string()) {
            actions.iter().for_each(|f| f(param.clone()));
        } else {
            println!("Unconnected action: {}", name.to_string());
        }
    }
}
pub trait ActionHubExt {
    fn connect_action<S: ToString, F: Fn(Option<Value>) + Send + Sync + 'static>(
        &self,
        name: S,
        f: F,
    ) {
        if let Ok(hub) = ACTION_HUB.lock() {
            hub.connect_action(name, f)
        }
    }
}

///////////////////////////////////// Macros
#[allow(unused_macros)]
#[macro_export]
macro_rules! emit {
    () => {};
    ( $name:expr, $x:expr ) => {{
        if let Ok(action_hub) = crate::actions::ACTION_HUB.lock() {
            let value = $x.to_value();
            println!("Emit action {}: {:?}", $name, value);
            action_hub.activate_action($name, Some(value));
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
    fn test_macro() {
        let widget = Widget::new();
        widget.reg_action();
        widget.emit()
    }
}
