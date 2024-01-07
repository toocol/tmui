use std::path::Path;
use tlib::{figure::ImageBuf, typedef::WinitIcon};

#[derive(Debug, Clone)]
pub struct Icon(ImageBuf);

impl Icon {
    #[inline]
    pub fn from_file<T: AsRef<Path>>(path: T) -> Option<Self> {
        match ImageBuf::from_file(path) {
            Some(buf) => Some(Self(buf)),
            None => None,
        }
    }
}

impl Into<WinitIcon> for Icon {
    #[inline]
    fn into(self) -> tlib::winit::window::Icon {
        tlib::winit::window::Icon::from_rgba(
            self.0
                .raw_file()
                .expect("Pixels bytes of icon was none.")
                .to_vec(),
            self.0.width() as u32,
            self.0.height() as u32,
        ).unwrap()
    }
}
