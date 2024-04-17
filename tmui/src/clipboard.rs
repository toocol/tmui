use clipboard::{ClipboardContext, ClipboardProvider};
use log::error;
use std::sync::Mutex;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClipboardLevel {
    Application,
    Os,
}

pub struct Clipboard {
    text: Mutex<Option<String>>,
    os_clipboard_ctx: ClipboardContext,
}

impl Clipboard {
    pub(crate) fn new() -> Self {
        Self {
            text: Mutex::new(None),
            os_clipboard_ctx: ClipboardContext::new().expect("Get `ClipboardContext` failed."),
        }
    }

    pub fn text(&mut self, level: ClipboardLevel) -> Option<String> {
        match level {
            ClipboardLevel::Application => (*self.text.lock().unwrap()).clone(),
            ClipboardLevel::Os => self.os_clipboard_ctx.get_contents().ok(),
        }
    }

    pub fn set_text<T: ToString>(&mut self, text: T, level: ClipboardLevel) {
        match level {
            ClipboardLevel::Application => {
                let _ = self.text.lock().unwrap().insert(text.to_string());
            }
            ClipboardLevel::Os => {
                let r = self.os_clipboard_ctx.set_contents(text.to_string());
                if r.is_err() {
                    error!("`Clipboard` set text failed, level=Os.")
                }
            }
        }
    }
}
