use super::PlatformContext;
use crate::graphics::bitmap::Bitmap;
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo};
use std::{mem::size_of, os::raw::c_void, thread};
use windows::{
    core::*,
    w,
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
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
    _wcls: WNDCLASSW,
    hwnd: HWND,
    hbmp: HBITMAP,
}

impl PlatformContext for PlatformWin32 {
    type Type = PlatformWin32;

    fn new(title: &str, width: i32, height: i32) -> Self {
        unsafe {
            let hins = GetModuleHandleW(None).unwrap();
            assert!(hins.0 != 0);

            let wcls = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: hins,
                lpszClassName: w!("TmuiMainClass"),

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wcls);
            assert!(atom != 0);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("TmuiMainClass"),
                PCWSTR(HSTRING::from(title).as_ptr()),
                WS_VISIBLE | WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width,
                height,
                None,
                None,
                hins,
                None,
            );

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

    fn handle_platform_event(&self) {
        unsafe {
            let mut message = MSG::default();
            if GetMessageW(&mut message, self.hwnd, 0, 0).into() {
                DispatchMessageW(&message);
            }
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT: {}", thread::current().name().unwrap());
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY: {}", thread::current().name().unwrap());
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
