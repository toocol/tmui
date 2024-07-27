#![allow(clippy::single_match)]
use derivative::Derivative;
use tlib::{
    figure::{Color, FRect},
    namespace::BorderStyle,
    skia_safe::PathEffect,
};

use super::painter::Painter;

pub(crate) const DEFAULT_BORDER_COLOR: Color = Color::BLACK;

#[derive(Derivative, Clone, Copy, Debug)]
#[derivative(Default)]
pub struct Border {
    pub(crate) style: BorderStyle,
    /// (top, right, bottom, left)
    pub(crate) width: (f32, f32, f32, f32),
    #[derivative(Default(
        value = "(DEFAULT_BORDER_COLOR, DEFAULT_BORDER_COLOR, DEFAULT_BORDER_COLOR, DEFAULT_BORDER_COLOR)"
    ))]
    pub(crate) border_color: (Color, Color, Color, Color),
    pub(crate) border_radius: f32,
}

impl Border {
    #[inline]
    pub fn new(
        style: BorderStyle,
        width: (f32, f32, f32, f32),
        border_color: (Color, Color, Color, Color),
        radius: f32,
    ) -> Self {
        Self {
            style,
            width,
            border_color,
            border_radius: radius,
        }
    }

    pub(crate) fn render(&self, painter: &mut Painter, geometry: FRect) {
        painter.save_pen();

        if self.border_radius > 0. {
            self.render_radius(painter, geometry);
        } else {
            self.render_normal(painter, geometry)
        }

        painter.restore_pen();
    }

    pub(crate) fn clear_border(&self, painter: &mut Painter, geometry: FRect, color: Color) {
        painter.save_pen();
        painter.set_color(color);

        let (top, right, bottom, left) = self.width;

        if top > 0. {
            painter.set_line_width(top);

            let (x1, y1, x2, y2) = (
                geometry.left(),
                geometry.top() + top / 2.,
                geometry.right(),
                geometry.top() + top / 2.,
            );
            match self.style {
                BorderStyle::Double => {
                    painter.draw_line_f(x1, y1 + top * 2., x2, y2 + top * 2.);
                }
                _ => {}
            }
            painter.draw_line_f(x1, y1, x2, y2);
        }

        if right > 0. {
            painter.set_line_width(right);

            let (x1, y1, x2, y2) = (
                geometry.right() - right / 2.,
                geometry.top(),
                geometry.right() - right / 2.,
                geometry.bottom(),
            );
            match self.style {
                BorderStyle::Double => {
                    painter.draw_line_f(x1 - right * 2., y1, x2 - right * 2., y2);
                }
                _ => {}
            }
            painter.draw_line_f(x1, y1, x2, y2);
        }

        if bottom > 0. {
            painter.set_line_width(bottom);

            let (x1, y1, x2, y2) = (
                geometry.left(),
                geometry.bottom() - bottom / 2.,
                geometry.right(),
                geometry.bottom() - bottom / 2.,
            );
            match self.style {
                BorderStyle::Double => {
                    painter.draw_line_f(x1, y1 - bottom * 2., x2, y2 - bottom * 2.);
                }
                _ => {}
            }
            painter.draw_line_f(x1, y1, x2, y2);
        }

        if left > 0. {
            painter.set_line_width(left);

            let (x1, y1, x2, y2) = (
                geometry.left() + left / 2.,
                geometry.top(),
                geometry.left() + left / 2.,
                geometry.bottom(),
            );
            match self.style {
                BorderStyle::Double => {
                    painter.draw_line_f(x1 + left * 2., y1, x2 + left * 2., y2);
                }
                _ => {}
            }
            painter.draw_line_f(x1, y1, x2, y2);
        }
        painter.restore_pen()
    }

