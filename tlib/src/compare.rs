use std::cmp::Ordering;

pub type FnCompare<T> = Box<dyn Fn(&T, &T) -> Ordering>;

pub struct Compare<T> {
    f: FnCompare<T>,
}

impl<T> Compare<T> {
    #[inline]
    pub fn new<F: 'static + Fn(&T, &T) -> Ordering>(f: F) -> Self {
        Self { f: Box::new(f) }
    }

    #[inline]
    pub fn cmp(&self, a: &T, b: &T) -> Ordering {
        self.f.as_ref()(a, b)
    }
}
