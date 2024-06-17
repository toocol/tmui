use super::painter::Painter;
use crate::{skia_safe, widget::WidgetImpl};
use derivative::Derivative;
use tlib::{
    bitflags::bitflags,
    figure::{Color, FRect},
    namespace::BlendMode,
    skia_safe::{ClipOp, MaskFilter},
};

#[derive(Derivative, PartialEq, Clone, Copy)]
#[derivative(Default)]
pub struct BoxShadow {
    blur: f32,
    color: Color,
    pos: ShadowPos,
    #[derivative(Default(value = "ShadowSide::all()"))]
    side: ShadowSide,
    strength: usize,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum ShadowPos {
    #[default]
    Outset,
    Inset,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ShadowSide: u8 {
        const TOP = 1;
        const RIGHT = 1 << 1;
        const BOTTOM = 1 << 2;
        const LEFT = 1 << 3;
    }
}
impl ShadowSide {
    #[inline]
    pub fn new(sides: &[ShadowSide]) -> Self {
        let mut rlt = ShadowSide::empty();
        for side in sides {
            rlt.insert(*side);
        }
        rlt
    }
}

impl BoxShadow {
    #[inline]
    pub fn new(
        blur: f32,
        color: Color,
        pos: Option<ShadowPos>,
        side: Option<ShadowSide>,
        strength: Option<usize>,
    ) -> Self {
        Self {
            blur,
            color,
            pos: pos.unwrap_or_default(),
            side: side.unwrap_or(ShadowSide::all()),
            strength: strength.unwrap_or_default(),
        }
    }

    #[inline]
    pub fn blur(&self) -> f32 {
        self.blur
    }

    #[inline]
    pub fn set_blur(&mut self, blur: f32) {
        self.blur = blur
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    #[inline]
    pub fn pos(&self) -> ShadowPos {
        self.pos
    }

    #[inline]
    pub fn set_pos(&mut self, pos: ShadowPos) {
        self.pos = pos
    }

    #[inline]
    pub fn side(&self) -> ShadowSide {
        self.side
    }

    #[inline]
    pub fn set_side(&mut self, side: ShadowSide) {
        self.side = side
    }

    #[inline]
    pub fn strength(&self) -> usize {
        self.strength
    }

    #[inline]
    pub fn set_strength(&mut self, strength: usize) {
        self.strength = strength
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

        painter.save();
        match box_shadow.pos {
            ShadowPos::Outset => painter.clip_rect_global(self.rect_f(), ClipOp::Difference),
            ShadowPos::Inset => painter.clip_rect_global(self.rect_f(), ClipOp::Intersect),
        }

        painter.save_pen();
        painter.set_blend_mode(BlendMode::SrcOver);
        painter.set_line_width(1.);
        painter.set_color(box_shadow.color);
        painter.paint_mut().set_mask_filter(blur);

        let rect = self.rect_f();
        let side = box_shadow.side;
        for _ in 0..box_shadow.strength + 1 {
            if side.is_all() {
                painter.draw_rect_global(rect);
            } else {
                if side.contains(ShadowSide::TOP) {
                    painter.draw_line_f_global(rect.left(), rect.top(), rect.right(), rect.top())
                } 
                if side.contains(ShadowSide::RIGHT) {
                    painter.draw_line_f_global(
                        rect.right(),
                        rect.top(),
                        rect.right(),
                        rect.bottom(),
                    )
                } 
                if side.contains(ShadowSide::BOTTOM) {
                    painter.draw_line_f_global(
                        rect.left(),
                        rect.bottom(),
                        rect.right(),
                        rect.bottom(),
                    )
                } 
                if side.contains(ShadowSide::LEFT) {
                    painter.draw_line_f_global(rect.left(), rect.bottom(), rect.left(), rect.top())
                }
            }
        }

        painter.paint_mut().set_mask_filter(None);
        painter.set_blend_mode(self.blend_mode());
        painter.restore_pen();
        painter.restore();
    }

    fn render_shadow_diff(&self, _painter: &mut Painter, _geometry: FRect, _background: Color) {
        // TODO: implements this function.
    }
}
impl<T: WidgetImpl> ShadowRender for T {}
