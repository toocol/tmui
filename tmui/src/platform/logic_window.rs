use std::sync::Arc;
use tipc::{ipc_master::IpcMaster, RwLock};

use crate::primitive::bitmap::Bitmap;

pub struct LogicWindow<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    bitmap: Arc<RwLock<Bitmap>>,
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> LogicWindow<T, M> {
    pub fn new(bitmap: Arc<RwLock<Bitmap>>, master: Option<Arc<RwLock<IpcMaster<T, M>>>>) -> Self {
        Self { master, bitmap }
    }

    pub fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        self.bitmap.clone()
    }

    pub fn resize(&self, width: u32, height: u32) {

    }
}
