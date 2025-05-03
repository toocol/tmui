#[cfg(windows_platform)]
use crate::prelude::{RawWindowHandle5, RawWindowHandle6};
use tlib::typedef::WinitWindow;

#[cfg(windows_platform)]
use windows::Win32::Foundation::HWND;

#[cfg(windows_platform)]
pub trait HwndGetter {
    fn hwnd(&self) -> HWND;
}
#[cfg(windows_platform)]
impl HwndGetter for RawWindowHandle5 {
    #[inline]
    fn hwnd(&self) -> HWND {
        match self {
            RawWindowHandle5::Win32(rwh) => HWND(rwh.hwnd as isize),
            _ => unreachable!(),
        }
    }
}
#[cfg(windows_platform)]
impl HwndGetter for RawWindowHandle6 {
    #[inline]
    fn hwnd(&self) -> HWND {
        match self {
            RawWindowHandle6::Win32(rwh) => HWND(rwh.hwnd.into()),
            _ => unreachable!(),
        }
    }
}

#[inline]
#[allow(unused_variables)]
pub(crate) fn set_undecoration_window(window: &WinitWindow) {
    #[cfg(windows_platform)]
    {
        use raw_window_handle::HasRawWindowHandle;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, GWL_STYLE, WINDOW_EX_STYLE, WINDOW_STYLE,
            WS_EX_TOOLWINDOW, WS_OVERLAPPEDWINDOW
        };

        let hwnd = window.raw_window_handle().hwnd();

        unsafe {
            let style = WINDOW_STYLE(GetWindowLongW(hwnd, GWL_STYLE) as u32);
            let ex_style = WINDOW_EX_STYLE(GetWindowLongW(hwnd, GWL_EXSTYLE) as u32);

            SetWindowLongW(hwnd, GWL_STYLE, (style & !WS_OVERLAPPEDWINDOW).0 as i32);
            SetWindowLongW(hwnd, GWL_EXSTYLE, (ex_style | WS_EX_TOOLWINDOW).0 as i32);
        }
    }
}
