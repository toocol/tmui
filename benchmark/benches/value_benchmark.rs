use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tlib::values::ToValue;

pub fn values(len: usize) {
    let mut string_list = vec![];
    for i in 0..len {
        string_list.push(format!("Hello, {}", i))
    }
    let value = string_list.to_value();
    assert_eq!(string_list, value.get::<Vec<String>>())
}

pub fn criterion_values(c: &mut Criterion) {
    c.bench_function("test-values-vec", |b| b.iter(|| values(black_box(30))));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
