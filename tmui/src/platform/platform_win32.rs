use super::PlatformContext;
use crate::graphics::bitmap::Bitmap;
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo};
use std::{mem::size_of, os::raw::c_void};
use windows::{
    w,
    Win32::{
        Foundation::{HANDLE, HINSTANCE, HWND},
        Graphics::Gdi::{
            CreateDIBSection, DeleteObject, GetDC, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
            COLOR_WINDOW, DIB_RGB_COLORS, HBITMAP, HBRUSH,
        },
        UI::WindowsAndMessaging::{
            CloseWindow, CreateWindowExW, RegisterClassExW, CS_HREDRAW, CS_VREDRAW, HCURSOR, HICON,
            WNDCLASSEXW, WNDPROC, WS_CAPTION, WS_EX_APPWINDOW, WS_EX_LEFT, WS_SIZEBOX, WS_VISIBLE,
        },
    },
};

#[cfg(target_os = "windows")]
pub struct PlatformWin32 {
    title: String,
    width: i32,
    height: i32,
    bitmap: Bitmap,
    image_info: ImageInfo,

    // The memory area of pixels managed by `PlatformWin32`.
    _pixels: Vec<u8>,

    /// The fileds associated with win32
    _hins: HINSTANCE,
    _wcls: WNDCLASSEXW,
    hwnd: HWND,
    hbmp: HBITMAP,
}

impl PlatformContext for PlatformWin32 {
    type Type = PlatformWin32;

    fn new(title: &str, width: i32, height: i32) -> Self {
        let hins = HINSTANCE::default();
        let mut wcls = WNDCLASSEXW::default();
        let mut brush = HBRUSH::default();
        brush.0 = COLOR_WINDOW.0 as isize + 1;
        wcls.cbClsExtra = 0;
        wcls.cbWndExtra = 0;
        wcls.hbrBackground = brush;
        wcls.hCursor = HCURSOR::default();
        wcls.hIcon = HICON::default();
        wcls.hInstance = hins;
        wcls.lpfnWndProc = WNDPROC::default();
        wcls.lpszClassName = w!("TmuiMainClass");
        wcls.lpszMenuName = w!("");
        wcls.style = CS_HREDRAW | CS_VREDRAW;
        unsafe { RegisterClassExW(&wcls as *const WNDCLASSEXW) };

        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW | WS_EX_LEFT,
                w!("TmuiMainClass"),
                w!("Tmui"),
                WS_VISIBLE | WS_CAPTION | WS_SIZEBOX,
                0,
                0,
                width,
                height,
                None,
                None,
                hins,
                None,
            )
        };

        let mut pixels = vec![0; (width * height * 4) as usize];
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
            title: title.to_string(),
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
            _hins: hins,
            _wcls: wcls,
            hwnd,
            hbmp,
        }
    }

    fn title(&self) -> &str {
        &self.title
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
