use super::painter::Painter;
use crate::prelude::*;
use tlib::skia_safe::region::RegionOp;
use tlib::skia_safe::{self, ClipOp};

pub trait RenderDiffence: WidgetImpl + ChildWidgetDiffRender {
    fn render_difference(&mut self, painter: &mut Painter, background: Color) {
        let mut widget_rect = self.rect();
        widget_rect.set_point(&(0, 0).into());

        let rect: skia_safe::IRect = self.rect().into();
        let old_rect: skia_safe::IRect = self.rect_record().into();

        let mut region = skia_safe::Region::new();
        region.op_rect(rect, RegionOp::Union);
        region.op_rect(old_rect, RegionOp::Difference);

        painter.save();
        painter.clip_region(region, ClipOp::Intersect);
        painter.fill_rect(widget_rect, background);
        painter.restore();

        let is_container = self.super_type().is_a(Container::static_type());
        if is_container {
            cast_mut!(self as ChildContainerDiffRender)
                .unwrap()
                .container_diff_render(painter, background);
        } else {
            self.widget_diff_render(painter, background);
        }
    }
}
impl<T: WidgetImpl> RenderDiffence for T {}

pub trait ChildWidgetDiffRender: WidgetImpl {
    fn widget_diff_render(&mut self, painter: &mut Painter, background: Color) {
        if let Some(child) = self.get_child_ref() {
            // Handle child with margins:
            let rec_rect = child.image_rect_record();
            let cur_rect = child.rect();

            if rec_rect != cur_rect {
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
                painter.fill_rect(rec_rect, background);
                painter.restore();
            }

            // Handle child with paddings:
            let rec_rect: skia_safe::IRect = child.rect().into();
            let child_content_rect: skia_safe::IRect = child.contents_rect(None).into();

            if rec_rect != child_content_rect {
                let mut clear_region = skia_safe::Region::new();
                clear_region.op_rect(rec_rect, RegionOp::Union);
                clear_region.op_rect(child_content_rect, RegionOp::Difference);

                painter.save();
                painter.clip_region(clear_region, ClipOp::Intersect);
                painter.fill_rect(rec_rect, background);
                painter.restore();
            }

            // Customize difference render:
            if let Some(custom_render_diff) = cast!(child as CustomRenderDiff) {
                custom_render_diff.custom_render_diff(painter, background);
            }
        }
    }
}
impl<T: WidgetImpl> ChildWidgetDiffRender for T {}

#[reflect_trait]
pub trait ChildContainerDiffRender {
    fn container_diff_render(&mut self, painter: &mut Painter, background: Color);
}

#[reflect_trait]
pub trait CustomRenderDiff {
    fn custom_render_diff(&self, painter: &mut Painter, parent_background: Color);
}
