use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use tlib::{
    figure::{CoordRect, CoordRegion, FRect},
    prelude::Coordinate,
};

fn generate_random_rects(count: usize) -> CoordRegion {
    let mut rng = rand::thread_rng();
    let mut region = CoordRegion::new();

    for _ in 0..count {
        let x = rng.gen_range(0.0..1000.0);
        let y = rng.gen_range(0.0..1000.0);
        let width = rng.gen_range(20.0..80.0);
        let height = rng.gen_range(20.0..80.0);
        let rect = CoordRect::new(FRect::new(x, y, width, height), Coordinate::World);
        region.add_rect(rect);
    }

    region
}

pub fn benchmark_region_merge(c: &mut Criterion) {
    c.bench_function("region_merge_10000", |b| {
        b.iter(|| {
            let mut region = generate_random_rects(10000);
            region.merge_all();
        })
    });
}

criterion_group!(benches, benchmark_region_merge);
criterion_main!(benches);
