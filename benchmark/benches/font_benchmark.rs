use criterion::{criterion_group, criterion_main, Criterion};
use tmui::font::{Font, FontCalculation};

fn font_calc_test(font: &Font) {
    let _ = font.calc_font_dimension();
}

pub fn font_calc(c: &mut Criterion) {
    let font = Font::with_family(vec!["Courier New"]);

    c.bench_function("font_calc_test", |b| b.iter(|| font_calc_test(&font)));
}

criterion_group!(benches, font_calc);
criterion_main!(benches);
