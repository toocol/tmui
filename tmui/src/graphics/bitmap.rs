use std::{ptr::NonNull, slice, os::raw::c_void};

/// `Bitmap` holding the raw pointer of specific memory area created by specific platform context.
#[derive(Default, Debug)]
pub struct Bitmap {
    /// The raw pointer of the origin memory area.
    raw_pointer: Option<NonNull<c_void>>,
    /// The length of the raw pointer.
    raw_bytes: usize,
}

impl Bitmap {
    /// Constructer to create the `Bitmap`.
    pub fn new(pointer: *mut c_void, width: i32, height: i32) -> Self {
        Self {
            raw_pointer: NonNull::new(pointer),
            raw_bytes: (width * height * 4) as usize,
        }
    }

    pub fn set_raw_pointer(&mut self, pointer: *mut c_void) {
        self.raw_pointer = NonNull::new(pointer)
    }

    pub fn set_raw_bytes(&mut self, raw_bytes: usize) {
        self.raw_bytes = raw_bytes
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.raw_pointer
            .as_ref()
            .expect("`The pointer of `Bitmap` was None.")
            .as_ptr()
    }

    pub fn get_pixels(&self) -> &'static mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.as_ptr() as *mut u8, self.raw_bytes) }
    }

    pub fn raw_bytes(&self) -> usize {
        self.raw_bytes
    }
}
