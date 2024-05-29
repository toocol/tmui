use super::painter::Painter;
use crate::{skia_safe, widget::WidgetImpl};
use tlib::{figure::{Color, FRect}, namespace::BlendMode, skia_safe::MaskFilter};

#[derive(Default, PartialEq, Clone, Copy)]
pub struct BoxShadow {
    blur: f32,
    color: Color,
}

impl BoxShadow {
    #[inline]
    pub fn new(blur: f32, color: Color) -> Self {
        Self { blur, color }
    }

    #[inline]
    pub fn blur(&self) -> f32 {
        self.blur
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }
}

pub trait ShadowRender: WidgetImpl {
    fn render_shadow(&self, painter: &mut Painter) {
        let box_shadow = self.box_shadow();
        if box_shadow.is_none() {
            return;
        }
        let box_shadow = box_shadow.unwrap();

        let sigma = box_shadow.blur / 3.;
        let blur = MaskFilter::blur(skia_safe::BlurStyle::Normal, sigma, None);

        painter.save_pen();
        painter.set_blend_mode(BlendMode::SrcOver);
        painter.set_line_width(1.);
        painter.set_color(box_shadow.color);
        painter.paint_mut().set_mask_filter(blur);

        painter.draw_rect_global(self.rect());

        painter.paint_mut().set_mask_filter(None);
        painter.restore_pen();
        painter.set_blend_mode(BlendMode::default());
    }

    fn render_shadow_diff(&self, _painter: &mut Painter, _geometry: FRect, _background: Color) {
        // TODO: implements this function.
    }
}
impl<T: WidgetImpl> ShadowRender for T {}