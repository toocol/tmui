use std::{
    ffi::{c_char, c_int, c_longlong, CStr, CString},
    slice,
};

use crate::ipc_event::{CIpcEvent, IpcEvent};

const IPC_NUM_NATIVE_EVT_MSG_SIZE: usize = 1024;

#[derive(Debug)]
pub struct NativeEvent {
    pub action_name: String,
    pub params: Option<Vec<String>>,
}
impl NativeEvent {
    pub fn from_bytes(bytes: *mut u8) -> Self {
        let mut evt_msg;

        unsafe {
            let bytes = slice::from_raw_parts(bytes, IPC_NUM_NATIVE_EVT_MSG_SIZE);
            let mut len = 0usize;
            for i in bytes.iter() {
                if *i == 0 {
                    break;
                }
                len += 1;
            }
            evt_msg = vec![0u8; len];

            evt_msg.copy_from_slice(&bytes[0..len]);
        }

        let evt_msg = String::from_utf8(evt_msg.to_vec())
            .expect("Transfer `evt_msg` to utf-8 string failed.");

        let mut action_name = String::new();
        let mut params = vec![];

        let mut idx = 0;
        for s in evt_msg.split(";").into_iter() {
            if idx == 0 {
                action_name = s.to_string();
            } else {
                params.push(s.to_string());
            }
            idx += 1;
        }

        let native_evt = NativeEvent {
            action_name,
            params: if params.len() == 0 {
                None
            } else {
                Some(params)
            },
        };

        native_evt
    }
}

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
        unsafe { send_event_master(id, evt) }
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

    // Slave
    /// Acquire next avaliable key of connection.
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

    /// Terminate the shared memory connection by key.
    #[inline]
    pub fn terminate_at(key: i32) -> bool {
        unsafe {
            return terminate_at(key);
        }
    }

    /// Judge whether the shared memory corresponding to the key is connected.
    #[inline]
    pub fn is_connected(key: i32) -> bool {
        unsafe {
            return is_connected(key);
        }
    }

    /// Slave send the event.
    #[inline]
    pub fn send_event_slave(id: i32, evt: CIpcEvent) {
        unsafe { send_event_slave(id, evt) }
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

    /// Block send msg to shared memory server side with response.
    #[inline]
    pub fn send_msg(key: i32, msg: &str, shared_string_type: i32) -> String {
        unsafe {
            let c_str = CString::new(msg).unwrap();
            let res = send_msg(key, c_str.as_ptr(), shared_string_type);
            let res = CString::from_raw(res as *mut c_char)
                .to_str()
                .unwrap()
                .to_string();
            return res;
        }
    }

    /// Determain whether has native events.
    #[inline]
    pub fn has_events(key: i32) -> bool {
        unsafe { has_native_events(key) }
    }

    /// Get the native event.
    #[inline]
    pub fn get_native_event(key: i32) -> NativeEvent {
        unsafe {
            let bytes = get_native_event(key);
            NativeEvent::from_bytes(bytes)
        }
    }

    /// Drop the native event.
    #[inline]
    pub fn drop_native_event(key: i32) {
        unsafe { drop_native_event(key) }
    }

    /// Resize the teminal emulator.
    #[inline]
    pub fn resize(key: i32, width: i32, height: i32) {
        unsafe {
            resize(key, width, height);
        }
    }

    /// Toggle buffer between primary/secondary buffer.
    #[inline]
    pub fn toggle_buffer(key: c_int) {
        unsafe { toggle_buffer(key) }
    }

    /// When the native image buffer was changed, the property of dirty was true.
    #[inline]
    pub fn is_dirty(key: i32) -> bool {
        unsafe { is_dirty(key) }
    }

    /// Client request redraw the native image buffer.
    #[inline]
    pub fn redraw(key: i32, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            redraw(key, x, y, w, h);
        }
    }

    /// Set the native image buffer was dirty.
    #[inline]
    pub fn set_dirty(key: i32, value: bool) {
        unsafe {
            set_dirty(key, value);
        }
    }

    /// Set true when the native image buffer was rendering completed, set false otherwise.
    #[inline]
    pub fn set_buffer_ready(key: i32, is_buffer_ready: bool) {
        unsafe {
            set_buffer_ready(key, is_buffer_ready);
        }
    }

    /// Get the native image buffer redering state.
    #[inline]
    pub fn is_buffer_ready(key: i32) -> bool {
        unsafe { is_buffer_ready(key) }
    }

    /// Get the width of native image buffer.
    #[inline]
    pub fn get_w(key: i32) -> i32 {
        unsafe { get_w(key) }
    }

    /// Get the height of native image buffer.
    #[inline]
    pub fn get_h(key: i32) -> i32 {
        unsafe { get_h(key) }
    }

    /// Tell terminal emulator to request focus or not.
    #[inline]
    pub fn request_focus(key: i32, is_focus: bool, timestamp: i64) -> bool {
        unsafe { request_focus(key, is_focus, timestamp) }
    }

    /// Get the primary native image buffer.
    #[inline]
    pub fn get_primary_buffer(key: i32) -> *mut u8 {
        unsafe { get_primary_buffer(key) }
    }

    /// Get the secondary native image buffer.
    #[inline]
    pub fn get_secondary_buffer(key: i32) -> *mut u8 {
        unsafe { get_secondary_buffer(key) }
    }

    /// Thread lock the common resource.
    #[inline]
    pub fn lock(key: i32) -> bool {
        unsafe { lock(key) }
    }

    /// Thread lock the common resource with timeout.
    #[inline]
    pub fn lock_timeout(key: i32, timeout: i64) -> bool {
        unsafe { lock_timeout(key, timeout) }
    }

    /// Unlock the common resource.
    #[inline]
    pub fn unlock(key: i32) {
        unsafe { unlock(key) }
    }

    /// Blocking wait for native image buffer changes.
    #[inline]
    pub fn wait_for_buffer_changes(key: i32) {
        unsafe { wait_for_buffer_changes(key) }
    }

    /// Whether the native image buffer has changed.
    #[inline]
    pub fn has_buffer_changes(key: i32) -> bool {
        unsafe { has_buffer_changes(key) }
    }

    /// Get current native image buffer status
    #[inline]
    pub fn buffer_status(key: i32) -> i32 {
        unsafe { buffer_status(key) }
    }

    /// Thread lock the primary native image buffer.
    #[inline]
    pub fn lock_buffer(key: i32) -> bool {
        unsafe { lock_buffer(key) }
    }

    /// Thread unlock the primary native image buffer.
    #[inline]
    pub fn unlock_buffer(key: i32) {
        unsafe {
            unlock_buffer(key);
        }
    }

    #[inline]
    pub fn fire_mouse_pressed_event(
        key: i32,
        n_press: i32,
        x: f64,
        y: f64,
        buttons: i32,
        modifiers: i32,
        timestamp: i64,
    ) -> bool {
        unsafe { fire_mouse_pressed_event(key, n_press, x, y, buttons, modifiers, timestamp) }
    }

    #[inline]
    pub fn fire_mouse_released_event(
        key: i32,
        x: f64,
        y: f64,
        buttons: i32,
        modifiers: i32,
        timestamp: i64,
    ) -> bool {
        unsafe { fire_mouse_released_event(key, x, y, buttons, modifiers, timestamp) }
    }

    #[inline]
    pub fn fire_mouse_clicked_event(
        key: i32,
        x: f64,
        y: f64,
        buttons: i32,
        modifiers: i32,
        click_count: i32,
        timestamp: i64,
    ) -> bool {
        unsafe { fire_mouse_clicked_event(key, x, y, buttons, modifiers, click_count, timestamp) }
    }

    #[inline]
    pub fn fire_mouse_entered_event(
        key: i32,
        x: f64,
        y: f64,
        modifiers: i32,
        timestamp: i64,
    ) -> bool {
        unsafe { fire_mouse_entered_event(key, x, y, modifiers, timestamp) }
    }

    #[inline]
    pub fn fire_mouse_exited_event(key: i32, modifiers: i32, timestamp: i64) -> bool {
        unsafe { fire_mouse_exited_event(key, modifiers, timestamp) }
    }

    #[inline]
    pub fn fire_mouse_move_event(key: i32, x: f64, y: f64, modifiers: i32, timestamp: i64) -> bool {
        unsafe { fire_mouse_move_event(key, x, y, modifiers, timestamp) }
    }

    #[inline]
    pub fn fire_mouse_wheel_event(
        key: i32,
        x: f64,
        y: f64,
        amount: f64,
        modifiers: i32,
        timestamp: i64,
    ) -> bool {
        unsafe { fire_mouse_wheel_event(key, x, y, amount, modifiers, timestamp) }
    }

    #[inline]
    pub fn fire_key_pressed_event(
        key: i32,
        characters: &str,
        key_code: i32,
        modifiers: i32,
        timestamp: i64,
    ) -> bool {
        unsafe {
            let characters = CString::new(characters).unwrap();
            fire_key_pressed_event(key, characters.as_ptr(), key_code, modifiers, timestamp)
        }
    }

    #[inline]
    pub fn fire_key_released_event(
        key: i32,
        characters: &str,
        key_code: i32,
        modifiers: i32,
        timestamp: i64,
    ) -> bool {
        unsafe {
            let characters = CString::new(characters).unwrap();
            fire_key_released_event(key, characters.as_ptr(), key_code, modifiers, timestamp)
        }
    }

    #[inline]
    pub fn fire_key_typed_event(
        key: i32,
        characters: &str,
        key_code: i32,
        modifiers: i32,
        timestamp: i64,
    ) -> bool {
        unsafe {
            let characters = CString::new(characters).unwrap();
            fire_key_typed_event(key, characters.as_ptr(), key_code, modifiers, timestamp)
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

    // Slave
    fn next_key() -> c_int;
    fn connect_to(name: *const c_char) -> c_int;
    fn terminate_at(key: c_int) -> bool;
    fn is_connected(key: c_int) -> bool;
    fn send_event_slave(id: c_int, evt: CIpcEvent);
    fn recv_from_master(id: c_int) -> CIpcEvent;
    fn try_recv_from_master(id: c_int) -> CIpcEvent;
    fn send_msg(key: c_int, msg: *const c_char, shared_string_type: c_int) -> *const c_char;
    fn has_native_events(key: c_int) -> bool;
    fn get_native_event(key: c_int) -> *mut u8;
    fn drop_native_event(key: c_int);
    fn resize(key: c_int, width: c_int, height: c_int);
    fn toggle_buffer(key: c_int);
    fn is_dirty(key: c_int) -> bool;
    fn redraw(key: c_int, x: c_int, y: c_int, w: c_int, h: c_int);
    fn set_dirty(key: c_int, value: bool);
    fn set_buffer_ready(key: c_int, is_buffer_ready: bool);
    fn is_buffer_ready(key: c_int) -> bool;
    fn get_w(key: c_int) -> c_int;
    fn get_h(key: c_int) -> c_int;
    fn request_focus(key: c_int, is_focus: bool, timestamp: c_longlong) -> bool;
    fn get_primary_buffer(key: c_int) -> *mut u8;
    fn get_secondary_buffer(key: c_int) -> *mut u8;
    fn lock(key: c_int) -> bool;
    fn lock_timeout(key: c_int, timeout: c_longlong) -> bool;
    fn unlock(key: c_int);
    fn wait_for_buffer_changes(key: c_int);
    fn has_buffer_changes(key: c_int) -> bool;
    fn buffer_status(key: c_int) -> i32;
    fn lock_buffer(key: c_int) -> bool;
    fn unlock_buffer(key: c_int);
    fn fire_mouse_pressed_event(
        key: c_int,
        n_press: i32,
        x: f64,
        y: f64,
        buttons: c_int,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_mouse_released_event(
        key: c_int,
        x: f64,
        y: f64,
        buttons: c_int,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_mouse_clicked_event(
        key: c_int,
        x: f64,
        y: f64,
        buttons: c_int,
        modifiers: c_int,
        click_count: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_mouse_entered_event(
        key: c_int,
        x: f64,
        y: f64,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_mouse_exited_event(key: c_int, modifiers: c_int, timestamp: c_longlong) -> bool;
    fn fire_mouse_move_event(
        key: c_int,
        x: f64,
        y: f64,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_mouse_wheel_event(
        key: c_int,
        x: f64,
        y: f64,
        amount: f64,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_key_pressed_event(
        key: c_int,
        characters: *const c_char,
        key_code: c_int,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_key_released_event(
        key: c_int,
        characters: *const c_char,
        key_code: c_int,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
    fn fire_key_typed_event(
        key: c_int,
        characters: *const c_char,
        key_code: c_int,
        modifiers: c_int,
        timestamp: c_longlong,
    ) -> bool;
}
