use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::{
    mem::size_of,
    sync::atomic::{AtomicBool, Ordering},
};

#[repr(C)]
struct _MemMutex {
    locked: AtomicBool,
}

/// Cross process mutex based on shared memory. <br>
/// `Non reentrant lock`
pub struct MemMutex {
    inner: Shmem,
}

impl MemMutex {
    pub fn new(key: &str, op: MemMutexOp) -> Result<Self, ShmemError> {
        let key = Self::gen_key(key);
        let inner = match op {
            MemMutexOp::Create => ShmemConf::new()
                .size(size_of::<_MemMutex>())
                .os_id(&key)
                .create()?,
            MemMutexOp::Open => ShmemConf::new().os_id(&key).open()?,
        };
        Ok(Self { inner })
    }

    #[inline]
    fn gen_key(str: &str) -> String {
        let mut key = "mmtx_".to_string();
        key.push_str(str);
        key
    }

    #[inline]
    pub fn lock(&self) -> MemMutexGuard {
        let inner = self.inner_mut();

        while inner
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {}

        MemMutexGuard::new(self)
    }

    #[inline]
    pub fn unlock(&self) {
        let inner = self.inner_mut();
        inner.locked.store(false, Ordering::Release);
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.inner_mut().locked.load(Ordering::Acquire)
    }

    #[inline]
    fn inner_mut(&self) -> &'static mut _MemMutex {
        unsafe { (self.inner.as_ptr() as *mut _MemMutex).as_mut().unwrap() }
    }
}

pub struct MemMutexGuard<'mutex> {
    mutex: &'mutex MemMutex,
}

impl Drop for MemMutexGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        if self.mutex.is_locked() {
            self.mutex.unlock()
        }
    }
}

impl<'mutex> MemMutexGuard<'mutex> {
    #[inline]
    fn new(mutex: &'mutex MemMutex) -> Self {
        Self { mutex }
    }
}

pub enum MemMutexOp {
    Open,
    Create,
}
