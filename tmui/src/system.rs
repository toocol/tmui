use once_cell::sync::Lazy;
use crate::clipboard::Clipboard;

pub fn system() -> &'static mut System {
    static mut SYSTEM: Lazy<Box<System>> = Lazy::new(|| System::new());
    unsafe { &mut SYSTEM }
}

pub struct System {
    clipboard: Clipboard,
}

impl System {
    fn new() -> Box<Self> {
        Box::new(Self {
            clipboard: Clipboard::new(),
        })
    }

    #[inline]
    pub fn clipboard<'a>() -> &'a mut Clipboard {
        &mut system().clipboard
    }
}

#[cfg(test)]
mod tests {
    use crate::clipboard::ClipboardLevel;

    use super::System;

    #[test]
    fn test_clipboard() {
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
