use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tlib::{prelude::*, values::ToValue};

pub fn values_i128(num: i128) {
    let value = num.to_value();
    assert_eq!(num, value.get::<i128>());
}

pub fn values_vec(string_list: Vec<String>) {
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

    let len = 30usize;
    let mut string_list = vec![];
    for i in 0..len {
        string_list.push(format!("Hello, {}", i))
    }

    c.bench_function("values-i128-test", |b| b.iter(|| values_i128(black_box(i128::MAX))));
    c.bench_function("values-vec-test", |b| b.iter(|| values_vec(black_box(string_list.clone()))));
    c.bench_function("values-clone-test", |b| b.iter(|| value_clone(&value)));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
