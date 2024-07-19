use crate::graphics::{border::Border, painter::Painter};
use derivative::Derivative;
use tlib::{
    figure::{Color, FRect},
    namespace::BorderStyle,
    skia_safe::ClipOp,
};

use super::Status;

const DEFAULT_SELECTION: Color = Color::rgb(51, 167, 255);
const DEFAULT_HOVER: Color = Color::rgb(190, 190, 190);

#[derive(Derivative, Clone, Copy)]
#[derivative(Default)]
pub struct NodeRender {
    #[derivative(Default(value = "DEFAULT_SELECTION"))]
    selection_color: Color,
    #[derivative(Default(value = "DEFAULT_HOVER"))]
    hover_color: Color,
    pub(crate) border: Border,
}

impl NodeRender {
    #[inline]
    pub fn builder() -> NodeRenderBuilder {
        NodeRenderBuilder::default()
    }
}

impl NodeRender {
    pub(crate) fn render(
        &self,
        painter: &mut Painter,
        geometry: FRect,
        background: Color,
        status: Status,
    ) {
        painter.save();
        painter.clip_rect(geometry, ClipOp::Intersect);

        let background = match status {
            Status::Default => background,
            Status::Selected => self.selection_color,
            Status::Hovered => self.hover_color,
        };

        if self.border.border_radius > 0. {
            painter.fill_rect(geometry, background)
        } else {
            painter.fill_round_rect(geometry, self.border.border_radius, background)
        }

        self.border.render(painter, geometry);

        painter.restore();
    }
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct NodeRenderBuilder {
    #[derivative(Default(value = "DEFAULT_SELECTION"))]
    selection_color: Color,
    #[derivative(Default(value = "DEFAULT_HOVER"))]
    hover_color: Color,
    border: Border,
}

impl NodeRenderBuilder {
    #[inline]
    pub fn selection_color(mut self, color: Color) -> Self {
        self.selection_color = color;
        self
    }

    #[inline]
    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    #[inline]
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border.style = style;
        self
    }

    #[inline]
    pub fn border(mut self, border: f32) -> Self {
        self.border.width = (border, border, border, border);
        self
    }

    #[inline]
    pub fn border_top(mut self, border: f32) -> Self {
        self.border.width.0 = border;
        self
    }

    #[inline]
    pub fn border_right(mut self, border: f32) -> Self {
        self.border.width.1 = border;
        self
    }

    #[inline]
    pub fn border_bottom(mut self, border: f32) -> Self {
        self.border.width.2 = border;
        self
    }

    #[inline]
    pub fn border_left(mut self, border: f32) -> Self {
        self.border.width.3 = border;
        self
    }

    #[inline]
    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border.border_color = (border_color, border_color, border_color, border_color);
        self
    }

    #[inline]
    pub fn border_top_color(mut self, border_color: Color) -> Self {
        self.border.border_color.0 = border_color;
        self
    }

    #[inline]
    pub fn border_right_color(mut self, border_color: Color) -> Self {
        self.border.border_color.1 = border_color;
        self
    }

    #[inline]
    pub fn border_bottom_color(mut self, border_color: Color) -> Self {
        self.border.border_color.2 = border_color;
        self
    }

    #[inline]
    pub fn border_left_color(mut self, border_color: Color) -> Self {
        self.border.border_color.3 = border_color;
        self
    }

    #[inline]
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border.border_radius = radius;
        self
    }

    #[inline]
    pub fn build(self) -> NodeRender {
        NodeRender {
            selection_color: self.selection_color,
            hover_color: self.hover_color,
            border: self.border,
        }
    }
}
