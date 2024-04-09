use rust_embed::RustEmbed;
use std::collections::HashMap;
use tlib::typedef::{SkiaData, SkiaTypeface};

#[derive(RustEmbed)]
#[folder = "resources/fonts"]
#[include = "*.ttf"]
pub struct FontAsset;

#[derive(Default)]
pub(crate) struct FontManager {
    fonts: HashMap<String, SkiaTypeface>,
}

impl FontManager {
    pub(crate) fn load_fonts(&mut self) {
        for font in DEFAULT_FONTS {
            let data = FontAsset::get(font)
                .expect(&format!("Load ttf file `{}` failed.", font))
                .data;

            let tf = SkiaTypeface::from_data(unsafe { SkiaData::new_bytes(&data) }, None)
                .expect(&format!("Make font typeface failed, tty_file: {}.", font));
            self.fonts.insert(font.to_string(), tf);
        }
    }

    pub(crate) fn get_font(&self, name: &str) -> Option<SkiaTypeface> {
        if let Some(tf) = self.fonts.get(name) {
            Some(tf.clone())
        } else {
            None
        }
    }
}

const DEFAULT_FONTS: [&'static str; 1] = ["NotoSansSC-VariableFont_wght.ttf"];

pub enum Fonts {
    /// Simplified Chinese
    NotoSansSC,
}
impl Fonts {
    pub(crate) fn name(&self) -> &'static str {
        todo!()
    }
}
