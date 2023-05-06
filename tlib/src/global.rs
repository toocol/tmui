use once_cell::sync::Lazy;
use rayon::prelude::*;

#[inline]
pub fn bound<T: Ord>(min: T, val: T, max: T) -> T {
    assert!(max >= min);
    min.max(max.min(val))
}

#[inline]
pub fn round64(d: f64) -> i32 {
    if d >= 0.0 {
        (d + 0.5) as i32
    } else {
        (d - ((d - 1.) as i32) as f64 + 0.5) as i32 + (d - 1.) as i32
    }
}

#[inline]
pub fn round32(d: f32) -> i32 {
    if d >= 0.0 {
        (d + 0.5) as i32
    } else {
        (d - ((d - 1.) as i32) as f32 + 0.5) as i32 + (d - 1.) as i32
    }
}

#[inline]
pub fn fuzzy_compare_64(x: f64, y: f64) -> bool {
    (x - y).abs() * 1000000000000. <= x.abs().min(y.abs())
}

#[inline]
pub fn fuzzy_compare_32(x: f32, y: f32) -> bool {
    (x - y).abs() * 100000. <= x.abs().min(y.abs())
}

#[inline]
pub fn fuzzy_is_null_64(d: f64) -> bool {
    d.abs() <= 0.000000000001
}

#[inline]
pub fn fuzzy_is_null_32(f: f32) -> bool {
    f.abs() <= 0.00001
}

#[inline]
pub fn is_null_64(d: f64) -> bool {
    d == 0.
}

#[inline]
pub fn is_null_32(f: f32) -> bool {
    f == 0.
}

#[inline]
pub fn cpu_nums() -> &'static usize {
    static CPU_NUMS: Lazy<usize> = Lazy::new(|| num_cpus::get());
    &CPU_NUMS
}

pub fn rgba_to_argb_convert(pixels: &mut [u8]) {
    let len = pixels.len();

    if len % 4 != 0 {
        panic!("The length of the pixel data must be a multiple of 4.");
    }

    // Converting u8 type to u32 type using unsafe code would be more efficient.
    let pixels_u32 =
        unsafe { std::slice::from_raw_parts_mut(pixels.as_mut_ptr() as *mut u32, len / 4) };

    // After benchmark testing, at a resolution of 640 * 480,
    // there is no significant difference in time consumption 
    // between rayon parallel computing conversion and direct single thread conversion,
    // which are [19.185 µs 19.237 µs 19.303 µs] and [24.715 µs 24.923 µs 25.139 µs].
    //
    // So 640 * 480 is selected as the threshold for the two calculation methods.
    if len >= 640 * 480 * 4 {
        pixels_u32
            .par_chunks_mut(1.max(pixels_u32.len() / *cpu_nums()))
            .for_each(|chunk| {
                for pixel in chunk.iter_mut() {
                    *pixel = (*pixel << 8) | (*pixel >> 24);
                }
            });
    } else {
        for i in 0..len / 4 {
            let pixel = pixels_u32[i];

            pixels_u32[i] = (pixel << 8) | (pixel >> 24);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rgba_to_argb_convert;

    #[test]
    fn test_rgba_to_argb_convert() {
        let mut pixels = [25u8, 0, 10, 20, 90, 30, 5, 15];
        rgba_to_argb_convert(&mut pixels);
        assert_eq!(pixels, [20, 25, 0, 10, 15, 90, 30, 5]);
    }

    #[test]
    fn test_rgba_to_argb_convert_large() {
        let mut v = vec![1; 3840 * 2160 * 4];
        rgba_to_argb_convert(&mut v);
    }
}
