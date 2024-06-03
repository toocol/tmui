use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_inner::WidgetInnerExt, WidgetImpl},
};
use std::io::Read;
use tlib::{
    connect,
    skia_safe::FontMgr,
    typedef::SkiaSvgDom,
};
use usvg::{fontdb::Database, Options, Tree};

/// TODO: Wait to improve, see https://github.com/rust-skia/rust-skia/discussions/928
#[extends(Widget)]
pub struct SvgIcon {
    dom: Option<SkiaSvgDom>,
    view_size: Size,
    origin: FPoint,
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
            handle_geometry_changed(FRect)
        );
    }
}

impl WidgetImpl for SvgIcon {
    fn paint(&mut self, painter: &mut Painter) {
        if let Some(ref dom) = self.dom {
            painter.save();
            painter.translate(self.origin.x(), self.origin.y());
            painter.draw_dom(dom);
            painter.restore();
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
    fn handle_geometry_changed(&mut self, rect: FRect) {
        let (x1, y1, w1, h1) = (rect.x(), rect.y(), rect.width(), rect.height());
        let (w2, h2) = (self.view_size.width() as f32, self.view_size.height() as f32);
        self.origin = FPoint::new(x1 + (w1 - w2) / 2., y1 + (h1 - h2) / 2.);
    }
}
