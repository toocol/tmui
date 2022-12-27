use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tlib::{prelude::*, values::ToValue};

pub fn values(len: usize) {
    let mut string_list = vec![];
    for i in 0..len {
        string_list.push(format!("Hello, {}", i))
    }
    let value = string_list.to_value();
    assert_eq!(string_list, value.get::<Vec<String>>())
}

pub fn value_clone(value: &Value) {
    let _ = value.clone();
}

pub fn criterion_values(c: &mut Criterion) {
    let value = (
        "Benchmark test value clone param",
        i32::MAX,
        vec!["value1", "value2", "value3", "value4", "value5"],
        i128::MAX,
        i128::MIN,
    )
        .to_value();

    c.bench_function("values-vec-test", |b| b.iter(|| values(black_box(30))));
    c.bench_function("values-clone-test", |b| b.iter(|| value_clone(&value)));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
