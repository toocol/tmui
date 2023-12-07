use once_cell::sync::Lazy;
use skia_safe::Font;
use std::{any::Any, cell::{RefCell, Cell}, rc::Rc, sync::Arc};

#[inline]
pub fn bound<T: Ord>(min: T, val: T, max: T) -> T {
    assert!(max >= min);
    min.max(max.min(val))
}

#[inline]
pub fn bound64(min: f64, val: f64, max: f64) -> f64 {
    assert!(max >= min);
    min.max(max.min(val))
}

#[inline]
pub fn bound32(min: f32, val: f32, max: f32) -> f32 {
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

#[inline]
pub fn to_static<T>(t: T) -> &'static T {
    Box::leak(Box::new(t))
}

#[inline]
pub fn same_second(a: u64, b: u64) -> bool {
    a / 1000 == b / 1000
}

#[inline]
pub fn skia_font_clone(src: &Font) -> Font {
    let mut font = Font::default();
    font.set_force_auto_hinting(src.is_force_auto_hinting());
    font.set_embedded_bitmaps(src.is_embedded_bitmaps());
    font.set_subpixel(src.is_subpixel());
    font.set_linear_metrics(src.is_linear_metrics());
    font.set_embolden(src.is_embolden());
    font.set_baseline_snap(src.is_baseline_snap());
    font.set_edging(src.edging());
    font.set_hinting(src.hinting());
    if let Some(typeface) = src.typeface() {
        font.set_typeface(typeface);
    }
    font.set_size(src.size());
    font.set_scale_x(src.scale_x());
    font.set_skew_x(src.skew_x());
    font
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any_boxed(self: Box<Self>) -> Box<dyn Any>;
}
#[macro_export]
macro_rules! impl_as_any {
    ( $st:ident ) => {
        impl AsAny for $st {
            #[inline]
            fn as_any(&self) -> &dyn Any {
                self
            }

            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            #[inline]
            fn as_any_boxed(self: Box<Self>) -> Box<dyn Any> {
                self
            }
        }
    };
}
impl_as_any!(i8);
impl_as_any!(u8);
impl_as_any!(i16);
impl_as_any!(u16);
impl_as_any!(i32);
impl_as_any!(u32);
impl_as_any!(i64);
impl_as_any!(u64);
impl_as_any!(i128);
impl_as_any!(u128);
impl_as_any!(f32);
impl_as_any!(f64);
impl_as_any!(String);

#[macro_export]
macro_rules! nonnull_ref {
    ( $st:ident ) => {
        unsafe { $st.as_ref().unwrap().as_ref() }
    };
    ( $st:expr ) => {
        unsafe { $st.as_ref().unwrap().as_ref() }
    };
}

#[macro_export]
macro_rules! nonnull_mut {
    ( $st:ident ) => {
        unsafe { $st.as_mut().unwrap().as_mut() }
    };
    ( $st:expr ) => {
        unsafe { $st.as_mut().unwrap().as_mut() }
    };
}

#[macro_export]
macro_rules! ptr_ref {
    ( $st:ident ) => {
        unsafe { $st.as_ref().unwrap() }
    };
    ( $st:expr ) => {
        unsafe { $st.as_ref().unwrap() }
    };
}

#[macro_export]
macro_rules! ptr_mut {
    ( $st:ident ) => {
        unsafe { $st.as_mut().unwrap() }
    };
    ( $st:expr ) => {
        unsafe { $st.as_mut().unwrap() }
    };
}

pub trait SemanticExt: Sized {
    #[inline]
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    #[inline]
    fn ref_cell(self) -> RefCell<Self> {
        RefCell::new(self)
    }

    #[inline]
    fn cell(self) -> Cell<Self> {
        Cell::new(self)
    }

    #[inline]
    fn rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    #[inline]
    fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
impl<T: Sized> SemanticExt for T {}

pub trait CreateBy<T> {
    fn create_by(t: T) -> Self;
}

#[cfg(test)]
mod tests {
    use super::SemanticExt;

    #[test]
    fn test_semantic_ext() {
        let p = 1.boxed();
        let p = p.ref_cell();
        let p = p.cell();
        let p = p.rc();
        let _ = p.arc();
    }
}