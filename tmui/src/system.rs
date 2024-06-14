#[cfg(free_unix)]
extern crate beep;
#[cfg(free_unix)]
extern crate dimensioned;
use std::ptr::addr_of_mut;

#[cfg(windows_platform)]
use winapi::um::winuser::MessageBeep;
#[cfg(macos_platform)]
use coreaudio_sys::{AudioServicesPlaySystemSound, kSystemSoundID_UserPreferredAlert};

use crate::clipboard::Clipboard;
use once_cell::sync::Lazy;

pub(crate) fn system() -> &'static mut System {
    static mut SYSTEM: Lazy<Box<System>> = Lazy::new(System::new);
    unsafe { addr_of_mut!(SYSTEM).as_mut().unwrap().as_mut() }
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
        #[cfg(free_unix)]
        {
            beep::beep(400).unwrap();
        }
        #[cfg(macos_platform)]
        {
            unsafe { AudioServicesPlaySystemSound(kSystemSoundID_UserPreferredAlert) };
        }
        #[cfg(windows_platform)]
        {
            // Default windows system beep tone:
            unsafe { MessageBeep(0) };
        }
    }
}

// There was some problem when running test on github workflow on platform ubuntu.
#[cfg(test)]
#[cfg(not(free_unix))]
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

         {
            clipboard.set_text(str, ClipboardLevel::Os);
            assert_eq!(str, clipboard.text(ClipboardLevel::Os).as_ref().unwrap());
        }
    }

    #[test]
    fn test_beep() {
        System::beep();
        std::thread::sleep(Duration::from_millis(500));
    }
}
