use crate::ipc_event::{CIpcEvent, IpcEvent};
use std::{
    ffi::{c_char, c_int, c_longlong, CString},
    os::raw,
};

pub(crate) struct IpcAdapter;
impl IpcAdapter {
    // Master
    /// Create the shared pixels.
    #[inline]
    pub fn create_master_context(name: &str, width: i32, height: i32) -> i32 {
        unsafe {
            let c_str = CString::new(name).unwrap();
            return create_master_context(c_str.as_ptr(), width, height);
        }
    }

    /// terminate and delete the shared pixels by id.
    #[inline]
    pub fn terminate_by_master(id: i32) -> i32 {
        unsafe { return terminate_by_master(id) }
    }

    /// Get the primary buffer on master side.
    #[inline]
    pub fn get_primary_buffer_master(id: i32) -> *mut u8 {
        unsafe { return get_primary_buffer_master(id) }
    }

    /// Get the secondary buffer on master side.
    #[inline]
    pub fn get_secondary_buffer_master(id: i32) -> *mut u8 {
        unsafe { return get_secondary_buffer_master(id) }
    }

    /// Master send the event.
    #[inline]
    pub fn send_event_master(id: i32, evt: CIpcEvent) {
        let raw_ptr = match &evt {
            CIpcEvent::KeyPressedEvent(ptr, _, _, _) => Some(*ptr),
            CIpcEvent::KeyReleasedEvent(ptr, _, _, _) => Some(*ptr),
            CIpcEvent::NativeEvent(ptr, _) => Some(*ptr),
            CIpcEvent::SharedMessage(ptr, _) => Some(*ptr),
            _ => None,
        };
        unsafe { send_event_master(id, evt) };
        // Release the C style string.
        if let Some(ptr) = raw_ptr {
            let _ = unsafe { CString::from_raw(ptr as *mut i8) };
        }
    }

    /// Blocked recived the event from slave.
    #[inline]
    pub fn recv_from_slave(id: i32) -> CIpcEvent {
        unsafe { return recv_from_slave(id) }
    }

    /// Non-blocked recived the event from slave.
    #[inline]
    pub fn try_recv_from_slave(id: i32) -> CIpcEvent {
        unsafe { return try_recv_from_slave(id) }
    }

    /// Slave block send msg to shared memory server side with response.
    #[inline]
    pub fn send_msg_master(id: i32, msg: &str, shared_string_type: i32) -> String {
        unsafe {
            let c_str = CString::new(msg).unwrap();
            let res = send_msg_master(id, c_str.as_ptr(), shared_string_type);
            let res = CString::from_raw(res as *mut c_char)
                .to_str()
                .unwrap()
                .to_string();
            return res;
        }
    }

    /// Get the shared msg, must invoke `resp_shared_msg_master()` after this.
    #[inline]
    pub fn get_shared_msg_master(id: i32) -> String {
        unsafe {
            let c_str = CString::from_raw(get_shared_msg_master(id) as *mut c_char);
            c_str.to_str().unwrap().to_string()
        }
    }

    /// Response the shared message result.
    #[inline]
    pub fn resp_shared_msg_master(id: i32, resp: &str) {
        unsafe {
            let c_str = CString::new(resp).unwrap();
            resp_shared_msg_master(id, c_str.as_ptr());
        }
    }

    /// Determine the Master has shared message to consume or not.
    #[inline]
    pub fn master_has_shared_msg(id: i32) -> bool {
        unsafe { master_has_shared_msg(id) }
    }

    // Slave
    /// Acquire next avaliable id of connection.
    #[inline]
    pub fn next_key() -> i32 {
        unsafe {
            return next_key();
        }
    }

    /// Connect to the shared memory by name;
    #[inline]
    pub fn connect_to(name: &str) -> i32 {
        unsafe {
            let c_str = CString::new(name).unwrap();
            return connect_to(c_str.as_ptr());
        }
    }

    /// Terminate the shared memory connection by id.
    #[inline]
    pub fn terminate_at(id: i32) -> bool {
        unsafe {
            return terminate_at(id);
        }
    }

    /// Judge whether the shared memory corresponding to the id is connected.
    #[inline]
    pub fn is_connected(id: i32) -> bool {
        unsafe {
            return is_connected(id);
        }
    }

