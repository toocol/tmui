use std::fmt::Debug;

pub trait CellIndex: Debug {
    fn index(&self) -> usize;
}

impl CellIndex for usize {
    #[inline]
    fn index(&self) -> usize {
        *self
    }
}