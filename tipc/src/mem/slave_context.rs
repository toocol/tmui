use std::sync::atomic::Ordering;

use super::{
    mem_queue::{BuildType, MemQueue, MemQueueBuilder, MemQueueError},
    IPC_MEM_MASTER_QUEUE, IPC_MEM_PRIMARY_BUFFER_NAME, IPC_MEM_SECONDARY_BUFFER_NAME,
    IPC_MEM_SHARED_INFO_NAME, IPC_MEM_SLAVE_QUEUE, IPC_QUEUE_SIZE, SharedInfo, MemContext,
};
use crate::ipc_event::InnerIpcEvent;
use shared_memory::{Shmem, ShmemConf};

pub(crate) struct SlaveContext {
    primary_buffer: Shmem,
    secondary_buffer: Shmem,
    shared_info: Shmem,
    master_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent>,
    slave_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent>,
}

impl SlaveContext {
    pub(crate) fn open<T: ToString>(name: T) -> Self {
        let mut primary_buffer_name = name.to_string();
        primary_buffer_name.push_str(IPC_MEM_PRIMARY_BUFFER_NAME);
        let primary_buffer = ShmemConf::new().os_id(primary_buffer_name).open().unwrap();

        let mut secondary_buffer_name = name.to_string();
        secondary_buffer_name.push_str(IPC_MEM_SECONDARY_BUFFER_NAME);
        let secondary_buffer = ShmemConf::new()
            .os_id(secondary_buffer_name)
            .open()
            .unwrap();

        let mut shared_info_name = name.to_string();
        shared_info_name.push_str(IPC_MEM_SHARED_INFO_NAME);
        let shared_info = ShmemConf::new().os_id(shared_info_name).open().unwrap();

        let mut master_queue_name = name.to_string();
        master_queue_name.push_str(IPC_MEM_MASTER_QUEUE);
        let master_queue = MemQueueBuilder::new()
            .build_type(BuildType::Open)
            .os_id(master_queue_name)
            .build()
            .unwrap();

        let mut slave_queue_name = name.to_string();
        slave_queue_name.push_str(IPC_MEM_SLAVE_QUEUE);
        let slave_queue = MemQueueBuilder::new()
            .build_type(BuildType::Open)
            .os_id(slave_queue_name)
            .build()
            .unwrap();

        Self {
            primary_buffer,
            secondary_buffer,
            shared_info,
            master_queue,
            slave_queue,
        }
    }

    pub(crate) fn shared_info(&self) -> &'static mut SharedInfo {
        unsafe {
            (self.shared_info.as_ptr() as *mut SharedInfo)
                .as_mut()
                .unwrap()
        }
    }
}

impl MemContext for SlaveContext {
    #[inline]
    fn primary_buffer(&self) -> *mut u8 {
        self.primary_buffer.as_ptr()
    }

    #[inline]
    fn secondary_buffer(&self) -> *mut u8 {
        self.secondary_buffer.as_ptr()
    }

    #[inline]
    fn width(&self) -> u32 {
        self.shared_info().width.load(Ordering::Relaxed)
    }

    #[inline]
    fn height(&self) -> u32 {
        self.shared_info().height.load(Ordering::Relaxed)
    }

    #[inline]
    fn try_send(&self, evt: InnerIpcEvent) -> Result<(), MemQueueError> {
        self.slave_queue.try_write(evt)
    }

    #[inline]
    fn try_recv(&self) -> Vec<InnerIpcEvent> {
        let mut vec = vec![];
        while self.master_queue.has_event() {
            if let Some(evt) = self.master_queue.try_read() {
                vec.push(evt)
            }
        }
        vec
    }
}
