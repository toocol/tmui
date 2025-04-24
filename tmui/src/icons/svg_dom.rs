use std::io::Read;

use tlib::{figure::Size, skia_safe::FontMgr, typedef::SkiaSvgDom};
use usvg::{fontdb::Database, Options, Tree};

#[derive(Debug, Clone)]
pub struct SvgDom {
    dom: SkiaSvgDom,
    size: Size,
}

impl SvgDom {
    pub fn from_file(path: &str) -> Self {
        let mut file = std::fs::File::open(path).expect("Open file failed");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Read file failed");

        Self::build_from_data(&data)
    }

    #[inline]
    pub fn from_bytes(data: &[u8]) -> Self {
        Self::build_from_data(data)
    }

    #[inline]
    fn build_from_data(data: &[u8]) -> Self {
        let svg_tree = Tree::from_data(data, &Options::default(), &Database::default())
            .expect("Create svg tree failed");
        let size = svg_tree.size();
        let (w, h) = (size.width().ceil() as i32, size.height().ceil() as i32);

        let dom = SkiaSvgDom::from_bytes(data, FontMgr::default()).expect("Create svg dom failed");

        Self {
            dom,
            size: Size::new(w, h),
        }
    }

    #[inline]
    pub fn get_size(&self) -> Size {
        self.size
    }
}

impl AsRef<SkiaSvgDom> for SvgDom {
    #[inline]
    fn as_ref(&self) -> &SkiaSvgDom {
        &self.dom
    }
}
