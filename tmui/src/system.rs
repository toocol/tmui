use crate::clipboard::Clipboard;
use std::{
    cell::RefCell,
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Once,
    },
};

static INIT: Once = Once::new();
static SYSTEM: AtomicPtr<System> = AtomicPtr::new(null_mut());
thread_local! {static IS_MAIN_THREAD: RefCell<bool>  = RefCell::new(false)}

pub struct System {
    clipboard: Clipboard,
}

impl System {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            clipboard: Clipboard::new(),
        })
    }

    pub fn initialize(self: &mut Box<Self>) {
        INIT.call_once(|| {
            IS_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);
            SYSTEM.store(self.as_mut(), Ordering::SeqCst);
        })
    }

    pub fn clipboard<'a>() -> &'a mut Clipboard {
        IS_MAIN_THREAD.with(|is_main| {
            if !*is_main.borrow() {
                panic!("function `clipboard()` of `System` can only call in main thread.")
            }
        });
        let system = unsafe {
            SYSTEM
                .load(Ordering::SeqCst)
                .as_mut()
                .expect("`System` is not initialized.")
        };
        &mut system.clipboard
    }
}

#[cfg(test)]
mod tests {
    use crate::clipboard::ClipboardLevel;

    use super::System;

    #[test]
    fn test_clipboard() {
        let mut system = System::new();
        system.initialize();
        let clipboard = System::clipboard();

        let str = "Hello World";
        clipboard.set_text(str, ClipboardLevel::Application);
        assert_eq!(
            str,
            clipboard
                .text(ClipboardLevel::Application)
                .as_ref()
                .unwrap()
        );

        clipboard.set_text(str, ClipboardLevel::Os);
        assert_eq!(str, clipboard.text(ClipboardLevel::Os).as_ref().unwrap());
    }
}
