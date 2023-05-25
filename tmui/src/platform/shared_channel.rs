use std::{sync::{mpsc::Receiver, Arc}, time::Instant};

use tipc::{ipc_event::IpcEvent, ipc_master::IpcMaster, ipc_slave::IpcSlave, IpcNode};

pub(crate) type SharedChannel<T, M> =
    (SharedSender<T, M>, SharedReceiver<T, M>);

enum SharedType {
    Master,
    Slave,
}

#[inline]
pub(crate) fn master_channel<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>(
    master: Arc<IpcMaster<T, M>>,
    user_event_receiver: Receiver<Vec<T>>,
) -> (SharedSender<T, M>, SharedReceiver<T, M>) {
    (
        SharedSender::<T, M>::new_master(master.clone()),
        SharedReceiver::<T, M>::new_master(master, user_event_receiver),
    )
}

#[inline]
pub(crate) fn slave_channel<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>(
    slave: Arc<IpcSlave<T, M>>,
    user_event_receiver: Receiver<Vec<T>>,
) -> (SharedSender<T, M>, SharedReceiver<T, M>) {
    (
        SharedSender::<T, M>::new_slave(slave.clone()),
        SharedReceiver::<T, M>::new_slave(slave, user_event_receiver),
    )
}

pub(crate) struct SharedSender<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    ty: SharedType,
    master: Option<Arc<IpcMaster<T, M>>>,
    slave: Option<Arc<IpcSlave<T, M>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> SharedSender<T, M> {
    #[inline]
    fn new_master(master: Arc<IpcMaster<T, M>>) -> Self {
        Self {
            ty: SharedType::Master,
            master: Some(master),
            slave: None,
        }
    }

    #[inline]
    fn new_slave(slave: Arc<IpcSlave<T, M>>) -> Self {
        Self {
            ty: SharedType::Slave,
            master: None,
            slave: Some(slave),
        }
    }

    #[inline]
    pub(crate) fn send_user_event(&self, user_evt: T) {
        match self.ty {
            SharedType::Master => self
                .master
                .as_ref()
                .unwrap()
                .try_send(IpcEvent::UserEvent(user_evt, Instant::now()))
                .unwrap(),
            SharedType::Slave => self
                .slave
                .as_ref()
                .unwrap()
                .try_send(IpcEvent::UserEvent(user_evt, Instant::now()))
                .unwrap(),
        }
    }

    #[inline]
    pub(crate) fn send_request(&self, request: M) -> Option<M> {
        match self.ty {
            SharedType::Master => self.master.as_ref().unwrap().send_request(request).unwrap(),
            SharedType::Slave => self.slave.as_ref().unwrap().send_request(request).unwrap(),
        }
    }

    #[inline]
    pub(crate) fn resp_request(&self, resp: Option<M>) {
        match self.ty {
            SharedType::Master => self.master.as_ref().unwrap().respose_request(resp),
            SharedType::Slave => self.slave.as_ref().unwrap().respose_request(resp),
        }
    }
}

pub(crate) struct SharedReceiver<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    ty: SharedType,
    master: Option<Arc<IpcMaster<T, M>>>,
    slave: Option<Arc<IpcSlave<T, M>>>,
    user_event_receiver: Receiver<Vec<T>>,
}
impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> SharedReceiver<T, M> {
    #[inline]
    fn new_master(master: Arc<IpcMaster<T, M>>, user_event_receiver: Receiver<Vec<T>>) -> Self {
        Self {
            ty: SharedType::Master,
            master: Some(master),
            slave: None,
            user_event_receiver,
        }
    }

    #[inline]
    fn new_slave(slave: Arc<IpcSlave<T, M>>, user_event_receiver: Receiver<Vec<T>>) -> Self {
        Self {
            ty: SharedType::Slave,
            master: None,
            slave: Some(slave),
            user_event_receiver,
        }
    }

    #[inline]
    pub(crate) fn receive_user_event_vec(&self) -> Vec<T> {
        let mut vec = vec![];
        while let Ok(mut evts) = self.user_event_receiver.try_recv() {
            vec.append(&mut evts);
        }
        vec
    }

    #[inline]
    pub(crate) fn receive_request(&self) -> Option<M> {
        match self.ty {
            SharedType::Master => self.master.as_ref().unwrap().try_recv_request(),
            SharedType::Slave => self.slave.as_ref().unwrap().try_recv_request(),
        }
    }
}
