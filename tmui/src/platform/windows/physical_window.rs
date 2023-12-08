#![cfg(windows_platform)]
use std::{ffi::c_void, mem::size_of, sync::Arc};
use tipc::RwLock;
use tlib::winit::window::Window;
use windows::Win32::{Foundation::*, Graphics::Gdi::*};

use crate::primitive::bitmap::Bitmap;
pub struct PhysicalWindow {
    hwnd: HWND,
    bitmap: Arc<RwLock<Bitmap>>,
}

impl PhysicalWindow {
    #[inline]
    pub fn new(hwnd: HWND, bitmap: Arc<RwLock<Bitmap>>) -> Self {
        Self { hwnd, bitmap }
    }

    /// Request to redraw the window.
    pub fn request_redraw(&self, _window: &Window) {
        unsafe {
            InvalidateRect(self.hwnd, None, false);
        }
    }

    /// Redraw the window.
    pub fn redraw(&self) {
        let bitmap_guard = self.bitmap.read();
        if !bitmap_guard.is_prepared() {
            return;
        }

        unsafe {
            let hwnd = self.hwnd;

            let width = bitmap_guard.width();
            let height = bitmap_guard.height();

            let mut bmi = BITMAPINFO::default();
            bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = width as i32;
            // Drawing start at top-left.
            bmi.bmiHeader.biHeight = -(height as i32);
            bmi.bmiHeader.biPlanes = 1;
            bmi.bmiHeader.biBitCount = 32;
            bmi.bmiHeader.biCompression = BI_RGB;

            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            let _guard = bitmap_guard.ipc_read();
            let pixels = bitmap_guard.get_pixels();
            StretchDIBits(
                hdc,
                0,
                0,
                width as i32,
                height as i32,
                0,
                0,
                width as i32,
                height as i32,
                Some(pixels.as_ptr() as *const c_void),
                &bmi,
                DIB_RGB_COLORS,
                SRCCOPY,
            );

            EndPaint(self.hwnd, &ps);

            ReleaseDC(hwnd, hdc);
        }
    }
}
