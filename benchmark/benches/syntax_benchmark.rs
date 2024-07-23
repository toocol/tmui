use std::{any::Any, time::Duration};
use criterion::{criterion_group, criterion_main, Criterion};

struct Inner {
    cnt: u64,
}
impl Inner {
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline]
    fn do_something(&mut self) {
        self.cnt += 1;
    }
}

struct Outer {
    inner: Inner
}
impl Outer {
    #[inline]
    fn new() -> Self {
        Self { inner: Inner { cnt: 0 } }
    }

    #[inline]
    fn get_inner_mut(&mut self) -> &mut Inner {
        self.inner.as_any_mut().downcast_mut::<Inner>().unwrap()
    }
}

fn test_own() {
    let mut outer = Outer::new(); 
    let inner = outer.get_inner_mut();

    std::thread::sleep(Duration::from_millis(100));
    for _ in 0..1000000 {
        inner.do_something();
    }
}

fn test_un_own() {
    let mut outer = Outer::new(); 

    std::thread::sleep(Duration::from_millis(100));
    for _ in 0..1000000 {
        outer.get_inner_mut().do_something();
    }
}

pub fn criterion_values(c: &mut Criterion) {
    c.bench_function("syntax_own", |b| b.iter(|| test_own()));
    c.bench_function("syntax_un_own", |b| b.iter(|| test_un_own()));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
