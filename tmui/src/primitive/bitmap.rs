use std::{os::raw::c_void, ptr::NonNull, slice};

/// `Bitmap` holding the raw pointer of specific memory area created by specific platform context.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bitmap {
    /// The raw pointer of the origin memory area.
    raw_pointer: Option<NonNull<c_void>>,
    /// The length of the raw pointer.
    total_bytes: usize,
    /// Bytes number of a row.
    row_bytes: usize,
    /// The width of `Bitmap`.
    width: u32,
    /// The height of `Bitmap`.
    height: u32,
    /// This bitmap has been rendered and is ready to display.
    prepared: bool,
}
unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}

impl Bitmap {
    /// Constructer to create the `Bitmap`.
    #[inline]
    pub fn new(pointer: *mut c_void, width: u32, height: u32) -> Self {
        Self {
            raw_pointer: NonNull::new(pointer),
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            width,
            height,
            prepared: false,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut c_void {
        self.raw_pointer
            .as_ref()
            .expect("`The pointer of `Bitmap` was None.")
            .as_ptr()
    }

    #[inline]
    pub fn get_pixels(&self) -> &'static mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.as_ptr() as *mut u8, self.total_bytes) }
    }

    #[inline]
    pub fn row_bytes(&self) -> usize {
        self.row_bytes
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}
