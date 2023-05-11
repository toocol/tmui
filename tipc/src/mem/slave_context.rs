use super::MemContext;

pub(crate) struct SlaveContext {

}

impl SlaveContext {

}

impl MemContext for SlaveContext {
    fn get_primary_buffer(&self) -> *mut u8 {
        todo!()
    }

    fn get_secodary_buffer(&self) -> *mut u8 {
        todo!()
    }

    fn send_shared_message(&mut self, message: &str, message_type: i32) -> String {
        todo!()
    }

    fn try_recv_shared_message(&self) -> (String, i32) {
        todo!()
    }

    fn response_shared_message(&mut self) {
        todo!()
    }
}