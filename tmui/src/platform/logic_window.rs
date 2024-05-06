use super::{
    gl_bootstrap::GlEnv,
    ipc_bridge::{IpcBridge, IpcBridgeModel},
    PlatformType,
};
use crate::{
    application::{FnActivate, FnRequestReceive, FnUserEventReceive},
    backend::BackendType,
    primitive::{bitmap::Bitmap, shared_channel::SharedChannel},
    runtime::window_context::LogicWindowContext,
};
use glutin::config::Config;
use std::sync::Arc;
use tipc::{
    ipc_master::IpcMaster, ipc_slave::IpcSlave, mem::mem_rw_lock::MemRwLock, parking_lot::RwLock,
    IpcNode, IpcType,
};
use tlib::winit::window::WindowId;

pub(crate) struct LogicWindow<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    window_id: Option<WindowId>,
    parent_window: Option<WindowId>,
    gl_env: Option<Arc<GlEnv>>,

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

    pub on_activate: Option<FnActivate>,
    pub on_user_event_receive: Option<FnUserEventReceive<T>>,
    pub on_request_receive: Option<FnRequestReceive<M>>,
}

unsafe impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> Send
    for LogicWindow<T, M>
{
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> LogicWindow<T, M> {
    pub fn master(
        window_id: WindowId,
        gl_env: Option<Arc<GlEnv>>,
        bitmap: Arc<RwLock<Bitmap>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        shared_channel: Option<SharedChannel<T, M>>,
        context: LogicWindowContext,
    ) -> Self {
        let lock = master.as_ref().map(|m| m.read().buffer_lock());
        Self {
            window_id: Some(window_id),
            parent_window: None, 
            gl_env,
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
            window_id: None,
            parent_window: None,
            gl_env: None,
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
    pub(crate) fn set_parent_window(&mut self, parent: WindowId) {
        self.parent_window = Some(parent);
    }

    #[inline]
    pub(crate) fn get_parent_window(&self) -> Option<WindowId> {
        self.parent_window
    }


    #[inline]
    pub fn window_id(&self) -> Option<WindowId> {
        self.window_id
    }

    #[inline]
    pub fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        self.bitmap.clone()
    }

    pub fn resize(&self, width: u32, height: u32, ipc_only: bool) {
        let mut bitmap_guard = self.bitmap.write();
        let _guard = self.lock.as_ref().map(|l| l.write());

        if let Some(ref slave) = self.slave {
            let mut slave = slave.write();
            slave.recreate_buffer();

            bitmap_guard.update_raw_pointer(
                slave.buffer_raw_pointer(),
                slave.width(),
                slave.height(),
            )
        } else {
            if !ipc_only {
                bitmap_guard.resize(width, height);
            }

            if let Some(ref master) = self.master {
                master.write().recreate_buffer();
            }
        }

        if let Some(ref gl_env) = self.gl_env {
            gl_env.resize(width, height)
        }
    }

    #[inline]
    pub fn create_ipc_bridge(&self) -> Option<Box<dyn IpcBridge>> {
        if self.master.is_none() && self.slave.is_none() {
            return None;
        }

        Some(IpcBridgeModel::<T, M>::new(
            self.shared_widget_id,
            self.master.clone(),
            self.slave.clone(),
        ))
    }

    #[inline]
    pub fn gl_make_current(&self) {
        if let Some(ref gl_env) = self.gl_env {
            gl_env.make_current()
        }
    }

    #[inline]
    pub fn gl_load(&self) {
        if let Some(ref gl_env) = self.gl_env {
            gl_env.load()
        }
    }

    #[inline]
    pub fn gl_config_unwrap(&self) -> &Config {
        self.gl_env.as_ref().unwrap().config()
    }

    #[inline]
    pub fn gl_swap_buffers(&self) {
        if let Some(ref gl_env) = self.gl_env {
            gl_env.swap_buffers();
        }
    }

    #[inline]
    pub fn is_gl_backend(&self) -> bool {
        self.gl_env.is_some()
    }
}
