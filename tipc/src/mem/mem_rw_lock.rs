use super::{
    mem_mutex::{MemMutex, MemMutexOp},
    BuildType,
};
use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::{
    mem::size_of,
    sync::atomic::{AtomicUsize, Ordering}, thread,
};

struct _MemRwLock {
    read: AtomicUsize,
    write: AtomicUsize,
}

// Read write lock based on shared memory.
pub struct MemRwLock {
    lock: Shmem,
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
            lock: shmem,
            mutex: MemMutex::new(os_id, MemMutexOp::Create)?,
        })
    }

    #[inline]
    pub fn open(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new().os_id(os_id).open()?;

        Ok(Self {
            lock: shmem,
            mutex: MemMutex::new(os_id, MemMutexOp::Open)?,
        })
    }

    #[inline]
    pub fn builder() -> MemRwLockBuilder {
        MemRwLockBuilder::new()
    }

    #[inline]
    pub fn read(&self) -> MemRwLockGuard {
        let lock = self.lock_mut();
        let _guard = self.mutex.lock();
        loop {
            let val = lock.write.load(Ordering::SeqCst);

            if val == 0 {
                break;
            }
            thread::yield_now();
        }
        lock.read.fetch_add(1, Ordering::SeqCst);

        MemRwLockGuard::new(self, LockType::Read)
    }

    #[inline]
    pub fn write(&self) -> MemRwLockGuard {
        let lock = self.lock_mut();
        let _guard = self.mutex.lock();
        loop {
            let read = lock.read.load(Ordering::SeqCst);
            let write = lock.write.load(Ordering::SeqCst);

            if read == 0 && write == 0 {
                break;
            }
            thread::yield_now();
        }
        lock.write.fetch_add(1, Ordering::SeqCst);

        MemRwLockGuard::new(self, LockType::Write)
    }

    #[inline]
    fn lock_mut(&self) -> &'static mut _MemRwLock {
        unsafe { (self.lock.as_ptr() as *mut _MemRwLock).as_mut().unwrap() }
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
        let lock = self.lock.lock_mut();

        match self.tty {
            LockType::Read => {
                lock.read.fetch_sub(1, Ordering::SeqCst);
            }
            LockType::Write => {
                let old = lock.write.fetch_sub(1, Ordering::SeqCst);
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
