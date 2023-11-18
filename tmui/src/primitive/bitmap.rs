#![allow(dead_code)]
#![allow(unused_unsafe)]
use std::{
    ffi::c_void,
    mem::size_of,
    ptr::NonNull,
    slice,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};
use tipc::{
    mem::mem_rw_lock::{MemRwLock, MemRwLockGuard},
    IpcType, Shmem, ShmemConf,
};
use tlib::global::SemanticExt;

const SHMEM_PIXELSS_ADDRESS_SUFFIX: &'static str = "_shpad";

#[repr(C)]
struct _ShmemPixels {
    ptr: AtomicPtr<c_void>,
}

pub(crate) enum Bitmap {
    Direct {
        /// Pixels data.
        pixels: Option<Box<Vec<u8>>>,
        /// Temporarily retain the ownership of the previous pixels data.
        retention: Option<Box<Vec<u8>>>,
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
    },

    Shared {
        /// The raw pointer of shared memory pixels data.
        raw_pointer: Option<NonNull<c_void>>,
        /// The address of shared memory pixels data.
        shmem_pixels: Shmem,
        /// The lock to visit `raw_pointer`
        lock: Arc<MemRwLock>,
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
    },
}

macro_rules! shmem_pixels {
    ( $shmem:expr) => {
        unsafe { ($shmem.as_ptr() as *mut _ShmemPixels).as_mut().unwrap() }
    };
}

impl Bitmap {
    /// Constructer to create the `Bitmap`.
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self::Direct {
            pixels: Some(vec![0u8; (width * height * 4) as usize].boxed()),
            retention: None,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            width,
            height,
            prepared: false,
        }
    }

    #[inline]
    pub fn from_raw_pointer(
        pointer: *mut c_void,
        width: u32,
        height: u32,
        lock: Arc<MemRwLock>,
        ipc_name: &str,
        ipc_type: IpcType,
    ) -> Self {
        let mut address_name = ipc_name.to_string();
        address_name.push_str(SHMEM_PIXELSS_ADDRESS_SUFFIX);

        let shmem_pixels = match ipc_type {
            IpcType::Master => ShmemConf::new()
                .size(size_of::<_ShmemPixels>())
                .os_id(address_name)
                .create()
                .unwrap(),
            IpcType::Slave => ShmemConf::new().os_id(address_name).open().unwrap(),
        };

        shmem_pixels!(shmem_pixels)
            .ptr
            .store(pointer, Ordering::SeqCst);

        let bitmap = Self::Shared {
            raw_pointer: NonNull::new(pointer),
            shmem_pixels,
            lock: lock,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            width,
            height,
            prepared: false,
        };

        bitmap
    }

    #[inline]
    pub fn resize(&mut self, w: u32, h: u32) {
        match self {
            Self::Direct {
                pixels,
                retention,
                total_bytes,
                row_bytes,
                width,
                height,
                prepared,
            } => {
                *retention = pixels.take();
                *pixels = Some(vec![0u8; (w * h * 4) as usize].boxed());
                *total_bytes = (w * h * 4) as usize;
                *row_bytes = (w * 4) as usize;
                *width = w;
                *height = h;
                *prepared = false;
            }
            _ => {}
        }
    }

    #[inline]
    pub fn update_raw_pointer(&mut self, n_raw_pointer: *mut c_void, w: u32, h: u32) {
        match self {
            Self::Shared {
                raw_pointer,
                shmem_pixels,
                total_bytes,
                row_bytes,
                width,
                height,
                prepared,
                ..
            } => {
                *raw_pointer = NonNull::new(n_raw_pointer);
                shmem_pixels!(shmem_pixels)
                    .ptr
                    .store(n_raw_pointer, Ordering::Release);
                *total_bytes = (w * h * 4) as usize;
                *row_bytes = (w * 4) as usize;
                *width = w;
                *height = h;
                *prepared = false;
            }
            _ => {}
        }
    }

    /// Can't used with [`get_pixels_mut()`](Bitmap::get_pixels_mut), [`ipc_write()`](Bitmap::ipc_write)
    /// in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn get_pixels(&self) -> (&[u8], Option<MemRwLockGuard>) {
        match self {
            Self::Shared {
                shmem_pixels,
                lock,
                total_bytes,
                ..
            } => {
                return unsafe {
                    (
                        slice::from_raw_parts(
                            shmem_pixels!(shmem_pixels)
                                .ptr
                                .load(Ordering::Acquire) as *const u8,
                            *total_bytes,
                        ),
                        Some(lock.read()),
                    )
                };
            }
            Self::Direct { pixels, .. } => {
                return (pixels.as_ref().unwrap().as_ref(), None);
            }
        }
    }

    /// Can't used with [`get_pixels()`](Bitmap::get_pixels), [`ipc_write()`](Bitmap::ipc_write)
    /// in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn get_pixels_mut(&mut self) -> (&mut [u8], Option<MemRwLockGuard>) {
        match self {
            Self::Shared {
                shmem_pixels,
                lock,
                total_bytes,
                ..
            } => {
                return unsafe {
                    (
                        slice::from_raw_parts_mut(
                            shmem_pixels!(shmem_pixels)
                                .ptr
                                .load(Ordering::Acquire) as *mut u8,
                            *total_bytes,
                        ),
                        Some(lock.write()),
                    )
                };
            }
            Self::Direct { pixels, .. } => {
                return (pixels.as_mut().unwrap().as_mut(), None);
            }
        }
    }

    #[inline]
    pub fn row_bytes(&self) -> usize {
        match self {
            Self::Direct { row_bytes, .. } => *row_bytes,
            Self::Shared { row_bytes, .. } => *row_bytes,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        match self {
            Self::Direct { width, .. } => *width,
            Self::Shared { width, .. } => *width,
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        match self {
            Self::Direct { height, .. } => *height,
            Self::Shared { height, .. } => *height,
        }
    }

    #[inline]
    pub fn prepared(&mut self) {
        match self {
            Self::Direct { prepared, .. } => *prepared = true,
            Self::Shared { prepared, .. } => *prepared = true,
        }
    }

    #[inline]
    pub fn is_prepared(&self) -> bool {
        match self {
            Self::Direct { prepared, .. } => *prepared,
            Self::Shared { prepared, .. } => *prepared,
        }
    }

    #[inline]
    pub fn release_retention(&mut self) {
        match self {
            Self::Direct { retention, .. } => {
                retention.take();
            }
            Self::Shared { .. } => {}
        }
    }

    /// Can't used with [`get_pixels()`](Bitmap::get_pixels), [`get_pixels_mut()`](Bitmap::get_pixels_mut)
    /// in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn ipc_write(&self) -> Option<MemRwLockGuard> {
        match self {
            Self::Direct { .. } => None,
            Self::Shared { lock, .. } => Some(lock.write()),
        }
    }
}
