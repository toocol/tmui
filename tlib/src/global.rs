use once_cell::sync::Lazy;
use std::any::Any;

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

#[inline]
pub fn to_static<T>(t: T) -> &'static T {
    Box::leak(Box::new(t))
}

#[inline]
pub fn same_second(a: u64, b: u64) -> bool {
    a / 1000 == b / 1000
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
