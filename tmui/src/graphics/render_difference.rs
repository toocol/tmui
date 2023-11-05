use super::painter::Painter;
use crate::prelude::*;
use tlib::skia_safe::region::RegionOp;
use tlib::skia_safe::{self, ClipOp};

pub trait RenderDiffence: WidgetImpl + ChildWidgetDiffRender {
    fn render_difference(&mut self, painter: &mut Painter) {
        let contents_rect = self.contents_rect(Some(Coordinate::Widget));
        let rect: skia_safe::IRect = self.rect().into();
        let old_rect: skia_safe::IRect = self.rect_record().into();

        let mut region = skia_safe::Region::new();
        region.op_rect(rect, RegionOp::Union);
        region.op_rect(old_rect, RegionOp::Difference);

        painter.save();
        painter.clip_region(region, ClipOp::Intersect);
        painter.fill_rect(contents_rect, self.background());
        painter.restore();

        let is_container = self.super_type().is_a(Container::static_type());
        if is_container {
            let widget = self;
            cast_mut!(widget as ChildContainerDiffRender)
                .unwrap()
                .container_diff_render(painter);
        } else {
            self.widget_diff_render(painter);
        }
    }
}
impl<T: WidgetImpl> RenderDiffence for T {}

pub trait ChildWidgetDiffRender: WidgetImpl {
    fn widget_diff_render(&mut self, painter: &mut Painter) {
        if let Some(child) = self.get_child_ref() {
            let rec_rect = child.rect_record();
            let cur_rect = child.rect();

            let intersects: skia_safe::IRect = rec_rect
                .intersects(&cur_rect)
                .unwrap_or((0, 0, 0, 0).into())
                .into();
            let rec_rect: skia_safe::IRect = rec_rect.into();

            let mut clear_region = skia_safe::Region::new();
            clear_region.op_rect(rec_rect, RegionOp::Union);
            clear_region.op_rect(intersects, RegionOp::Difference);

            painter.save();
            painter.clip_region(clear_region, ClipOp::Intersect);
            painter.fill_rect(rec_rect, self.background());
            painter.restore();
        }
    }
}
impl<T: WidgetImpl> ChildWidgetDiffRender for T {}

#[reflect_trait]
pub trait ChildContainerDiffRender {
    fn container_diff_render(&mut self, painter: &mut Painter);
}
