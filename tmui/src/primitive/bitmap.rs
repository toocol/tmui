#![allow(unused_unsafe)]
use std::{
    ffi::c_void,
    ptr::NonNull,
    slice,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
};
use tipc::mem::mem_rw_lock::{MemRwLock, MemRwLockGuard};
use tlib::global::SemanticExt;

use crate::platform::ipc_inner_agent::IpcInnerAgent;

#[repr(C)]
struct _ShmemInfo {
    prepared: AtomicBool,

    master_release_idx: AtomicUsize,
    slave_release_idx: AtomicUsize,
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
        /// Used when application has shared memory widget, 
        /// Inner ipc agent to access shared memory related methods
        inner_agent: Option<Box<dyn IpcInnerAgent>>,
    },

    Shared {
        /// The raw pointer of shared memory pixels data.
        raw_pointer: Option<NonNull<c_void>>,
        /// The lock to visit `raw_pointer`.
        lock: Arc<MemRwLock>,
        /// The width of bitmap.
        width: u32,
        /// The height of bitmap.
        height: u32,
        /// The length of the pixels.
        total_bytes: usize,
        /// Bytes number of a row.
        row_bytes: usize,
        /// Only used on platform `macos`
        resized: bool,
        /// Inner ipc agent to access shared memory related methods.
        inner_agent: Box<dyn IpcInnerAgent>,
    },

    Empty {
        /// The width of bitmap.
        width: u32,
        /// The height of bitmap.
        height: u32,
        /// Is this bitmap rendered.
        prepared: bool,
    },
}

unsafe impl Send for Bitmap {}

impl Bitmap {
    #[inline]
    pub fn new(width: u32, height: u32, inner_agent: Option<Box<dyn IpcInnerAgent>>) -> Self {
        Self::Direct {
            pixels: Some(vec![0u8; (width * height * 4) as usize].boxed()),
            retention: None,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            width,
            height,
            prepared: false,
            resized: false,
            inner_agent,
        }
    }

    #[inline]
    pub fn empty(width: u32, height: u32) -> Self {
        Self::Empty { width, height, prepared: false }
    }

    #[inline]
    pub fn from_raw_pointer(
        pointer: *mut c_void,
        width: u32,
        height: u32,
        lock: Arc<MemRwLock>,
        inner_agent: Box<dyn IpcInnerAgent>,
    ) -> Self {
        Self::Shared {
            raw_pointer: NonNull::new(pointer),
            lock: lock,
            width,
            height,
            total_bytes: (width * height * 4) as usize,
            row_bytes: (width * 4) as usize,
            resized: false,
            inner_agent,
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
                ..
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
            Self::Empty { width, height, prepared } => {
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
                total_bytes,
                row_bytes,
                resized,
                width,
                height,
                ..
            } => {
                *raw_pointer = NonNull::new(n_raw_pointer);
                *total_bytes = (w * h * 4) as usize;
                *row_bytes = (w * 4) as usize;
                *resized = true;
                *width = w;
                *height = h;
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
            Self::Empty { .. } => {
                unreachable!()
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
            Self::Empty { .. } => {
                unreachable!()
            }
        }
    }

    #[inline]
    pub fn row_bytes(&self) -> usize {
        match self {
            Self::Direct { row_bytes, .. } => *row_bytes,
            Self::Shared { row_bytes, .. } => *row_bytes,
            Self::Empty { .. } => unreachable!(),
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        match self {
            Self::Direct { width, .. } => *width,
            Self::Shared { width, .. } => *width,
            Self::Empty { width, .. } => *width,
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        match self {
            Self::Direct { height, .. } => *height,
            Self::Shared { height, .. } => *height,
            Self::Empty { height, .. } => *height,
        }
    }

    #[inline]
    pub fn prepared(&mut self) {
        match self {
            Self::Direct { prepared, .. } => *prepared = true,
            Self::Shared { inner_agent, .. } => inner_agent.prepared(),
            Self::Empty { prepared, .. } => *prepared = true,
        }
    }

    #[inline]
    pub fn is_prepared(&self) -> bool {
        match self {
            Self::Direct { prepared, .. } => *prepared,
            Self::Empty { prepared, .. } => *prepared,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn release_retention(&mut self) {
        match self {
            Self::Direct {
                retention,
                inner_agent,
                ..
            } => {
                retention.take();

                if let Some(agent) = inner_agent.as_ref() {
                    agent.release_retention();
                }
            }
            Self::Shared { inner_agent, .. } => {
                inner_agent.release_retention();
            }
            Self::Empty { .. } => {},
        }
    }

    /// Can't used with [`ipc_write()`](Bitmap::ipc_write) in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn ipc_read(&self) -> Option<MemRwLockGuard> {
        match self {
            Self::Direct { .. } => None,
            Self::Shared { lock, .. } => Some(lock.read()),
            Self::Empty { .. } => None,
        }
    }

    /// Can't used with [`ipc_read()`](Bitmap::ipc_read) in same code block, otherwise, it may cause deadlock issues
    #[inline]
    pub fn ipc_write(&self) -> Option<MemRwLockGuard> {
        match self {
            Self::Direct { .. } => None,
            Self::Shared { lock, .. } => Some(lock.write()),
            Self::Empty { .. } => None,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_resized(&self) -> bool {
        match self {
            Self::Direct { resized, .. } => *resized,
            Self::Shared { resized, .. } => *resized,
            Self::Empty { .. } => unreachable!(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn reset_resized(&mut self) {
        match self {
            Self::Direct { resized, .. } => *resized = false,
            Self::Shared { resized, .. } => *resized = false,
            Self::Empty { .. } => unreachable!(),
        }
    }

    #[inline]
    pub fn is_shared_invalidate(&self) -> bool {
        match self {
            Self::Direct { inner_agent, .. } => {
                if let Some(agent) = inner_agent {
                    return agent.is_invalidate();
                } else {
                    return false;
                }
            }
            Self::Shared { .. } => false,
            Self::Empty { .. } => false,
        }
    }

    #[inline]
    pub fn set_shared_invalidate(&self, invalidate: bool) {
        match self {
            Self::Shared { inner_agent, .. } => inner_agent.set_invalidate(invalidate),
            _ => {}
        }
    }
}