    /// Slave send the event.
    #[inline]
    pub fn send_event_slave(id: i32, evt: CIpcEvent) {
        let raw_ptr = match &evt {
            CIpcEvent::KeyPressedEvent(ptr, _, _, _) => Some(*ptr),
            CIpcEvent::KeyReleasedEvent(ptr, _, _, _) => Some(*ptr),
            CIpcEvent::NativeEvent(ptr, _) => Some(*ptr),
            CIpcEvent::SharedMessage(ptr, _) => Some(*ptr),
            _ => None,
        };
        unsafe { send_event_slave(id, evt) };
        // Release the C style string.
        if let Some(ptr) = raw_ptr {
            let _ = unsafe { CString::from_raw(ptr as *mut i8) };
        }
    }

    /// Blocked recived the event from master.
    #[inline]
    pub fn recv_from_master(id: i32) -> CIpcEvent {
        unsafe { return recv_from_master(id) }
    }

    /// Non-blocked recived the event from slave.
    #[inline]
    pub fn try_recv_from_master(id: i32) -> CIpcEvent {
        unsafe { return try_recv_from_master(id) }
    }

    /// Slave block send msg to shared memory server side with response.
    #[inline]
    pub fn send_msg_slave(id: i32, msg: &str, shared_string_type: i32) -> String {
        unsafe {
            let c_str = CString::new(msg).unwrap();
            let res = send_msg_slave(id, c_str.as_ptr(), shared_string_type);
            let res = CString::from_raw(res as *mut c_char)
                .to_str()
                .unwrap()
                .to_string();
            return res;
        }
    }

    /// Get the shared msg, must invoke `resp_shared_msg_master()` after this.
    #[inline]
    pub fn get_shared_msg_slave(id: i32) -> String {
        unsafe {
            let c_str = CString::from_raw(get_shared_msg_slave(id) as *mut c_char);
            c_str.to_str().unwrap().to_string()
        }
    }

    /// Response the shared message result.
    #[inline]
    pub fn resp_shared_msg_slave(id: i32, resp: &str) {
        unsafe {
            let c_str = CString::new(resp).unwrap();
            resp_shared_msg_slave(id, c_str.as_ptr());
        }
    }

    /// Determine the Slave has shared message to consume or not.
    #[inline]
    pub fn slave_has_shared_msg(id: i32) -> bool {
        unsafe { slave_has_shared_msg(id) }
    }

    /// Resize the teminal emulator.
    #[inline]
    pub fn resize(id: i32, width: i32, height: i32) {
        unsafe {
            resize(id, width, height);
        }
    }

    /// Toggle buffer between primary/secondary buffer.
    #[inline]
    pub fn toggle_buffer(id: i32) {
        unsafe { toggle_buffer(id) }
    }

    /// When the native image buffer was changed, the property of dirty was true.
    #[inline]
    pub fn is_dirty(id: i32) -> bool {
        unsafe { is_dirty(id) }
    }

    /// Set the native image buffer was dirty.
    #[inline]
    pub fn set_dirty(id: i32, value: bool) {
        unsafe {
            set_dirty(id, value);
        }
    }

    /// Set true when the native image buffer was rendering completed, set false otherwise.
    #[inline]
    pub fn set_buffer_ready(id: i32, is_buffer_ready: bool) {
        unsafe {
            set_buffer_ready(id, is_buffer_ready);
        }
    }

    /// Get the native image buffer redering state.
    #[inline]
    pub fn is_buffer_ready(id: i32) -> bool {
        unsafe { is_buffer_ready(id) }
    }

    /// Get the width of native image buffer.
    #[inline]
    pub fn get_w(id: i32) -> i32 {
        unsafe { get_w(id) }
    }

    /// Get the height of native image buffer.
    #[inline]
    pub fn get_h(id: i32) -> i32 {
        unsafe { get_h(id) }
    }

    /// Get the primary native image buffer.
    #[inline]
    pub fn get_primary_buffer(id: i32) -> *mut u8 {
        unsafe { get_primary_buffer(id) }
    }

