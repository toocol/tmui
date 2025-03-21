use super::painter::Painter;
use crate::{skia_safe, widget::WidgetImpl};
use derivative::Derivative;
use tlib::{
    bitflags::bitflags,
    figure::{Color, FRect},
    namespace::BlendMode,
    skia_safe::{ClipOp, MaskFilter},
};

#[derive(Derivative, PartialEq, Clone, Copy, Debug)]
#[derivative(Default)]
pub struct BoxShadow {
    blur: f32,
    color: Color,
    pos: ShadowPos,
    #[derivative(Default(value = "ShadowSide::all()"))]
    side: ShadowSide,
    strength: usize,
    blend_mode: BlendMode,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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
        blend_mode: Option<BlendMode>,
    ) -> Self {
        Self {
            blur,
            color,
            pos: pos.unwrap_or_default(),
            side: side.unwrap_or(ShadowSide::all()),
            strength: strength.unwrap_or_default(),
            blend_mode: blend_mode.unwrap_or(BlendMode::SrcOver),
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
        let draw_radius = self.border_ref().should_draw_radius();
        match box_shadow.pos {
            ShadowPos::Outset => {
                if draw_radius {
                    painter.clip_round_rect_global(
                        self.rect_f(),
                        self.border_ref().border_radius,
                        ClipOp::Difference,
                    )
                } else {
                    painter.clip_rect_global(self.rect_f(), ClipOp::Difference)
                }
            }
            ShadowPos::Inset => {
                if draw_radius {
                    painter.clip_round_rect_global(
                        self.rect_f(),
                        self.border_ref().border_radius,
                        ClipOp::Intersect,
                    )
                } else {
                    painter.clip_rect_global(self.rect_f(), ClipOp::Intersect)
                }
            }
        }

        painter.save_pen();
        painter.set_blend_mode(box_shadow.blend_mode);
        painter.set_line_width(1.);
        painter.set_color(box_shadow.color);
        painter.paint_mut().set_mask_filter(blur);

        let rect = self.rect_f();
        let (lt, rt, rb, lb) = rect.arc_points(self.border_ref().border_radius);
        let radius = self.border_ref().border_radius;
        let side = box_shadow.side;
        for _ in 0..box_shadow.strength + 1 {
            if side.is_all() {
                if self.border_ref().should_draw_radius() {
                    painter.draw_round_rect_global(rect, radius);
                } else {
                    painter.draw_rect_global(rect);
                }
            } else {
                if side.contains(ShadowSide::TOP) {
                    if radius.0 > 0. || radius.1 > 0. {
                        // Radius: top-left
                        painter.draw_line_f_global(lt.1.x(), lt.1.y(), rt.0.x(), rt.0.y());
                        let dimension = 2. * radius.0;
                        let lt = FRect::new(rect.left(), rect.top(), dimension, dimension);
                        painter.draw_varying_arc_global(lt, 225., 45., 1., 1., 8);
                    } else {
                        painter.draw_line_f_global(
                            rect.left(),
                            rect.top(),
                            rect.right(),
                            rect.top(),
                        );
                    }
                    if radius.1 > 0. {
                        // Radius: top-right
                        let dimension = 2. * radius.1;
                        let rt =
                            FRect::new(rect.right() - dimension, rect.top(), dimension, dimension);
                        painter.draw_varying_arc_global(rt, 270., 45., 1., 1., 8);
                    }
                }
                if side.contains(ShadowSide::RIGHT) {
                    if radius.1 > 0. || radius.2 > 0. {
                        // Radius: top-right
                        painter.draw_line_f_global(rt.1.x(), rt.1.y(), rb.0.x(), rb.0.y());
                        let dimension = 2. * radius.1;
                        let rt =
                            FRect::new(rect.right() - dimension, rect.top(), dimension, dimension);
                        painter.draw_varying_arc_global(rt, 315., 45., 1., 1., 8);
                    } else {
                        painter.draw_line_f_global(
                            rect.right(),
                            rect.top(),
                            rect.right(),
                            rect.bottom(),
                        )
                    }
                    if radius.2 > 0. {
                        // Radius: bottom-right
                        let dimension = 2. * radius.2;
                        let rb = FRect::new(
                            rect.right() - dimension,
                            rect.bottom() - dimension,
                            dimension,
                            dimension,
                        );
                        painter.draw_varying_arc_global(rb, 0., 45., 1., 1., 8);
                    }
                }
                if side.contains(ShadowSide::BOTTOM) {
                    if radius.2 > 0. || radius.3 > 0. {
                        // Radius: bottom-right
                        painter.draw_line_f_global(rb.1.x(), rb.1.y(), lb.0.x(), lb.0.y());
                        let dimension = 2. * radius.2;
                        let rb = FRect::new(
                            rect.right() - dimension,
                            rect.bottom() - dimension,
                            dimension,
                            dimension,
                        );
                        painter.draw_varying_arc_global(rb, 45., 45., 1., 1., 8);
                    } else {
                        painter.draw_line_f_global(
                            rect.left(),
                            rect.bottom(),
                            rect.right(),
                            rect.bottom(),
                        )
                    }
                    if radius.3 > 0. {
                        // Radius: bottom-left
                        let dimension = 2. * radius.3;
                        let lb = FRect::new(
                            rect.left(),
                            rect.bottom() - dimension,
                            dimension,
                            dimension,
                        );
                        painter.draw_varying_arc_global(lb, 90., 45., 1., 1., 8);
                    }
                }
                if side.contains(ShadowSide::LEFT) {
                    if radius.3 > 0. || radius.0 > 0. {
                        // Radius: bottom-left
                        painter.draw_line_f_global(lb.1.x(), lb.1.y(), lt.0.x(), lt.0.y());
                        let dimension = 2. * radius.3;
                        let lb = FRect::new(
                            rect.left(),
                            rect.bottom() - dimension,
                            dimension,
                            dimension,
                        );
                        painter.draw_varying_arc_global(lb, 135., 45., 1., 1., 8);
                    } else {
                        painter.draw_line_f_global(
                            rect.left(),
                            rect.bottom(),
                            rect.left(),
                            rect.top(),
                        )
                    }
                }
                if radius.0 > 0. {
                    // Radius: top-left
                    let dimension = 2. * radius.0;
                    let lt = FRect::new(rect.left(), rect.top(), dimension, dimension);
                    painter.draw_varying_arc_global(lt, 180., 45., 1., 1., 8);
                }
            }
        }

        painter.paint_mut().set_mask_filter(None);
        painter.set_blend_mode(self.blend_mode());
        painter.restore_pen();
        painter.restore();
    }
}
impl<T: WidgetImpl> ShadowRender for T {}
