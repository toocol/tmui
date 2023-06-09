use skia_safe;
use std::path::PathBuf;

/// Convinent wrapper to create [`skia_safe::Image`], create image by system file.
#[derive(Debug, Clone)]
pub struct Image {
    file: Vec<u8>,
    image: skia_safe::Image,
}

impl Image {
    /// Create image by file.
    ///
    /// SAFETY: filed `image`'s lifetime was equals to origin file bytes `file`.
    #[inline]
    pub fn from_file(path: PathBuf) -> Self {
        let file = std::fs::read(path).unwrap();
        let data = unsafe { skia_safe::Data::new_bytes(&file) };
        let image = skia_safe::Image::from_encoded(data).unwrap();

        Self { file, image }
    }

    #[inline]
    pub fn image_ref(&self) -> &skia_safe::Image {
        &self.image
    }

    #[inline]
    pub fn image_mut(&mut self) -> &mut skia_safe::Image {
        &mut self.image
    }
}

impl AsRef<skia_safe::Image> for Image {
    #[inline]
    fn as_ref(&self) -> &skia_safe::Image {
        &self.image
    }
}

impl AsMut<skia_safe::Image> for Image {
    #[inline]
    fn as_mut(&mut self) -> &mut skia_safe::Image {
        &mut self.image
    }
}
