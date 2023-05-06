use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tlib::global::rgba_to_argb_convert;

fn my_benchmark(c: &mut Criterion) {
    let mut v = vec![0; 3840 * 2160 * 4];
    c.bench_function("color_format_convert_3840_2160", |b| {
        // Benchmark the parallel function call using criterion's `black_box` method
        b.iter(|| rgba_to_argb_convert(black_box(&mut v)))
    });

    v = vec![0; 1920 * 1080 * 4];
    c.bench_function("color_format_convert_1920_1080", |b| {
        // Benchmark the parallel function call using criterion's `black_box` method
        b.iter(|| rgba_to_argb_convert(black_box(&mut v)))
    });

    v = vec![0; 1600 * 900 * 4];
    c.bench_function("color_format_convert_1600_900", |b| {
        // Benchmark the parallel function call using criterion's `black_box` method
        b.iter(|| rgba_to_argb_convert(black_box(&mut v)))
    });

    v = vec![0; 1280 * 720 * 4];
    c.bench_function("color_format_convert_1280_720", |b| {
        // Benchmark the parallel function call using criterion's `black_box` method
        b.iter(|| rgba_to_argb_convert(black_box(&mut v)))
    });

    v = vec![0; 640 * 480 * 4];
    c.bench_function("color_format_convert_640_480", |b| {
        // Benchmark the parallel function call using criterion's `black_box` method
        b.iter(|| rgba_to_argb_convert(black_box(&mut v)))
    });
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);