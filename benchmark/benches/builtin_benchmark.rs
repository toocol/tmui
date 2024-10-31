use ahash::{AHashMap, AHashSet};
use criterion::{criterion_group, criterion_main, Criterion};
use fxhash::FxHashMap;
use nohash_hasher::IntMap;
use rand::Rng;
use std::collections::HashMap;

const CNT: usize = 1000000;

fn builtin_hash_map_i32() {
    let mut map = HashMap::new();
    for i in 0..CNT {
        map.insert(i, i);
    }
    for i in 0..CNT {
        let _ = map.get(&i);
    }
}

fn non_hash_map_i32() {
    let mut map = IntMap::default();
    for i in 0..CNT {
        map.insert(i, i);
    }
    for i in 0..CNT {
        let _ = map.get(&i);
    }
}

fn builtin_hash_map_string(set: &AHashSet<String>) {
    let mut map = HashMap::new();
    for i in set.iter() {
        map.insert(i, i);
    }
    for i in set.iter() {
        let _ = map.get(&i);
    }
}

fn ahash_map_string(set: &AHashSet<String>) {
    let mut map = AHashMap::default();
    for i in set.iter() {
        map.insert(i, i);
    }
    for i in set.iter() {
        let _ = map.get(&i);
    }
}

fn fxhash_map_string(set: &AHashSet<String>) {
    let mut map = FxHashMap::default();
    for i in set.iter() {
        map.insert(i, i);
    }
    for i in set.iter() {
        let _ = map.get(&i);
    }
}

pub fn builtin_bench(c: &mut Criterion) {
    c.bench_function("builtin_i32_hash_map", |b| {
        b.iter(|| builtin_hash_map_i32())
    });
    c.bench_function("builtin_i32_non_hash_map", |b| {
        b.iter(|| non_hash_map_i32())
    });

    let set = generate_unique_strings(CNT, 10);
    c.bench_function("builtin_string_hash_map", |b| {
        b.iter(|| builtin_hash_map_string(&set))
    });
    c.bench_function("builtin_string_ahash_map", |b| {
        b.iter(|| ahash_map_string(&set))
    });
    c.bench_function("builtin_string_fxhash_map", |b| {
        b.iter(|| fxhash_map_string(&set))
    });
}

criterion_group!(benches, builtin_bench);
criterion_main!(benches);

fn generate_unique_strings(count: usize, length: usize) -> AHashSet<String> {
    let mut set = AHashSet::new();
    let mut rng = rand::thread_rng();

    while set.len() < count {
        let s: String = (0..length)
            .map(|_| rng.gen::<u8>())
            .map(char::from)
            .collect();
        set.insert(s);
    }

    set
}
