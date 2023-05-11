use super::{mem_queue::{MemQueue, MemQueueError}, MemContext, IPC_QUEUE_SIZE};
use crate::{
    ipc_event::InnerIpcEvent,
    mem::{
        mem_queue::{BuildType, MemQueueBuilder},
        SharedInfo, IPC_MEM_MASTER_QUEUE, IPC_MEM_PRIMARY_BUFFER_NAME,
        IPC_MEM_SECONDARY_BUFFER_NAME, IPC_MEM_SHARED_INFO_NAME, IPC_MEM_SLAVE_QUEUE,
    },
};
use shared_memory::{Shmem, ShmemConf};
use std::{mem::size_of, sync::atomic::Ordering};

pub(crate) struct MasterContext {
    primary_buffer: Shmem,
    secondary_buffer: Shmem,
    shared_info: Shmem,
    master_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent>,
    slave_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent>,
}

impl MasterContext {
    pub(crate) fn create<T: ToString>(name: T, width: u32, height: u32) -> Self {
        let mut primary_buffer_name = name.to_string();
        primary_buffer_name.push_str(IPC_MEM_PRIMARY_BUFFER_NAME);
        let primary_buffer = ShmemConf::new()
            .size((width * height * 4) as usize)
            .os_id(primary_buffer_name)
            .create()
            .unwrap();

        let mut secondary_buffer_name = name.to_string();
        secondary_buffer_name.push_str(IPC_MEM_SECONDARY_BUFFER_NAME);
        let secondary_buffer = ShmemConf::new()
            .size((width * height * 4) as usize)
            .os_id(secondary_buffer_name)
            .create()
            .unwrap();

        let mut shared_info_name = name.to_string();
        shared_info_name.push_str(IPC_MEM_SHARED_INFO_NAME);
        let shared_info = ShmemConf::new()
            .size(size_of::<SharedInfo>())
            .os_id(shared_info_name)
            .create()
            .unwrap();
        let info_data = unsafe { (shared_info.as_ptr() as *mut SharedInfo).as_mut().unwrap() };
        info_data.width.store(width, Ordering::SeqCst);
        info_data.height.store(height, Ordering::SeqCst);

        let mut master_queue_name = name.to_string();
        master_queue_name.push_str(IPC_MEM_MASTER_QUEUE);
        let master_queue = MemQueueBuilder::new()
            .build_type(BuildType::Create)
            .os_id(master_queue_name)
            .build()
            .unwrap();

        let mut slave_queue_name = name.to_string();
        slave_queue_name.push_str(IPC_MEM_SLAVE_QUEUE);
        let slave_queue = MemQueueBuilder::new()
            .build_type(BuildType::Create)
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

impl MemContext for MasterContext {
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
        self.master_queue.try_write(evt)
    }

    #[inline]
    fn try_recv(&self) -> Vec<InnerIpcEvent> {
        let mut vec = vec![];
        while self.slave_queue.has_event() {
            if let Some(evt) = self.slave_queue.try_read() {
                vec.push(evt)
            }
        }
        vec
    }
}
