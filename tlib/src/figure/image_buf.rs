use skia_safe;
use std::path::Path;

/// Convinent wrapper to create [`skia_safe::Image`], create image by system file.
#[derive(Debug, Clone)]
pub struct ImageBuf {
    file: Option<Vec<u8>>,
    raw_bytes: Option<*const [u8]>,
    image: skia_safe::Image,
}

impl ImageBuf {
    /// Create image by file.
    ///
    /// SAFETY: field `image`'s lifetime was equals to origin file bytes `file`.
    #[inline]
    pub fn from_file<T: AsRef<Path>>(path: T) -> Option<Self> {
        let file = if let Ok(file) = std::fs::read(path) {
            file
        } else {
            return None;
        };
        let data = unsafe { skia_safe::Data::new_bytes(&file) };
        let image = skia_safe::Image::from_encoded(data).unwrap();

        Some(Self {
            file: Some(file),
            raw_bytes: None,
            image,
        })
    }

    /// Create image by bytes.
    ///
    /// UNSAFE: User should guarantee the bytes wiil not outlive the lifetime of the `ImageBuf`
    #[inline]
    pub unsafe fn from_bytes<'a>(bytes: &'a [u8]) -> Self {
        let data = skia_safe::Data::new_bytes(bytes);
        let image = skia_safe::Image::from_encoded(data).unwrap();

        Self {
            file: None,
            raw_bytes: Some(bytes),
            image,
        }
    }

    /// Create image by static bytes.
    #[inline]
    pub fn from_static_bytes(bytes: &'static [u8]) -> Self {
        let data = unsafe { skia_safe::Data::new_bytes(bytes) };
        let image = skia_safe::Image::from_encoded(data).unwrap();

        Self {
            file: None,
            raw_bytes: Some(bytes),
            image,
        }
    }

    #[inline]
    pub fn image_ref(&self) -> &skia_safe::Image {
        &self.image
    }

    #[inline]
    pub fn image_mut(&mut self) -> &mut skia_safe::Image {
        &mut self.image
    }

    #[inline]
    pub fn raw_file(&self) -> Option<&[u8]> {
        self.file.as_ref().map(|v| v.as_slice())
    }

    #[inline]
    pub unsafe fn raw_bytes(&self) -> Option<&[u8]> {
        self.raw_bytes.map(|v| v.as_ref().unwrap())
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.image.width()
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.image.height()
    }
}

impl AsRef<skia_safe::Image> for ImageBuf {
    #[inline]
    fn as_ref(&self) -> &skia_safe::Image {
        &self.image
    }
}

impl AsMut<skia_safe::Image> for ImageBuf {
    #[inline]
    fn as_mut(&mut self) -> &mut skia_safe::Image {
        &mut self.image
    }
}
