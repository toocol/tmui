use derivative::Derivative;
use tlib::{
    figure::{Color, Rect},
    skia_safe::ClipOp,
};
use crate::graphics::painter::Painter;

use super::tree_node::Status;

const DEFAULT_SELECTION: Color = Color::from_rgb(51, 167, 255);
const DEFAULT_HOVER: Color = Color::from_rgb(190, 190, 190);


#[derive(Derivative)]
#[derivative(Default)]
pub struct NodeRender {
    #[derivative(Default(value = "DEFAULT_SELECTION"))]
    selection_color: Color,
    #[derivative(Default(value = "DEFAULT_HOVER"))]
    hover_color: Color,
    border: (f32, f32, f32, f32),
    border_radius: f32,
}

impl NodeRender {
    #[inline]
    pub fn builder() -> NodeRenderBuilder {
        NodeRenderBuilder::default()
    }
}

impl NodeRender {
    pub(crate) fn render(&self, painter: &mut Painter, geometry: Rect, background: Color, status: Status) {
        painter.save();
        painter.clip_rect(geometry, ClipOp::Intersect);

        let background = match status {
            Status::Default => background,
            Status::Selected => self.selection_color,
            Status::Hovered => self.hover_color,
        };

        painter.fill_rect(geometry, background);

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
    border: (f32, f32, f32, f32),
    border_radius: f32,
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
    pub fn border(mut self, border: (f32, f32, f32, f32)) -> Self {
        self.border = border;
        self
    }

    #[inline]
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    #[inline]
    pub fn build(self) -> NodeRender {
        NodeRender {
            selection_color: self.selection_color,
            hover_color: self.hover_color,
            border: self.border,
            border_radius: self.border_radius,
        }
    }
}
