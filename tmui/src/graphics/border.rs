use derivative::Derivative;
use tlib::{
    figure::{Color, FRect},
    namespace::BorderStyle,
    skia_safe::PathEffect,
};

use super::painter::Painter;

pub(crate) const DEFAULT_BORDER_COLOR: Color = Color::BLACK;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Border {
    pub(crate) style: BorderStyle,
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
            // TODO: Deal with round rect with border.
        } else {
            self.render_normal(painter, geometry)
        }

        painter.restore_pen();
    }

    fn render_normal(&self, painter: &mut Painter, geometry: FRect) {
        let f_rect: FRect = geometry.into();

        let (top, right, bottom, left) = self.width;

        if top > 0. {
            painter.set_line_width(top);
            painter.set_color(self.border_color.0);

            let (x1, y1, x2, y2) = (
                f_rect.left(),
                f_rect.top() - top / 2.,
                f_rect.right(),
                f_rect.top() - top / 2.,
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
                    painter.draw_line_f(x1, y1 - top * 2., x2, y2 - top * 2.);
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
                f_rect.right() + right / 2.,
                f_rect.top(),
                f_rect.right() + right / 2.,
                f_rect.bottom(),
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
                    painter.draw_line_f(x1 + right * 2., y1, x2 + right * 2., y2)
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
                f_rect.left(),
                f_rect.bottom() + bottom / 2.,
                f_rect.right(),
                f_rect.bottom() + bottom / 2.,
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
                    painter.draw_line_f(x1, y1 + bottom * 2., x2, y2 + bottom * 2.);
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
                f_rect.left() - left / 2.,
                f_rect.top(),
                f_rect.left() - left / 2.,
                f_rect.bottom(),
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
                    painter.draw_line_f(x1 - left * 2., y1, x2 - left * 2., y2);
                }
                _ => {}
            }

            painter.draw_line_f(x1, y1, x2, y2);
            painter.reset();
        }
    }
}
