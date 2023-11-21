#![allow(unused_unsafe)]
use std::{
    collections::VecDeque,
    ffi::c_void,
    mem::size_of,
    ptr::NonNull,
    slice,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};
use tipc::{
    mem::mem_rw_lock::{MemRwLock, MemRwLockGuard},
    IpcType, Shmem, ShmemConf,
};
use tlib::global::SemanticExt;

const SHMEM_BITMAP_INFO_SUFFIX: &'static str = "_shbif";

#[repr(C)]
struct _ShmemInfo {
    prepared: AtomicBool,

    master_release_idx: AtomicUsize,
    slave_release_idx: AtomicUsize,
}

macro_rules! shmem_info {
    ( $shmem:expr) => {
        unsafe { ($shmem.as_ptr() as *mut _ShmemInfo).as_mut().unwrap() }
    };
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
        ty: IpcType,
        /// The raw pointer of shared memory pixels data.
        raw_pointer: Option<NonNull<c_void>>,
        /// Temporarily retain the previous pixels datas.
        rentention: VecDeque<Shmem>,
        /// The address of shared memory pixels data.
        shmem_info: Shmem,
        /// The lock to visit `raw_pointer`.
        lock: Arc<MemRwLock>,
        /// The length of the pixels.
        total_bytes: usize,
        /// Bytes number of a row.
        row_bytes: usize,
        /// The width of `Bitmap`.
        width: u32,
        /// The height of `Bitmap`.
        height: u32,
    },
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
        address_name.push_str(SHMEM_BITMAP_INFO_SUFFIX);

        let shmem_info = match ipc_type {
            IpcType::Master => ShmemConf::new()
                .size(size_of::<_ShmemInfo>())
                .os_id(address_name)
                .create()
                .unwrap(),
            IpcType::Slave => ShmemConf::new().os_id(address_name).open().unwrap(),
        };

        Self::Shared {
            ty: ipc_type,
            raw_pointer: NonNull::new(pointer),
            rentention: VecDeque::new(),
            shmem_info,
            lock: lock,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            width,
            height,
        }
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
    pub fn update_raw_pointer(&mut self, n_raw_pointer: *mut c_void, old_shmem: Shmem, w: u32, h: u32) {
        match self {
            Self::Shared {
                ty,
                raw_pointer,
                rentention,
                shmem_info,
                total_bytes,
                row_bytes,
                width,
                height,
                ..
            } => {
                *raw_pointer = NonNull::new(n_raw_pointer);
                if *ty == IpcType::Master {
                    rentention.push_back(old_shmem)
                }
                let shmem_info = shmem_info!(shmem_info);
                shmem_info.prepared.store(false, Ordering::Release);
                *total_bytes = (w * h * 4) as usize;
                *row_bytes = (w * 4) as usize;
                *width = w;
                *height = h;
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
                raw_pointer,
                lock,
                total_bytes,
                ..
            } => {
                return unsafe {
                    (
                        slice::from_raw_parts(
                            raw_pointer.as_ref().unwrap().as_ptr() as *const u8,
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
                raw_pointer,
                lock,
                total_bytes,
                ..
            } => {
                return unsafe {
                    (
                        slice::from_raw_parts_mut(
                            raw_pointer.as_mut().unwrap().as_ptr() as *mut u8,
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
            Self::Shared {
                shmem_info,
                ..
            } => shmem_info!(shmem_info)
                .prepared
                .store(true, Ordering::Release),
        }
    }

    #[inline]
    pub fn is_prepared(&self) -> bool {
        match self {
            Self::Direct { prepared, .. } => *prepared,
            Self::Shared {
                shmem_info,
                ..
            } => shmem_info!(shmem_info).prepared.load(Ordering::Acquire),
        }
    }

    #[inline]
    pub fn release_retention(&mut self) {
        match self {
            Self::Direct { retention, .. } => {
                retention.take();
            }
            Self::Shared {
                ty,
                rentention,
                shmem_info,
                ..
            } => {
                let shmem_info = shmem_info!(shmem_info);

                match ty {
                    IpcType::Master => shmem_info.master_release_idx.fetch_add(1, Ordering::Release),
                    IpcType::Slave => shmem_info.slave_release_idx.fetch_add(1, Ordering::Release),
                };
                
                if *ty == IpcType::Slave {
                    return
                }

                shmem_info
                    .master_release_idx
                    .fetch_update(Ordering::Release, Ordering::Acquire, |master_idx| {
                        let mut cnt = 0;
                        shmem_info.slave_release_idx.fetch_update(
                            Ordering::Release,
                            Ordering::Acquire,
                            |slave_idx| {
                                for _ in 0..slave_idx.min(master_idx) {
                                    rentention.pop_front();
                                    cnt += 1;
                                }
                                Some(slave_idx - cnt)
                            }
                        ).expect("Shared bitmap release retention failed, cause by `slave_release_idx` fetch_update()");
                        Some(master_idx - cnt)
                    })
                    .expect("Shared bitmap release retention failed, cause by `master_release_idx` fetch_update()");
            }
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
