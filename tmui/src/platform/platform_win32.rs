use super::{PlatformContext, CODE_PIXELS_UPDATE};
use crate::graphics::bitmap::Bitmap;
use lazy_static::lazy_static;
use std::{
    mem::size_of,
    os::raw::c_void,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
    thread,
};
use windows::{
    core::*,
    w,
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

lazy_static! {
    static ref PLATFORM_WIN32: AtomicPtr<PlatformWin32> = AtomicPtr::new(null_mut());
}

#[cfg(target_os = "windows")]
pub struct PlatformWin32 {
    title: String,
    width: i32,
    height: i32,
    bitmap: Bitmap,

    // The memory area of pixels managed by `PlatformWin32`.
    _pixels: Vec<u8>,

    /// The fileds associated with win32
    _hins: HINSTANCE,
    _wcls: WNDCLASSW,
    hwnd: HWND,
    hbmp: HBITMAP,
}

#[cfg(target_os = "windows")]
impl PlatformContext for PlatformWin32 {
    type Type = PlatformWin32;

    fn new(title: &str, width: i32, height: i32) -> Self {
        let mut platform = unsafe {
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

            let hdc = GetDC(hwnd);
            let hbmp = CreateDIBSection(
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
                _pixels: pixels,
                _hins: hins,
                _wcls: wcls,
                hwnd,
                hbmp,
            }
        };
        PLATFORM_WIN32.store(&mut platform as *mut PlatformWin32, Ordering::SeqCst);
        platform
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

    fn handle_platform_event(&self) {
        unsafe {
            let mut message = MSG::default();
            if PeekMessageW(&mut message, self.hwnd, 0, 0, PM_NOREMOVE).into() {
                if GetMessageW(&mut message, self.hwnd, 0, 0).into() {
                    DispatchMessageW(&message);
                }
            }
        }
    }

    fn send_message(&self, message: super::Message) {
        unsafe {
            SendMessageW(self.hwnd, message.0, WPARAM(0), LPARAM(0));
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT: {}", thread::current().name().unwrap());
                let platform = PLATFORM_WIN32
                    .load(Ordering::SeqCst)
                    .as_ref()
                    .expect("`PLATFORM_WIN32` is None.");

                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(window, &mut ps);
                let hdc_mem = CreateCompatibleDC(hdc);
                SelectObject(hdc_mem, platform.hbmp);

                BitBlt(
                    hdc,
                    0,
                    0,
                    platform.width,
                    platform.height,
                    hdc_mem,
                    0,
                    0,
                    SRCCOPY,
                );

                ReleaseDC(window, hdc_mem);
                EndPaint(window, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY: {}", thread::current().name().unwrap());
                PostQuitMessage(0);
                LRESULT(0)
            }
            CODE_PIXELS_UPDATE => {
                println!("Pixels update.");
                SendMessageW(window, WM_PAINT, WPARAM(0), LPARAM(0));
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
