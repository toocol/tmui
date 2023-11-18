use super::{
    mem_mutex::{MemMutex, MemMutexOp},
    BuildType,
};
use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::{
    mem::size_of,
    sync::atomic::{Ordering, AtomicU8}, thread,
};

#[repr(C)]
struct _MemRwLock {
    read: AtomicU8,
    write: AtomicU8,
}

/// Read write lock based on shared memory.
/// It's a cross process lock. <br>
/// `Non reentrant lock`
pub struct MemRwLock {
    inner: Shmem,
    mutex: MemMutex,
}

unsafe impl Send for MemRwLock {}
unsafe impl Sync for MemRwLock {}

impl MemRwLock {
    #[inline]
    pub fn create_with_os_id(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new()
            .size(size_of::<_MemRwLock>())
            .os_id(os_id)
            .create()?;

        Ok(Self {
            inner: shmem,
            mutex: MemMutex::new(os_id, MemMutexOp::Create)?,
        })
    }

    #[inline]
    pub fn open(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new().os_id(os_id).open()?;

        Ok(Self {
            inner: shmem,
            mutex: MemMutex::new(os_id, MemMutexOp::Open)?,
        })
    }

    #[inline]
    pub fn builder() -> MemRwLockBuilder {
        MemRwLockBuilder::new()
    }

    #[inline]
    pub fn read(&self) -> MemRwLockGuard {
        let inner = self.inner_mut();
        let _guard = self.mutex.lock();
        loop {
            let val = inner.write.load(Ordering::SeqCst);

            if val == 0 {
                break;
            }
            thread::yield_now();
        }
        inner.read.fetch_add(1, Ordering::SeqCst);

        MemRwLockGuard::new(self, LockType::Read)
    }

    #[inline]
    pub fn write(&self) -> MemRwLockGuard {
        let inner = self.inner_mut();
        let _guard = self.mutex.lock();
        loop {
            let read = inner.read.load(Ordering::SeqCst);
            let write = inner.write.load(Ordering::SeqCst);

            if read == 0 && write == 0 {
                break;
            }
            thread::yield_now();
        }
        inner.write.fetch_add(1, Ordering::SeqCst);

        MemRwLockGuard::new(self, LockType::Write)
    }

    #[inline]
    fn inner_mut(&self) -> &'static mut _MemRwLock {
        unsafe { (self.inner.as_ptr() as *mut _MemRwLock).as_mut().unwrap() }
    }
}

pub struct MemRwLockGuard<'lock> {
    lock: &'lock MemRwLock,
    tty: LockType,
}

impl<'lock> MemRwLockGuard<'lock> {
    #[inline]
    fn new(lock: &'lock MemRwLock, tty: LockType) -> Self {
        Self { lock, tty }
    }
}

impl<'lock> Drop for MemRwLockGuard<'lock> {
    #[inline]
    fn drop(&mut self) {
        let inner = self.lock.inner_mut();

        match self.tty {
            LockType::Read => {
                inner.read.fetch_sub(1, Ordering::SeqCst);
            }
            LockType::Write => {
                let old = inner.write.fetch_sub(1, Ordering::SeqCst);
                debug_assert_eq!(old, 1)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum LockType {
    Read,
    Write,
}

#[derive(Default)]
pub struct MemRwLockBuilder {
    build_type: BuildType,
    os_id: Option<String>,
}

impl MemRwLockBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn build_type(mut self, build_type: BuildType) -> Self {
        self.build_type = build_type;
        self
    }

    #[inline]
    pub fn os_id<P: ToString>(mut self, os_id: P) -> Self {
        self.os_id = Some(os_id.to_string());
        self
    }

    pub fn build(self) -> Result<MemRwLock, ShmemError> {
        match self.build_type {
            BuildType::Create => {
                if let Some(ref os_id) = self.os_id {
                    return MemRwLock::create_with_os_id(os_id);
                } else {
                    panic!("`Create` MemRwLock must assign the os_id")
                }
            }
            BuildType::Open => {
                if let Some(ref os_id) = self.os_id {
                    return MemRwLock::open(os_id);
                } else {
                    panic!("`Open` MemRwLock must assign the os_id")
                }
            }
        }
    }
}
