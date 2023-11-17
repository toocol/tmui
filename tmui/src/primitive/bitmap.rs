use std::{ffi::c_void, ptr::NonNull, slice, sync::Arc};
use tipc::mem::mem_rw_lock::{MemRwLock, MemRwLockGuard};
use tlib::global::SemanticExt;

pub struct Bitmap {
    /// Pixels data.
    pixels: Option<Box<Vec<u8>>>,
    /// Temporarily retain the ownership of the previous pixels data.
    retention: Option<Box<Vec<u8>>>,

    /// The raw pointer of pixels data.
    raw_pointer: Option<NonNull<c_void>>,
    /// The lock to visit `raw_pointer`
    lock: Option<Arc<MemRwLock>>,

    /// The length of the pixels.
    total_bytes: usize,
    /// Bytes number of a row.
    row_bytes: usize,
    /// The width of `Bitmap`.
    width: u32,
    /// The height of `Bitmap`.
    height: u32,
    /// Is this bitmap rendered.
    prepared: bool,
}

impl Bitmap {
    /// Constructer to create the `Bitmap`.
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: Some(vec![0u8; (width * height * 4) as usize].boxed()),
            retention: None,
            raw_pointer: None,
            lock: None,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            width,
            height,
            prepared: false,
        }
    }

    #[inline]
    pub fn from_raw_pointer(pointer: *mut c_void, width: u32, height: u32) -> Self {
        Self {
            pixels: None,
            retention: None,
            raw_pointer: NonNull::new(pointer),
            lock: None,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            width,
            height,
            prepared: false,
        }
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(_) = self.pixels.as_ref() {
            self.retention = self.pixels.take();
            self.pixels = Some(vec![0u8; (width * height * 4) as usize].boxed());
            self.total_bytes = (width * height * 4) as usize;
            self.row_bytes = (width * 4) as usize;
            self.width = width;
            self.height = height;
            self.prepared = false;
        }
    }

    #[inline]
    pub fn update_raw_pointer(&mut self, raw_pointer: *mut c_void, width: u32, height: u32) {
        if let Some(_) = self.raw_pointer {
            self.raw_pointer = NonNull::new(raw_pointer);
            self.total_bytes = (width * height * 4) as usize;
            self.row_bytes = (width * 4) as usize;
            self.width = width;
            self.height = height;
            self.prepared = false;
        }
    }

    #[inline]
    pub fn get_pixels(&self) -> (&[u8], Option<MemRwLockGuard>) {
        if let Some(pixels) = self.pixels.as_ref() {
            return (pixels.as_ref(), None);
        }
        if let Some(ptr) = self.raw_pointer {
            let guard = if let Some(lock) = self.lock.as_ref() {
                Some(lock.read())
            } else {
                None
            };

            return unsafe {
                (
                    slice::from_raw_parts(ptr.as_ptr() as *const u8, self.total_bytes),
                    guard,
                )
            };
        }
        unreachable!()
    }

    #[inline]
    pub fn get_pixels_mut(&mut self) -> (&mut [u8], Option<MemRwLockGuard>) {
        if let Some(pixels) = self.pixels.as_mut() {
            return (pixels.as_mut(), None);
        }
        if let Some(ptr) = self.raw_pointer {
            let guard = if let Some(lock) = self.lock.as_ref() {
                Some(lock.write())
            } else {
                None
            };

            return unsafe {
                (
                    slice::from_raw_parts_mut(ptr.as_ptr() as *mut u8, self.total_bytes),
                    guard,
                )
            };
        }
        unreachable!()
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

    #[inline]
    pub fn prepared(&mut self) {
        self.prepared = true;
    }

    #[inline]
    pub fn is_prepared(&self) -> bool {
        self.prepared
    }

    #[inline]
    pub fn release_retention(&mut self) {
        self.retention.take();
    }
}
