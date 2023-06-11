
#[cfg(target_os = "unix")]
extern crate beep;
#[cfg(target_os = "unix")]
extern crate dimensioned;
use coreaudio_sys::{AudioServicesPlaySystemSound, kSystemSoundID_UserPreferredAlert};
#[cfg(target_os = "unix")]
use dimensioned::si;
#[cfg(target_os = "windows")]
use winapi::um::winuser::MessageBeep;

use crate::clipboard::Clipboard;
use once_cell::sync::Lazy;

pub(crate) fn system() -> &'static mut System {
    static mut SYSTEM: Lazy<Box<System>> = Lazy::new(|| System::new());
    unsafe { &mut SYSTEM }
}

pub struct System {
    clipboard: Clipboard,
}

impl System {
    #[inline]
    fn new() -> Box<Self> {
        Box::new(Self {
            clipboard: Clipboard::new(),
        })
    }

    #[inline]
    pub fn clipboard<'a>() -> &'a mut Clipboard {
        &mut system().clipboard
    }

    #[inline]
    pub fn beep() {
        #[cfg(target_os = "unix")]
        {
            beep::beep(si::Hertz::new(400)).unwrap();
        }
        #[cfg(target_os = "macos")]
        {
            unsafe { AudioServicesPlaySystemSound(kSystemSoundID_UserPreferredAlert) };
        }
        #[cfg(target_os = "windows")]
        {
            // Default windows system beep tone:
            unsafe { MessageBeep(0) };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

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

    #[test]
    fn test_beep() {
        System::beep();
        std::thread::sleep(Duration::from_millis(500));
    }
}
