use super::BuildType;
use parking_lot::Mutex;
use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::{
    error::Error,
    fmt::Display,
    mem::size_of,
    sync::atomic::{AtomicUsize, Ordering},
};

#[repr(C)]
struct _MemQueue<const QUEUE_SIZE: usize, T: 'static + Copy> {
    read_indicate: AtomicUsize,
    write_indicate: AtomicUsize,
    events: [MaybeUninit<T>; QUEUE_SIZE],
}

impl<const QUEUE_SIZE: usize, T: 'static + Copy> _MemQueue<QUEUE_SIZE, T> {
    #[inline]
    fn clear(&self) {
        self.read_indicate.store(0, Ordering::Release);
        self.write_indicate.store(0, Ordering::Release);
    }

    #[inline]
    fn has_event(&self) -> bool {
        self.read_indicate.load(Ordering::Relaxed) != self.write_indicate.load(Ordering::Relaxed)
    }

    #[inline]
    fn is_full(&self) -> bool {
        let read = self.read_indicate.load(Ordering::Relaxed);
        let write = self.write_indicate.load(Ordering::Relaxed);
        write.wrapping_add(1) % QUEUE_SIZE == read % QUEUE_SIZE
    }

    #[inline]
    fn try_read(&self) -> Option<T> {
        if self.has_event() {
            let read = self.read_indicate.load(Ordering::Acquire);
            let evt = unsafe { self.events[read % QUEUE_SIZE].assume_init_read() };
            self.read_indicate
                .store(read.wrapping_add(1), Ordering::Release);
            Some(evt)
        } else {
            None
        }
    }

    #[inline]
    fn try_write(&mut self, evt: T) -> Result<(), MemQueueError> {
        if !self.is_full() {
            let write = self.write_indicate.load(Ordering::Acquire);
            unsafe { std::ptr::write(self.events[write % QUEUE_SIZE].assume_init_mut(), evt) };
            self.write_indicate
                .store(write.wrapping_add(1), Ordering::Release);
            Ok(())
        } else {
            Err(MemQueueError::new("`MemQueue` was full, evt was aborted."))
        }
    }
}

pub struct MemQueue<const QUEUE_SIZE: usize, T: 'static + Copy> {
    shmem: Shmem,
    _type_holder: PhantomData<T>,
    mutex: Mutex<()>,
}

impl<const QUEUE_SIZE: usize, T: 'static + Copy> MemQueue<QUEUE_SIZE, T> {
    pub fn create() -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new()
            .size(size_of::<_MemQueue<QUEUE_SIZE, T>>())
            .create()?;

        Ok(Self {
            shmem,
            _type_holder: PhantomData,
            mutex: Mutex::new(()),
        })
    }

    pub fn create_with_os_id(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new()
            .size(size_of::<_MemQueue<QUEUE_SIZE, T>>())
            .os_id(os_id)
            .create()?;

        Ok(Self {
            shmem,
            _type_holder: PhantomData,
            mutex: Mutex::new(()),
        })
    }

    pub fn open(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new().os_id(os_id).open()?;

        Ok(Self {
            shmem,
            _type_holder: PhantomData,
            mutex: Mutex::new(()),
        })
    }

    #[inline]
    pub fn os_id(&self) -> &str {
        self.shmem.get_os_id()
    }

    #[inline]
    pub fn clear(&self) {
        self.queue_mut().clear();
    }

    #[inline]
    pub fn has_event(&self) -> bool {
        self.queue_mut().has_event()
    }

    #[inline]
    pub fn try_read(&self) -> Option<T> {
        let _guard = self.mutex.lock();
        self.queue_mut().try_read()
    }

    /// If the queue was full, the event will be aborted.
    #[inline]
    pub fn try_write(&self, evt: T) -> Result<(), MemQueueError> {
        let _guard = self.mutex.lock();
        self.queue_mut().try_write(evt)?;
        Ok(())
    }

    #[inline]
    fn queue_mut(&self) -> &'static mut _MemQueue<QUEUE_SIZE, T> {
        unsafe {
            (self.shmem.as_ptr() as *mut _MemQueue<QUEUE_SIZE, T>)
                .as_mut()
                .unwrap()
        }
    }
}

#[derive(Default)]
pub struct MemQueueBuilder {
    build_type: BuildType,
    os_id: Option<String>,
}

impl MemQueueBuilder {
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

    pub fn build<const QUEUE_SIZE: usize, T: 'static + Copy>(
        self,
    ) -> Result<MemQueue<QUEUE_SIZE, T>, ShmemError> {
        match self.build_type {
            BuildType::Create => {
                if let Some(ref os_id) = self.os_id {
                    MemQueue::create_with_os_id(os_id)
                } else {
                    MemQueue::create()
                }
            }
            BuildType::Open => {
                if let Some(ref os_id) = self.os_id {
                    MemQueue::open(os_id)
                } else {
                    panic!("`Open` MemQueue must assign the os_id")
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MemQueueError {
    msg: &'static str,
}
impl MemQueueError {
    pub fn new(msg: &'static str) -> Self {
        Self { msg }
    }
}

impl Error for MemQueueError {}

impl Display for MemQueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg)
    }
}
