use rayon::prelude::*;
use crate::global::cpu_nums;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorFormat {
    Rgba8888,
    Argb8888,
}

pub struct ColorConvert;
impl ColorConvert {
    #[inline]
    pub fn convert(pixels: &mut [u8], from: ColorFormat, to: ColorFormat) {
        match (from, to) {
            (ColorFormat::Rgba8888, ColorFormat::Argb8888) => rgba_to_argb_convert(pixels),
            _ => unimplemented!()
        }
    }
}

#[inline]
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
        for p in pixels_u32.iter_mut().take(len / 4) {
            let pixel = *p;

            *p = (pixel << 8) | (pixel >> 24);
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
        let mut pixels = vec![0; 3840 * 2160 * 4];
        for i in (0..pixels.len()).step_by(4) {
            pixels[i] = 0;
            pixels[i + 1] = 5;
            pixels[i + 2] = 10;
            pixels[i + 3] = 15;
        }
        let mut should_be = vec![0; 3840 * 2160 * 4];
        for i in (0..should_be.len()).step_by(4) {
            should_be[i] = 15;
            should_be[i + 1] = 0;
            should_be[i + 2] = 5;
            should_be[i + 3] = 10;
        }
        rgba_to_argb_convert(&mut pixels);
        assert_eq!(pixels, should_be);
    }
}