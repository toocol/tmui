use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::{collections::HashMap, io::Read};
use tipc::parking_lot::{
    lock_api::{RwLockReadGuard, RwLockWriteGuard},
    RawRwLock, RwLock,
};
use tlib::{
    count_exprs,
    typedef::{SkiaData, SkiaTypeface},
};

macro_rules! external_fonts {
    ($($font:expr),*) => {
        const EXTERNAL_FONTS: [&'static str; count_exprs!($($font),*)] = [$($font),*];
    };
}

external_fonts!(
    "NotoSansSC-VariableFont_wght.ttf",
    "Font Awesome 6 Free-Regular-400.otf"
);

#[derive(RustEmbed)]
#[folder = "resources/fonts"]
#[include = "*.ttf"]
#[include = "*.otf"]
struct FontAsset;

#[derive(Default)]
pub struct FontManager {
    fonts: HashMap<String, SkiaTypeface>,
}

static mut MGR: Lazy<RwLock<FontManager>> = Lazy::new(|| RwLock::new(FontManager::default()));

impl FontManager {
    #[inline]
    fn write() -> RwLockWriteGuard<'static, RawRwLock, FontManager> {
        unsafe { MGR.write() }
    }

    #[inline]
    fn read() -> RwLockReadGuard<'static, RawRwLock, FontManager> {
        unsafe { MGR.read() }
    }

    #[inline]
    pub(crate) fn load_fonts() {
        for font in EXTERNAL_FONTS {
            let data = FontAsset::get(font)
                .expect(&format!("Load ttf file `{}` failed.", font))
                .data;

            let tf = SkiaTypeface::from_data(unsafe { SkiaData::new_bytes(&data) }, None)
                .expect(&format!("Make font typeface failed, ttf file: {}.", font));

            println!("Load font sucess: {}", tf.family_name());

            Self::write().fonts.insert(tf.family_name(), tf);
        }
    }

    #[inline]
    pub(crate) fn get(name: &str) -> Option<SkiaTypeface> {
        if let Some(tf) = Self::read().fonts.get(name) {
            Some(tf.clone())
        } else {
            None
        }
    }

    #[inline]
    pub fn load(path: &str) {
        let mut file = std::fs::File::open(path).expect(&format!("Open file `{}` failed.", path));

        let mut data = vec![];
        file.read_to_end(&mut data)
            .expect(&format!("Read file `{}` failed", path));

        let tf = SkiaTypeface::from_data(unsafe { SkiaData::new_bytes(&data) }, None).expect(
            &format!("Make customize font typeface failed, ttf file: {}.", path),
        );
        Self::write().fonts.insert(tf.family_name(), tf);
    }
}
