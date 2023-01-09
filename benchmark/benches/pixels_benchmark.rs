use criterion::{criterion_group, criterion_main, Criterion};

fn test_pixels_to_argb(pixels: &mut [u8]) {
    pixels.reverse();
}

pub fn criterion_values(c: &mut Criterion) {
    let mut pixels_bgra8888 = vec![0u8; 2560 * 1440 * 4];
    c.bench_function("pixels_reverse_test", |b| b.iter(|| test_pixels_to_argb(&mut pixels_bgra8888)));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
