use super::svg_attr::SvgAttr;
use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::RustEmbed;
use std::borrow::Cow;

lazy_static! {
    static ref WIDTH_REGEX: Regex = Regex::new(r#"width="[^"]*""#).unwrap();
    static ref HEIGHT_REGEX: Regex = Regex::new(r#"height="[^"]*""#).unwrap();
    static ref FILL_REGEX: Regex = Regex::new(r#"fill="\#[0-9a-fA-F]{6}""#).unwrap();
}

pub struct SvgStr<'a> {
    data: &'a str,
}

impl<'a> SvgStr<'a> {
    #[inline]
    pub fn new(data: &'a str) -> Self {
        SvgStr { data }
    }

    #[inline]
    pub fn get<T: RustEmbed>(file_path: &str, attr: SvgAttr) -> Option<String> {
        let file = T::get(file_path).to_owned()?;
        let data = match file.data {
            Cow::Borrowed(bytes) => std::str::from_utf8(bytes).ok(),
            Cow::Owned(ref bytes) => std::str::from_utf8(bytes).ok(),
        };

        data.map(|data| SvgStr { data }.with_attr(attr))
    }

    #[inline]
    pub fn with_attr(&self, attr: SvgAttr) -> String {
        let data = WIDTH_REGEX.replace(self.data, &format!(r#"width="{}""#, attr.width()));

        let data = HEIGHT_REGEX.replace(
            match data {
                Cow::Borrowed(str) => str,
                Cow::Owned(ref str) => str.as_str(),
            },
            &format!(r#"height="{}""#, attr.height()),
        );

        let data = FILL_REGEX.replace(
            match data {
                Cow::Borrowed(str) => str,
                Cow::Owned(ref str) => str.as_str(),
            },
            &format!(r#"fill="{}""#, attr.color().hexcode()),
        );

        data.to_string()
    }
}