    fn render_normal(&self, painter: &mut Painter, geometry: FRect) {
        let (top, right, bottom, left) = self.width;

        if top > 0. {
            painter.set_line_width(top);
            painter.set_color(self.border_color.0);

            let (x1, y1, x2, y2) = (
                geometry.left(),
                geometry.top() + top / 2.,
                geometry.right(),
                geometry.top() + top / 2.,
            );

            match self.style {
                BorderStyle::Dotted => {
                    let intervals = [top, top];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Dashed => {
                    let intervals = [10., 10.];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Double => {
                    painter.draw_line_f(x1, y1 + top * 2., x2, y2 + top * 2.);
                }
                _ => {}
            }

            painter.draw_line_f(x1, y1, x2, y2);
            painter.reset();
        }

        if right > 0. {
            painter.set_line_width(right);
            painter.set_color(self.border_color.1);

            let (x1, y1, x2, y2) = (
                geometry.right() - right / 2.,
                geometry.top(),
                geometry.right() - right / 2.,
                geometry.bottom(),
            );

            match self.style {
                BorderStyle::Dotted => {
                    let intervals = [right, right];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Dashed => {
                    let intervals = [10., 10.];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Double => {
                    painter.draw_line_f(x1 - right * 2., y1, x2 - right * 2., y2)
                }
                _ => {}
            }

            painter.draw_line_f(x1, y1, x2, y2);
            painter.reset();
        }

        if bottom > 0. {
            painter.set_line_width(bottom);
            painter.set_color(self.border_color.2);

            let (x1, y1, x2, y2) = (
                geometry.left(),
                geometry.bottom() - bottom / 2.,
                geometry.right(),
                geometry.bottom() - bottom / 2.,
            );

            match self.style {
                BorderStyle::Dotted => {
                    let intervals = [bottom, bottom];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Dashed => {
                    let intervals = [10., 10.];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Double => {
                    painter.draw_line_f(x1, y1 - bottom * 2., x2, y2 - bottom * 2.);
                }
                _ => {}
            }

            painter.draw_line_f(x1, y1, x2, y2);
            painter.reset();
        }

        if left > 0. {
            painter.set_line_width(left);
            painter.set_color(self.border_color.3);

            let (x1, y1, x2, y2) = (
                geometry.left() + left / 2.,
                geometry.top(),
                geometry.left() + left / 2.,
                geometry.bottom(),
            );

            match self.style {
                BorderStyle::Dotted => {
                    let intervals = [left, left];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Dashed => {
                    let intervals = [10., 10.];
                    painter
                        .paint_mut()
                        .set_path_effect(PathEffect::dash(&intervals, 0.));
                }
                BorderStyle::Double => {
                    painter.draw_line_f(x1 + left * 2., y1, x2 + left * 2., y2);
                }
                _ => {}
            }

            painter.draw_line_f(x1, y1, x2, y2);
            painter.reset();
        }
    }

    fn render_radius(&self, painter: &mut Painter, mut geometry: FRect) {
        let (top, right, bottom, left) = self.width;
        if self.is_same_color() && self.is_same_width() {
            painter.set_color(self.border_color.0);
            painter.set_line_width(top);

            geometry.offset(left / 2., top / 2.);
            geometry.set_width(geometry.width() - right);
            geometry.set_height(geometry.height() - bottom);
            painter.draw_round_rect(geometry, self.border_radius);

            painter.reset();
            return;
        }

        let (lt, rt, rb, lb) = geometry.arc_points(self.border_radius);

        if rt.0.x() > lt.1.x() {
            painter.set_line_width(top);
            painter.set_color(self.border_color.0);
            painter.draw_line_f(lt.1.x(), lt.1.y() + top / 2., rt.0.x(), rt.0.y() + top / 2.);
        }
        if rb.0.y() > rt.1.y() {
            painter.set_line_width(right);
            painter.set_color(self.border_color.1);
            painter.draw_line_f(
                rt.1.x() - right / 2.,
                rt.1.y(),
                rb.0.x() - right / 2.,
                rb.0.y(),
            );
        }
        if lb.0.x() < rb.1.x() {
            painter.set_line_width(bottom);
            painter.set_color(self.border_color.2);
            painter.draw_line_f(
                rb.1.x(),
                rb.1.y() - bottom / 2.,
                lb.0.x(),
                lb.0.y() - bottom / 2.,
            );
        }
        if lt.0.y() < lb.1.y() {
            painter.set_line_width(left);
            painter.set_color(self.border_color.3);
            painter.draw_line_f(
                lb.1.x() + left / 2.,
                lb.1.y(),
                lt.0.x() + left / 2.,
                lt.0.y(),
            );
        }

        // Draw arc angles:
        painter.set_antialiasing(true);
        let dimension = 2. * self.border_radius;

        let lt = FRect::new(
            geometry.left() + left / 2.,
            geometry.top() + top / 2.,
            dimension,
            dimension,
        );
        let (start_width, mid_width, end_width) = (left, (left + top) / 2., top);
        painter.set_color(self.border_color.3);
        painter.draw_varying_arc(lt, 180., 45., start_width, mid_width, 8);
        painter.set_color(self.border_color.0);
        painter.draw_varying_arc(lt, 225., 45., mid_width, end_width, 8);

        let rt = FRect::new(
            geometry.right() - dimension - right / 2.,
            geometry.top() + top / 2.,
            dimension,
            dimension,
        );
        let (start_width, mid_width, end_width) = (top, (top + right) / 2., right);
        painter.set_color(self.border_color.0);
        painter.draw_varying_arc(rt, 270., 45., start_width, mid_width, 8);
        painter.set_color(self.border_color.1);
        painter.draw_varying_arc(rt, 315., 45., mid_width, end_width, 8);

        let rb = FRect::new(
            geometry.right() - dimension - right / 2.,
            geometry.bottom() - dimension - bottom / 2.,
            dimension,
            dimension,
        );
        let (start_width, mid_width, end_width) = (right, (bottom + right) / 2., bottom);
        painter.set_color(self.border_color.1);
        painter.draw_varying_arc(rb, 0., 45., start_width, mid_width, 8);
        painter.set_color(self.border_color.2);
        painter.draw_varying_arc(rb, 45., 45., mid_width, end_width, 8);

        let lb = FRect::new(
            geometry.left() + left / 2.,
            geometry.bottom() - dimension - bottom / 2.,
            dimension,
            dimension,
        );
        let (start_width, mid_width, end_width) = (bottom, (bottom + left) / 2., left);
        painter.set_color(self.border_color.2);
        painter.draw_varying_arc(lb, 90., 45., start_width, mid_width, 8);
        painter.set_color(self.border_color.3);
        painter.draw_varying_arc(lb, 135., 45., mid_width, end_width, 8);

        painter.reset();
    }

    #[inline]
    fn is_same_width(&self) -> bool {
        self.width.0 == self.width.1 && self.width.1 == self.width.2 && self.width.2 == self.width.3
    }

    #[inline]
    fn is_same_color(&self) -> bool {
        self.border_color.0 == self.border_color.1
            && self.border_color.1 == self.border_color.2
            && self.border_color.2 == self.border_color.3
    }
}
