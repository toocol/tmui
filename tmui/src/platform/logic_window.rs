use std::sync::Arc;
use tipc::{
    ipc_master::IpcMaster, ipc_slave::IpcSlave, lock_api::RwLockWriteGuard, IpcNode, RawRwLock,
    RwLock,
};
use tlib::ptr_ref;

use crate::{
    primitive::{bitmap::Bitmap, shared_channel::SharedChannel},
    runtime::window_context::LogicWindowContext, application_window::ApplicationWindow, backend::BackendType,
};

use super::{ipc_bridge::{IpcBridge, IpcBridgeModel}, PlatformType};

pub(crate) struct LogicWindow<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    bitmap: Arc<RwLock<Bitmap>>,

    shared_widget_id: Option<&'static str>,
    slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,

    pub platform_type: PlatformType,
    pub backend_type: BackendType,

    pub master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    pub shared_channel: Option<SharedChannel<T, M>>,
    pub context: Option<LogicWindowContext>,

    pub on_activate: Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>>,
    pub on_user_event_receive: Option<Box<dyn Fn(&mut ApplicationWindow, T) + Send + Sync>>,
    pub on_request_receive: Option<Box<dyn Fn(&mut ApplicationWindow, M) -> Option<M> + Send + Sync>>,
}

unsafe impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> Send
    for LogicWindow<T, M>
{
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> LogicWindow<T, M> {
    pub fn master(
        bitmap: Arc<RwLock<Bitmap>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        shared_channel: Option<SharedChannel<T, M>>,
        context: LogicWindowContext,
    ) -> Self {
        Self {
            bitmap,
            shared_widget_id: None,
            slave: None,
            platform_type: PlatformType::default(),
            backend_type: BackendType::default(),
            master,
            shared_channel,
            context: Some(context),
            on_activate: None,
            on_user_event_receive: None,
            on_request_receive: None,
        }
    }

    pub fn slave(
        bitmap: Arc<RwLock<Bitmap>>,
        shared_widget_id: &'static str,
        slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,
        shared_channel: Option<SharedChannel<T, M>>,
        context: LogicWindowContext,
    ) -> Self {
        Self {
            bitmap,
            shared_widget_id: Some(shared_widget_id),
            slave,
            platform_type: PlatformType::default(),
            backend_type: BackendType::default(),
            master: None,
            shared_channel,
            context: Some(context),
            on_activate: None,
            on_user_event_receive: None,
            on_request_receive: None,
        }
    }

    pub fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        self.bitmap.clone()
    }

    pub fn resize(&self, width: u32, height: u32) {
        let mut bitmap_guard = self.bitmap.write();

        if let Some(ref slave) = self.slave {
            let _guard = ptr_ref!(&bitmap_guard as *const RwLockWriteGuard<'_, RawRwLock, Bitmap>)
                .ipc_write();

            let mut slave = slave.write();
            let old_shmem = slave.resize(width, height);

            bitmap_guard.update_raw_pointer(
                slave.buffer_raw_pointer(),
                old_shmem,
                slave.width(),
                slave.height(),
            )
        } else {
            match self.master {
                Some(ref master) => {
                    let _guard =
                        ptr_ref!(&bitmap_guard as *const RwLockWriteGuard<'_, RawRwLock, Bitmap>)
                            .ipc_write();

                    let mut master = master.write();
                    let old_shmem = master.resize(width, height);

                    bitmap_guard.update_raw_pointer(
                        master.buffer_raw_pointer(),
                        old_shmem,
                        width,
                        height,
                    );
                }
                None => bitmap_guard.resize(width, height),
            }
        }
    }

    #[inline]
    pub fn create_ipc_bridge(&self) -> Option<Box<dyn IpcBridge>> {
        if self.master.is_none() && self.slave.is_none() {
            return None;
        }

        Some(IpcBridgeModel::<T, M>::new(
            self.shared_widget_id.clone(),
            self.master.clone(),
            self.slave.clone(),
        ))
    }
}