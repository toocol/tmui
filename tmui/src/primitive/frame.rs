use tlib::{global::same_second, utils::Timestamp};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Frame {
    /// Frames of the same second have the same id.
    id: u64,
    /// Nth frame in the same second, count from 1.
    nth: u16,
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
        let now = Timestamp::now().as_millis();
        let (id, nth) = if same_second(now, self.timestamp) {
            (self.id, self.nth + 1)
        } else {
            (self.id + 1, 1)
        };
        Frame {
            id,
            nth,
            timestamp: now,
        }
    }

    /// Frames of the same second have the same id.
    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Nth frame in the same second, count from 1.
    #[inline]
    pub fn nth(&self) -> u16 {
        self.nth
    }

    /// The timestamp when the frame was created.
    #[inline]
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
