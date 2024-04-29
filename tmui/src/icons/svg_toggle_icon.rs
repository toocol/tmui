use std::io::Read;

use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_inner::WidgetInnerExt, WidgetImpl},
};
use tlib::{
    connect,
    skia_safe::FontMgr,
    typedef::SkiaSvgDom,
};
use usvg::{fontdb::Database, Options, Tree};

#[extends(Widget)]
pub struct SvgToggleIcon {
    doms: Vec<SkiaSvgDom>,
    view_size: Size,
    origin: FPoint,
    index: usize,
}

impl ObjectSubclass for SvgToggleIcon {
    const NAME: &'static str = "SvgToggleIcon";
}

impl ObjectImpl for SvgToggleIcon {
    fn initialize(&mut self) {
        connect!(
            self,
            geometry_changed(),
            self,
            handle_geometry_changed(Rect)
        );
    }
}

impl WidgetImpl for SvgToggleIcon {
    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        if let Some(dom) = self.doms.get(self.index) {
            painter.save();
            painter.translate(self.origin.x(), self.origin.y());
            painter.draw_dom(dom);
            painter.restore();
        }
    }
}

impl SvgToggleIcon {
    #[inline]
    pub fn from_files(paths: &[&str]) -> Box<Self> {
        let mut icon: Box<Self> = Object::new(&[]);

        let mut datas = vec![];
        for path in paths {
            let mut file = std::fs::File::open(path).expect("Open file failed");
            let mut data = vec![];
            file.read_to_end(&mut data).expect("Read file failed");
            datas.push(data);
        }

        icon.build_from_datas(
            datas
                .iter()
                .map(|d| d.as_slice())
                .collect::<Vec<&[u8]>>()
                .as_slice(),
        );

        icon
    }

    #[inline]
    pub fn from_bytes(data: &[&[u8]]) -> Box<Self> {
        let mut icon: Box<Self> = Object::new(&[]);

        icon.build_from_datas(data);

        icon
    }

    #[inline]
    pub fn toggle(&mut self) {
        self.index += 1;
        if self.index >= self.doms.len() {
            self.index = 0;
        }
        self.set_rerender_styles(true);
        self.update();
    }

    #[inline]
    pub fn current_icon(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn set_current_icon(&mut self, index: usize) {
        if index >= self.doms.len() || index == self.index {
            return;
        }
        self.index = index;
        self.update();
    }
}

impl SvgToggleIcon {
    fn build_from_datas(&mut self, datas: &[&[u8]]) {
        for &data in datas {
            let svg_tree = Tree::from_data(data, &Options::default(), &Database::default())
                .expect("Create svg tree failed");
            let size = svg_tree.size();
            let (w, h) = (size.width().ceil() as i32, size.height().ceil() as i32);

            self.view_size.set_width(self.view_size.width().max(w));
            self.view_size.set_height(self.view_size.height().max(h));

            let dom =
                SkiaSvgDom::from_bytes(data, FontMgr::default()).expect("Create svg dom failed");
            self.doms.push(dom);
        }

        self.set_fixed_width(self.view_size.width());
        self.set_detecting_width(self.view_size.width());
        self.set_fixed_height(self.view_size.height());
        self.set_detecting_height(self.view_size.height());
    }

    #[inline]
    fn handle_geometry_changed(&mut self, rect: Rect) {
        let (x1, y1, w1, h1) = (rect.x(), rect.y(), rect.width(), rect.height());
        let (w2, h2) = (self.view_size.width(), self.view_size.height());
        self.origin = FPoint::new((x1 + (w1 - w2) / 2) as f32, (y1 + (h1 - h2) / 2) as f32);
    }
}
