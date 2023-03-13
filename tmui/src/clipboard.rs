use clipboard::{ClipboardContext, ClipboardProvider};
use log::error;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClipboardLevel {
    Application,
    Os,
}

pub struct Clipboard {
    text: Option<String>,
    os_clipboard_ctx: ClipboardContext,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            text: None,
            os_clipboard_ctx: ClipboardContext::new().expect("Get `ClipboardContext` failed."),
        }
    }

    pub fn text(&mut self, level: ClipboardLevel) -> Option<String> {
        match level {
            ClipboardLevel::Application => self.text.clone(),
            ClipboardLevel::Os => self
                .os_clipboard_ctx
                .get_contents()
                .map_or(None, |text| Some(text)),
        }
    }

    pub fn set_text<T: ToString>(&mut self, text: T, level: ClipboardLevel) {
        match level {
            ClipboardLevel::Application => self.text = Some(text.to_string()),
            ClipboardLevel::Os => {
                let r = self.os_clipboard_ctx
                    .set_contents(text.to_string());
                if r.is_err() {
                    error!("`Clipboard` set text failed, level=Os.")
                }
            }
        }
    }
}
