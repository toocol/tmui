use std::sync::Arc;
use tipc::{
    ipc_master::IpcMaster, ipc_slave::IpcSlave,
    mem::mem_rw_lock::MemRwLock, IpcNode, IpcType, RwLock,
};
use tlib::winit::raw_window_handle::RawWindowHandle;
use crate::{
    application_window::ApplicationWindow,
    backend::BackendType,
    primitive::{bitmap::Bitmap, shared_channel::SharedChannel},
    runtime::window_context::LogicWindowContext,
};
use super::{
    ipc_bridge::{IpcBridge, IpcBridgeModel},
    PlatformType,
};

pub(crate) struct LogicWindow<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    raw_window_handle: Option<RawWindowHandle>,
    bitmap: Arc<RwLock<Bitmap>>,
    lock: Option<Arc<MemRwLock>>,

    shared_widget_id: Option<&'static str>,
    slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,

    pub platform_type: PlatformType,
    pub backend_type: BackendType,
    pub ipc_type: IpcType,

    pub master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    pub shared_channel: Option<SharedChannel<T, M>>,
    pub context: Option<LogicWindowContext>,

    pub on_activate: Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>>,
    pub on_user_event_receive: Option<Box<dyn Fn(&mut ApplicationWindow, T) + Send + Sync>>,
    pub on_request_receive:
        Option<Box<dyn Fn(&mut ApplicationWindow, M) -> Option<M> + Send + Sync>>,
}

unsafe impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> Send
    for LogicWindow<T, M>
{
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> LogicWindow<T, M> {
    pub fn master(
        raw_window_handle: RawWindowHandle,
        bitmap: Arc<RwLock<Bitmap>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        shared_channel: Option<SharedChannel<T, M>>,
        context: LogicWindowContext,
    ) -> Self {
        let lock = master.as_ref().and_then(|m| Some(m.read().buffer_lock()));
        Self {
            raw_window_handle: Some(raw_window_handle),
            bitmap,
            lock,
            shared_widget_id: None,
            slave: None,
            platform_type: PlatformType::default(),
            backend_type: BackendType::default(),
            ipc_type: IpcType::Master,
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
        slave: Arc<RwLock<IpcSlave<T, M>>>,
        shared_channel: Option<SharedChannel<T, M>>,
        context: LogicWindowContext,
    ) -> Self {
        let lock = Some(slave.read().buffer_lock());
        Self {
            raw_window_handle: None,
            bitmap,
            lock,
            shared_widget_id: Some(shared_widget_id),
            slave: Some(slave),
            platform_type: PlatformType::default(),
            backend_type: BackendType::default(),
            ipc_type: IpcType::Slave,
            master: None,
            shared_channel,
            context: Some(context),
            on_activate: None,
            on_user_event_receive: None,
            on_request_receive: None,
        }
    }

    #[inline]
    pub fn raw_window_handle(&self) -> Option<RawWindowHandle> {
        self.raw_window_handle
    }

    #[inline]
    pub fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        self.bitmap.clone()
    }

    pub fn resize(&self, width: u32, height: u32) {
        let mut bitmap_guard = self.bitmap.write();
        let _guard = self.lock.as_ref().and_then(|l| Some(l.write()));

        if let Some(ref slave) = self.slave {
            let mut slave = slave.write();
            slave.recreate_buffer();

            bitmap_guard.update_raw_pointer(
                slave.buffer_raw_pointer(),
                slave.width(),
                slave.height(),
            )
        } else {
            bitmap_guard.resize(width, height);

            if let Some(ref master) = self.master {
                master.write().recreate_buffer();
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
