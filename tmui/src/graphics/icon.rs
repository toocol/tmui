use std::path::Path;
use tlib::{figure::ImageBuf, typedef::WinitIcon};

#[derive(Debug, Clone)]
pub struct Icon(ImageBuf);

impl Icon {
    #[inline]
    pub fn from_file<T: AsRef<Path>>(path: T) -> Option<Self> {
        ImageBuf::from_file(path).map(Self)
    }

    /// # Safety
    /// User should guarantee the bytes wiil not outlive the lifetime of the `Icon`
    pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
        Icon(ImageBuf::from_bytes(bytes))
    }
}

impl From<Icon> for WinitIcon {
    #[inline]
    fn from(val: Icon) -> WinitIcon {
        WinitIcon::from_rgba(
            val.0.get_pixels(),
            val.0.width() as u32,
            val.0.height() as u32,
        )
        .unwrap()
    }
}
