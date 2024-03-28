use criterion::{criterion_group, criterion_main, Criterion};
use tlib::{
    figure::{Font, FontCalculation},
    typedef::SkiaFont,
};

fn font_calc_test(font: &SkiaFont) {
    let _ = font.calc_font_dimension();
}

pub fn font_calc(c: &mut Criterion) {
    let font = Font::with_family("Courier New");
    let skia_font: SkiaFont = font.into();

    c.bench_function("font_calc_test", |b| b.iter(|| font_calc_test(&skia_font)));
}

criterion_group!(benches, font_calc);
criterion_main!(benches);
