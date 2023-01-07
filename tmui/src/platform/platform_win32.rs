use super::PlatformContext;
use crate::graphics::bitmap::Bitmap;
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo};
use std::{mem::size_of, os::raw::c_void};
use windows::{
    w,
    Win32::{
        Foundation::{HANDLE, HWND},
        Graphics::Gdi::{
            CreateDIBSection, DeleteObject, GetDC, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
            DIB_RGB_COLORS, HBITMAP,
        },
        UI::WindowsAndMessaging::{
            CloseWindow, CreateWindowExW, WS_EX_APPWINDOW, WS_EX_LEFT, WS_VISIBLE, WS_CAPTION, WS_SIZEBOX,
        },
    },
};

#[cfg(target_os = "windows")]
pub struct PlatformWin32 {
    width: i32,
    height: i32,
    bitmap: Bitmap,
    image_info: ImageInfo,

    // The memory area of pixels managed by `PlatformWin32`.
    _pixels: Vec<u8>,

    /// The fileds  associated with win32
    hwnd: HWND,
    hbmp: HBITMAP,
}

impl PlatformContext for PlatformWin32 {
    type Type = PlatformWin32;

    fn new(width: i32, height: i32) -> Self {
        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW | WS_EX_LEFT,
                w!("#32769"),
                w!("Tmui"),
                WS_VISIBLE | WS_CAPTION | WS_SIZEBOX,
                0,
                0,
                width,
                height,
                None,
                None,
                None,
                None,
            )
        };

        let mut pixels = vec![0; (width * height) as usize];
        let bitmap = Bitmap::new(&mut pixels[..] as *mut [u8] as *mut c_void, width, height);

        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = width;
        bmi.bmiHeader.biHeight = -height;
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB;
        bmi.bmiHeader.biSizeImage = 0;

        let hbmp;
        unsafe {
            let hdc = GetDC(hwnd);
            hbmp = CreateDIBSection(
                hdc,
                &bmi as *const BITMAPINFO,
                DIB_RGB_COLORS,
                bitmap.as_ptr() as *mut *mut c_void,
                HANDLE::default(),
                0,
            )
            .expect("Create `HBITMAP` failed.");
            ReleaseDC(hwnd, hdc);
        }

        Self {
            width,
            height,
            bitmap,
            image_info: ImageInfo::new(
                (width, height),
                ColorType::BGRA8888,
                AlphaType::Premul,
                ColorSpace::new_srgb(),
            ),
            _pixels: pixels,
            hwnd,
            hbmp,
        }
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.bitmap.set_raw_bytes((width * height) as usize);
        todo!()
    }

    fn close(&self) {
        unsafe {
            DeleteObject(self.hbmp);
            CloseWindow(self.hwnd);
        }
    }

    fn context_bitmap(&self) -> &Bitmap {
        &self.bitmap
    }

    fn image_info(&self) -> &ImageInfo {
        &self.image_info
    }
}
