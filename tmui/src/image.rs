use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use std::path::Path;
use tlib::{
    figure::ImageBuf,
    namespace::ImageOption,
    skia_safe::{
        self, matrix::ScaleToFit, region::RegionOp, ClipOp, Matrix, Paint, SamplingOptions,
        TileMode,
    },
};

#[extends(Widget)]
pub struct Image {
    image_buf: Option<ImageBuf>,
    option: ImageOption,
}

impl ObjectSubclass for Image {
    const NAME: &'static str = "Image";
}

impl ObjectImpl for Image {}

impl WidgetImpl for Image {
    fn paint(&mut self, painter: &mut Painter) {
        let mut contents_rect = self.contents_rect(Some(Coordinate::Widget));
        let image_buf = self.image_buf.as_ref().unwrap();

        match self.option {
            ImageOption::Fill => {
                let rect = self.contents_rect(None);
                let src: skia_safe::Rect = skia_safe::Rect::from_xywh(
                    rect.x() as f32,
                    rect.y() as f32,
                    image_buf.width() as f32,
                    image_buf.height() as f32,
                );
                let dst: skia_safe::Rect = skia_safe::Rect::from_xywh(
                    rect.x() as f32,
                    rect.y() as f32,
                    contents_rect.width() as f32,
                    contents_rect.height() as f32,
                );
                let matrix = Matrix::rect_to_rect(src, dst, Some(ScaleToFit::Fill)).unwrap();

                painter.set_transform(matrix, false);
                painter.draw_image(image_buf, contents_rect.top_left());
            }
            ImageOption::Adapt => {
                let rect = self.contents_rect(None);
                let src: skia_safe::Rect = skia_safe::Rect::from_xywh(
                    rect.x() as f32,
                    rect.y() as f32,
                    image_buf.width() as f32,
                    image_buf.height() as f32,
                );
                let dst: skia_safe::Rect = skia_safe::Rect::from_xywh(
                    rect.x() as f32,
                    rect.y() as f32,
                    contents_rect.width() as f32,
                    contents_rect.height() as f32,
                );
                let matrix = Matrix::rect_to_rect(src, dst, None).unwrap();

                painter.set_transform(matrix, false);
                painter.draw_image(image_buf, contents_rect.top_left());
            }
            ImageOption::Tile => {
                let mut paint = Paint::default();
                let shader = image_buf.image_ref().to_shader(
                    Some((TileMode::Repeat, TileMode::Repeat)),
                    SamplingOptions::default(),
                    None,
                );
                paint.set_shader(shader);

                painter.offset_rect(&mut contents_rect);
                let mut region = skia_safe::Region::new();
                let contents_rect: skia_safe::IRect = contents_rect.into();
                region.op_rect(contents_rect, RegionOp::Union);

                painter.clip_region_global(region, ClipOp::Intersect);
                painter.draw_paint(&paint);
            }
            ImageOption::Stretch => {
                painter.draw_image_rect(image_buf, None, contents_rect);
            }
            ImageOption::Center => {
                let rect = self.contents_rect(None);
                let src: skia_safe::Rect = skia_safe::Rect::from_xywh(
                    rect.x() as f32,
                    rect.y() as f32,
                    image_buf.width() as f32,
                    image_buf.height() as f32,
                );
                let dst: skia_safe::Rect = skia_safe::Rect::from_xywh(
                    rect.x() as f32,
                    rect.y() as f32,
                    contents_rect.width() as f32,
                    contents_rect.height() as f32,
                );
                let matrix = Matrix::rect_to_rect(src, dst, Some(ScaleToFit::Center)).unwrap();

                painter.set_transform(matrix, false);
                painter.draw_image(image_buf, contents_rect.top_left());
            }
        }
    }
}

impl Image {
    #[inline]
    pub fn new<T: AsRef<Path>>(path: T) -> Box<Self> {
        let image_buf = ImageBuf::from_file(&path).unwrap_or_else(|| {
            panic!(
                "load image file `{:?}` failed, maybe it's not exist",
                path.as_ref().as_os_str()
            )
        });

        let mut image: Box<Self> = Object::new(&[]);
        image.image_buf = Some(image_buf);
        image
    }

    #[inline]
    pub fn image_option(&self) -> ImageOption {
        self.option
    }

    #[inline]
    pub fn set_image_option(&mut self, option: ImageOption) {
        self.option = option;
        self.update();
    }
}
