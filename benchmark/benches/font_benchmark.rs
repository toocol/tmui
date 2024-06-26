use criterion::{criterion_group, criterion_main, Criterion};
use tlib::typedef::SkiaTypeface;
use tmui::font::{Font, FontCalculation, FontTypeface};

fn font_calc_test(font: &Font) {
    let _ = font.calc_font_dimension();
}

pub fn typeface_tmui(font: &Font, typeface: &FontTypeface) {
    let _ = typeface.to_skia_typeface(font);
}

pub fn typeface_skia(typeface: &SkiaTypeface) {
    let _ = typeface.clone();
}

pub fn font_calc(c: &mut Criterion) {
    let font = Font::with_families(&vec!["Courier New"]);
    let typeface = FontTypeface::new("Courier New");
    let skia_typeface = typeface.to_skia_typeface(&font).unwrap();

    c.bench_function("font_calc_test", |b| b.iter(|| font_calc_test(&font)));
    c.bench_function("font_typeface_convert", |b| {
        b.iter(|| typeface_tmui(&font, &typeface))
    });
    c.bench_function("font_typeface_clone", |b| {
        b.iter(|| typeface_skia(&skia_typeface))
    });
}

criterion_group!(benches, font_calc);
criterion_main!(benches);
