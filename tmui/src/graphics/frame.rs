#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Frame {
    /// Frames of the same second have the same id.
    id: u64,
    /// Nth frame in the same second, count from 1.
    nth: u8,
    /// The timestamp when the frame was created.
    timestamp: u64,
}

impl Frame {
    #[inline]
    pub(crate) fn empty_frame() -> Frame {
        Frame {
            id: 0,
            nth: 0,
            timestamp: 0,
        }
    }

    #[inline]
    pub(crate) fn next(&self) -> Frame {
        todo!()
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }

    #[inline]
    pub fn nth(&self) -> u8 {
        self.nth
    }

    #[inline]
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
