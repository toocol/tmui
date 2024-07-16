use std::{borrow::Cow, io::Read};

pub struct BytesReader {
    data: Cow<'static, [u8]>,
    position: usize,
}

impl BytesReader {
    #[inline]
    pub fn new(data: Cow<'static, [u8]>) -> Self {
        Self { data, position: 0 }
    }
}

impl Read for BytesReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.position >= self.data.len() {
            return Ok(0);
        }

        let remaining_data = &self.data[self.position..];
        let bytes_to_read = remaining_data.len().min(buf.len());
        buf[..bytes_to_read].copy_from_slice(&remaining_data[..bytes_to_read]);
        self.position += bytes_to_read;
        Ok(bytes_to_read)
    }
}
