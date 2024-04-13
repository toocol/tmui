use derivative::Derivative;
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::{collections::HashMap, io::Read};
use tipc::parking_lot::{
    lock_api::{RwLockReadGuard, RwLockWriteGuard},
    RawRwLock, RwLock,
};
use tlib::{
    count_exprs,
    skia_safe::FontMgr,
    typedef::{SkiaFontStyle, SkiaTypeface},
};

macro_rules! external_fonts {
    ($($font:expr),*) => {
        const EXTERNAL_FONTS: [&'static str; count_exprs!($($font),*)] = [$($font),*];
    };
}

external_fonts!("Font Awesome 6 Free-Regular-400.otf");

#[derive(RustEmbed)]
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
        let mut manager = Self::write();

        for font in EXTERNAL_FONTS {
            let data = FontAsset::get(font)
                .expect(&format!("Load ttf file `{}` failed.", font))
                .data;

            let tf = manager
                .system_mgr
                .new_from_data(&data, None)
                .expect(&format!("Make font typeface failed, ttf file: {}.", font));

            manager.fonts.insert(tf.family_name(), tf);
        }
    }

    #[inline]
    pub(crate) fn get(family: &str) -> Option<SkiaTypeface> {
        if let Some(tf) = Self::read().fonts.get(family) {
            Some(tf.clone())
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn make_typeface(family: &str, style: SkiaFontStyle) -> Option<SkiaTypeface> {
        Self::read().system_mgr.match_family_style(family, style)
    }

    #[inline]
    pub fn load_path(path: &str) {
        let mut file = std::fs::File::open(path).expect(&format!("Open file `{}` failed.", path));

        let mut data = vec![];
        file.read_to_end(&mut data)
            .expect(&format!("Read file `{}` failed", path));

        let mut manager = Self::write();

        let tf = manager
            .system_mgr
            .new_from_data(&data, None)
            .expect(&format!(
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
            .expect(&format!("Make customize font typeface failed."));

        manager.fonts.insert(tf.family_name(), tf);
    }
}
