use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_inner::WidgetInnerExt, WidgetImpl},
};
use std::io::Read;
use tlib::{
    connect,
    skia_safe::{surfaces, FontMgr, Surface},
    typedef::SkiaSvgDom,
};
use usvg::{fontdb::Database, Options, Tree};

// TODO: The reason for adding a new rendering mode is that when directly using the existing Canvas for rendering, 
// the rendered image may appear abnormally transparent, and it cannot be reproduced.
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum RenderMode {
    #[default]
    Direct,
    TempSurface,
}

/// TODO: Wait to improve, see https://github.com/rust-skia/rust-skia/discussions/928
#[extends(Widget)]
pub struct SvgIcon {
    dom: Option<SkiaSvgDom>,
    view_size: Size,
    origin: FPoint,

    render_mode: RenderMode,
    surface: Option<Surface>,
}

impl ObjectSubclass for SvgIcon {
    const NAME: &'static str = "SvgIcon";
}

impl ObjectImpl for SvgIcon {
    fn initialize(&mut self) {
        connect!(
            self,
            geometry_changed(),
            self,
            handle_geometry_changed(Rect)
        );
    }
}

impl WidgetImpl for SvgIcon {
    fn paint(&mut self, painter: &mut Painter) {
        if let Some(ref dom) = self.dom {
            match self.render_mode {
                RenderMode::Direct => {
                    painter.save();
                    painter.translate(self.origin.x(), self.origin.y());
                    painter.draw_dom(dom);
                    painter.restore();
                }
                RenderMode::TempSurface => {
                    if self.surface.is_none() {
                        self.surface = Some(
                            surfaces::raster_n32_premul((
                                self.view_size.width(),
                                self.view_size.height(),
                            ))
                            .unwrap(),
                        );
                    }

                    let surface = self.surface.as_mut().unwrap();
                    let canvas = surface.canvas();
                    canvas.clear(Color::TRANSPARENT);
                    dom.render(canvas);
                    let image = surface.image_snapshot();
                    let mapping_origin = self.map_to_widget_f(&self.origin);
                    painter.draw_image(image, mapping_origin);
                }
            }
        }
    }
}

impl SvgIcon {
    #[inline]
    pub fn from_file(path: &str) -> Box<Self> {
        let mut icon: Box<Self> = Object::new(&[]);

        let mut file = std::fs::File::open(path).expect("Open file failed");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Read file failed");

        icon.build_from_data(&data);

        icon
    }

    #[inline]
    pub fn from_bytes(data: &[u8]) -> Box<Self> {
        let mut icon: Box<Self> = Object::new(&[]);

        icon.build_from_data(data);

        icon
    }

    #[inline]
    pub fn set_render_mode(&mut self, mode: RenderMode) {
        self.render_mode = mode;

        self.update();
    }
}

impl SvgIcon {
    #[inline]
    fn build_from_data(&mut self, data: &[u8]) {
        let svg_tree = Tree::from_data(data, &Options::default(), &Database::default())
            .expect("Create svg tree failed");
        let size = svg_tree.size();
        let (w, h) = (size.width().ceil() as i32, size.height().ceil() as i32);

        self.view_size = (w, h).into();

        self.set_fixed_width(w);
        self.set_detecting_width(w);
        self.set_fixed_height(h);
        self.set_detecting_height(h);

        let dom = SkiaSvgDom::from_bytes(data, FontMgr::default()).expect("Create svg dom failed");

        self.dom = Some(dom);
    }

    #[inline]
    fn handle_geometry_changed(&mut self, rect: Rect) {
        let (x1, y1, w1, h1) = (rect.x(), rect.y(), rect.width(), rect.height());
        let (w2, h2) = (self.view_size.width(), self.view_size.height());
        self.origin = FPoint::new((x1 + (w1 - w2) / 2) as f32, (y1 + (h1 - h2) / 2) as f32);
    }
}