    /// Get the secondary native image buffer.
    #[inline]
    pub fn get_secondary_buffer(id: i32) -> *mut u8 {
        unsafe { get_secondary_buffer(id) }
    }

    /// Thread lock the common resource.
    #[inline]
    pub fn lock(id: i32) -> bool {
        unsafe { lock(id) }
    }

    /// Thread lock the common resource with timeout.
    #[inline]
    pub fn lock_timeout(id: i32, timeout: i64) -> bool {
        unsafe { lock_timeout(id, timeout) }
    }

    /// Unlock the common resource.
    #[inline]
    pub fn unlock(id: i32) {
        unsafe { unlock(id) }
    }

    /// Blocking wait for native image buffer changes.
    #[inline]
    pub fn wait_for_buffer_changes(id: i32) {
        unsafe { wait_for_buffer_changes(id) }
    }

    /// Whether the native image buffer has changed.
    #[inline]
    pub fn has_buffer_changes(id: i32) -> bool {
        unsafe { has_buffer_changes(id) }
    }

    /// Get current native image buffer status
    #[inline]
    pub fn buffer_status(id: i32) -> i32 {
        unsafe { buffer_status(id) }
    }

    /// Thread lock the primary native image buffer.
    #[inline]
    pub fn lock_buffer(id: i32) -> bool {
        unsafe { lock_buffer(id) }
    }

    /// Thread unlock the primary native image buffer.
    #[inline]
    pub fn unlock_buffer(id: i32) {
        unsafe {
            unlock_buffer(id);
        }
    }
}

#[link(name = "ipc-native", kind = "static")]
extern "C" {
    // Master
    fn create_master_context(name: *const c_char, width: c_int, height: c_int) -> c_int;
    fn terminate_by_master(id: c_int) -> c_int;
    fn get_primary_buffer_master(id: c_int) -> *mut u8;
    fn get_secondary_buffer_master(id: c_int) -> *mut u8;
    fn send_event_master(id: c_int, evt: CIpcEvent);
    fn recv_from_slave(id: c_int) -> CIpcEvent;
    fn try_recv_from_slave(id: c_int) -> CIpcEvent;
    fn send_msg_master(id: c_int, msg: *const c_char, shared_string_type: c_int) -> *const c_char;
    fn get_shared_msg_master(id: c_int) -> *const c_char;
    fn resp_shared_msg_master(id: c_int, resp: *const c_char);
    fn master_has_shared_msg(id: c_int) -> bool;

    // Slave
    fn next_key() -> c_int;
    fn connect_to(name: *const c_char) -> c_int;
    fn terminate_at(id: c_int) -> bool;
    fn is_connected(id: c_int) -> bool;
    fn send_event_slave(id: c_int, evt: CIpcEvent);
    fn recv_from_master(id: c_int) -> CIpcEvent;
    fn try_recv_from_master(id: c_int) -> CIpcEvent;
    fn send_msg_slave(id: c_int, msg: *const c_char, shared_string_type: c_int) -> *const c_char;
    fn get_shared_msg_slave(id: c_int) -> *const c_char;
    fn resp_shared_msg_slave(id: c_int, resp: *const c_char);
    fn slave_has_shared_msg(id: c_int) -> bool;
    fn resize(id: c_int, width: c_int, height: c_int);
    fn toggle_buffer(id: c_int);
    fn is_dirty(id: c_int) -> bool;
    fn set_dirty(id: c_int, value: bool);
    fn set_buffer_ready(id: c_int, is_buffer_ready: bool);
    fn is_buffer_ready(id: c_int) -> bool;
    fn get_w(id: c_int) -> c_int;
    fn get_h(id: c_int) -> c_int;
    fn get_primary_buffer(id: c_int) -> *mut u8;
    fn get_secondary_buffer(id: c_int) -> *mut u8;
    fn lock(id: c_int) -> bool;
    fn lock_timeout(id: c_int, timeout: c_longlong) -> bool;
    fn unlock(id: c_int);
    fn wait_for_buffer_changes(id: c_int);
    fn has_buffer_changes(id: c_int) -> bool;
    fn buffer_status(id: c_int) -> i32;
    fn lock_buffer(id: c_int) -> bool;
    fn unlock_buffer(id: c_int);
}
