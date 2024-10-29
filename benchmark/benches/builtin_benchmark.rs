use std::collections::HashMap;
use criterion::{criterion_group, criterion_main, Criterion};
use nohash_hasher::IntMap;

const CNT: usize = 1000000;

fn builtin_hash_map() {
    let mut map = HashMap::new();
    for i in 0..CNT {
        map.insert(i, i);
    }
    for i in 0..CNT {
        let _ = map.get(&i);
    }
}

fn non_hash_map() {
    let mut map = IntMap::default();
    for i in 0..CNT {
        map.insert(i, i);
    }
    for i in 0..CNT {
        let _ = map.get(&i);
    }
}

pub fn builtin_bench(c: &mut Criterion) {
    c.bench_function("builtin_hash_map", |b| b.iter(|| builtin_hash_map()));
    c.bench_function("builtin_non_hash_map", |b| b.iter(|| non_hash_map()));
}

criterion_group!(benches, builtin_bench);
criterion_main!(benches);
