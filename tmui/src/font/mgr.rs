use derivative::Derivative;
use once_cell::sync::Lazy;
use std::{collections::HashMap, io::Read};
use tipc::parking_lot::{
    lock_api::{RwLockReadGuard, RwLockWriteGuard},
    RawRwLock, RwLock,
};
use tlib::{
    skia_safe::FontMgr,
    typedef::{SkiaFontStyle, SkiaTypeface},
};

#[cfg(font_awesome)]
macro_rules! external_fonts {
    ($($font:expr),*) => {
        use tlib::count_exprs;
        const EXTERNAL_FONTS: [&'static str; count_exprs!($($font),*)] = [$($font),*];
    };
}

#[cfg(font_awesome)]
external_fonts!(
    "Font Awesome 6 Brands-Regular-400.otf",
    "Font Awesome 6 Free-Regular-400.otf",
    "Font Awesome 6 Free-Solid-900.otf"
);

#[cfg(font_awesome)]
#[derive(rust_embed::RustEmbed)]
#[folder = "resources/fonts"]
#[include = "*.ttf"]
#[include = "*.otf"]
struct FontAsset;

#[derive(Derivative)]
#[derivative(Default)]
pub struct FontManager {
    system_mgr: FontMgr,
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
        #[cfg(font_awesome)]
        {
            let mut manager = Self::write();

            for font in EXTERNAL_FONTS {
                let data = FontAsset::get(font)
                    .unwrap_or_else(|| panic!("Load ttf file `{}` failed.", font))
                    .data;

                let tf = manager
                    .system_mgr
                    .new_from_data(&data, None)
                    .unwrap_or_else(|| panic!("Make font typeface failed, ttf file: {}.", font));

                manager.fonts.insert(tf.family_name(), tf);
            }
        }
    }

    #[inline]
    pub(crate) fn get(family: &str) -> Option<SkiaTypeface> {
        Self::read().fonts.get(family).cloned()
    }

    #[inline]
    pub(crate) fn make_typeface(family: &str, style: SkiaFontStyle) -> Option<SkiaTypeface> {
        Self::read().system_mgr.match_family_style(family, style)
    }

    #[inline]
    pub fn load_file(path: &str) {
        let mut file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Open file `{}` failed.", path));

        let mut data = vec![];
        file.read_to_end(&mut data)
            .unwrap_or_else(|_| panic!("Read file `{}` failed", path));

        let mut manager = Self::write();

        let tf = manager
            .system_mgr
            .new_from_data(&data, None)
            .unwrap_or_else(|| panic!(
                "Make customize font typeface failed, ttf file: {}.",
                path
            ));

        manager.fonts.insert(tf.family_name(), tf);
    }

    #[inline]
    pub fn load_data(data: &[u8]) {
        let mut manager = Self::write();

        let tf = manager
            .system_mgr
            .new_from_data(data, None)
            .unwrap_or_else(|| panic!("Make customize font typeface failed."));

        manager.fonts.insert(tf.family_name(), tf);
    }
}
