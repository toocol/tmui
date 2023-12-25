#![allow(unused_unsafe)]
use std::{
    collections::VecDeque,
    ffi::c_void,
    mem::size_of,
    ptr::NonNull,
    slice,
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering},
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

    master_width: AtomicU32,
    master_height: AtomicU32,

    slave_width: AtomicU32,
    slave_height: AtomicU32,
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
        /// Only used on platform `macos`
        resized: bool,
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
        /// Only used on platform `macos`
        resized: bool,
    },
}

unsafe impl Send for Bitmap {}

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
            resized: false,
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

        {
            let shmem_info = shmem_info!(shmem_info);
            match ipc_type {
                IpcType::Master => {
                    shmem_info.master_width.store(width, Ordering::Release);
                    shmem_info.master_height.store(height, Ordering::Release);
                }
                IpcType::Slave => {
                    shmem_info.slave_width.store(width, Ordering::Release);
                    shmem_info.slave_height.store(height, Ordering::Release);
                }
            }
        }

        Self::Shared {
            ty: ipc_type,
            raw_pointer: NonNull::new(pointer),
            rentention: VecDeque::new(),
            shmem_info,
            lock: lock,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            resized: false,
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
                resized,
            } => {
                *retention = pixels.take();
                *pixels = Some(vec![0u8; (w * h * 4) as usize].boxed());
                *total_bytes = (w * h * 4) as usize;
                *row_bytes = (w * 4) as usize;
                *width = w;
                *height = h;
                *prepared = false;
                *resized = true;
            }
            _ => {}
        }
    }

    #[inline]
    pub fn update_raw_pointer(
        &mut self,
        n_raw_pointer: *mut c_void,
        old_shmem: Shmem,
        w: u32,
        h: u32,
    ) {
        match self {
            Self::Shared {
                ty,
                raw_pointer,
                rentention,
                shmem_info,
                total_bytes,
                row_bytes,
                resized,
                ..
            } => {
                *raw_pointer = NonNull::new(n_raw_pointer);
                rentention.push_back(old_shmem);
                let shmem_info = shmem_info!(shmem_info);
                shmem_info.prepared.store(false, Ordering::Release);
                *total_bytes = (w * h * 4) as usize;
                *row_bytes = (w * 4) as usize;
                *resized = true;

                if *ty == IpcType::Master {
                    shmem_info.master_width.store(w, Ordering::Release);
                    shmem_info.master_height.store(h, Ordering::Release);
                } else {
                    shmem_info.slave_width.store(w, Ordering::Release);
                    shmem_info.slave_height.store(h, Ordering::Release);
                }
            }
            _ => {}
        }
    }

    /// Can't used with [`get_pixels_mut()`](Bitmap::get_pixels_mut), [`ipc_write()`](Bitmap::ipc_write)
    /// in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn get_pixels(&self) -> &[u8] {
        match self {
            Self::Shared {
                raw_pointer,
                total_bytes,
                ..
            } => {
                return unsafe {
                    slice::from_raw_parts(
                        raw_pointer.as_ref().unwrap().as_ptr() as *const u8,
                        *total_bytes,
                    )
                };
            }
            Self::Direct { pixels, .. } => {
                return pixels.as_ref().unwrap().as_ref();
            }
        }
    }

    /// Can't used with [`get_pixels()`](Bitmap::get_pixels), [`ipc_write()`](Bitmap::ipc_write)
    /// in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn get_pixels_mut(&mut self) -> &mut [u8] {
        match self {
            Self::Shared {
                raw_pointer,
                total_bytes,
                ..
            } => {
                return unsafe {
                    slice::from_raw_parts_mut(
                        raw_pointer.as_mut().unwrap().as_ptr() as *mut u8,
                        *total_bytes,
                    )
                };
            }
            Self::Direct { pixels, .. } => {
                return pixels.as_mut().unwrap().as_mut();
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
            Self::Shared { ty, shmem_info, .. } => {
                if *ty == IpcType::Master {
                    shmem_info!(shmem_info).master_width.load(Ordering::Acquire)
                } else {
                    shmem_info!(shmem_info).slave_width.load(Ordering::Acquire)
                }
            }
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        match self {
            Self::Direct { height, .. } => *height,
            Self::Shared { ty, shmem_info, .. } => {
                if *ty == IpcType::Master {
                    shmem_info!(shmem_info)
                        .master_height
                        .load(Ordering::Acquire)
                } else {
                    shmem_info!(shmem_info).slave_height.load(Ordering::Acquire)
                }
            }
        }
    }

    #[inline]
    pub fn prepared(&mut self) {
        match self {
            Self::Direct { prepared, .. } => *prepared = true,
            Self::Shared { shmem_info, .. } => shmem_info!(shmem_info)
                .prepared
                .store(true, Ordering::Release),
        }
    }

    #[inline]
    pub fn is_prepared(&self) -> bool {
        match self {
            Self::Direct { prepared, .. } => *prepared,
            Self::Shared { shmem_info, .. } => {
                shmem_info!(shmem_info).prepared.load(Ordering::Acquire)
            }
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
                    IpcType::Master => {
                        shmem_info
                            .master_release_idx
                            .fetch_add(1, Ordering::Release);

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
                    IpcType::Slave => {
                        rentention.pop_front();

                        shmem_info.slave_release_idx.fetch_add(1, Ordering::Release);
                    }
                };
            }
        }
    }

    /// Can't used with [`ipc_write()`](Bitmap::ipc_write) in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn ipc_read(&self) -> Option<MemRwLockGuard> {
        match self {
            Self::Direct { .. } => None,
            Self::Shared { lock, .. } => Some(lock.read()),
        }
    }

    /// Can't used with [`ipc_read()`](Bitmap::ipc_read) in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn ipc_write(&self) -> Option<MemRwLockGuard> {
        match self {
            Self::Direct { .. } => None,
            Self::Shared { lock, .. } => Some(lock.write()),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_resized(&self) -> bool {
        match self {
            Self::Direct { resized, .. } => *resized,
            Self::Shared { resized, .. } => *resized,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn reset_resized(&mut self) {
        match self {
            Self::Direct { resized, .. } => *resized = false,
            Self::Shared { resized, .. } => *resized = false,
        }
    }
}
