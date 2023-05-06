use std::mem::size_of;

use criterion::{criterion_group, criterion_main, Criterion};
use windows::Win32::Graphics::Gdi::{BITMAPINFO, BITMAPINFOHEADER, BI_RGB};

fn test_win32_bitmapinfo() {
    let mut bmi = BITMAPINFO::default();
    bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = 1280;
    // Drawing start at top-left.
    bmi.bmiHeader.biHeight = -800;
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;
}

pub fn criterion_wind32window(c: &mut Criterion) {
    c.bench_function("win32_bitmapinfo_test", |b| {
        b.iter(|| test_win32_bitmapinfo())
    });
}

criterion_group!(benches, criterion_wind32window);
criterion_main!(benches);
