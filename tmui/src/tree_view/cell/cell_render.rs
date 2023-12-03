#![allow(dead_code)]
use std::fmt::Debug;
use crate::graphics::painter::Painter;
use derivative::Derivative;
use tlib::{
    figure::{Color, Rect},
    namespace::BorderStyle,
    Value,
};

pub const DEFAULT_CELL_BACKGROUND: Color = Color::WHITE;
pub const DEFAULT_CELL_FOREGROUND: Color = Color::BLACK;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellRenderType {
    Text,
    Image,
}

pub trait CellRender: Debug {
    fn render(&self, painter: &mut Painter, geometry: Rect, val: &Value);

    fn border(&self) -> (f32, f32, f32, f32);

    fn set_border(&mut self, border: (f32, f32, f32, f32));

    fn border_style(&self) -> BorderStyle;

    fn set_border_style(&mut self, style: BorderStyle);

    fn border_radius(&self) -> f32;

    fn set_border_radius(&mut self, radius: f32);

    fn background(&self) -> Color;

    fn set_background(&mut self, background: Color);

    fn foreground(&self) -> Color;

    fn set_foreground(&mut self, foreground: Color);

    fn ty(&self) -> CellRenderType;
}

macro_rules! cell_render_struct {
    ( $cell:ident, $builder:ident, $render_type:ident ) => {
        #[derive(Debug)]
        pub struct $cell {
            border: (f32, f32, f32, f32),
            border_style: BorderStyle,
            border_radius: f32,
            background: Color,
            foreground: Color,
            ty: CellRenderType,
        }
        #[derive(Derivative)]
        #[derivative(Default)]
        pub struct $builder {
            border: (f32, f32, f32, f32),
            border_style: BorderStyle,
            border_radius: f32,
            #[derivative(Default(value = "DEFAULT_CELL_BACKGROUND"))]
            background: Color,
            #[derivative(Default(value = "DEFAULT_CELL_FOREGROUND"))]
            foreground: Color,
        }
        impl $cell {
            #[inline]
            pub fn builder() -> $builder {
                $builder::default()
            }
        }
        impl $builder {
            #[inline]
            pub fn border(mut self, border: (f32, f32, f32, f32)) -> Self {
                self.border = border;
                self
            }

            #[inline]
            pub fn border_style(mut self, style: BorderStyle) -> Self {
                self.border_style = style;
                self
            }

            #[inline]
            pub fn border_radius(mut self, radius: f32) -> Self {
                self.border_radius = radius;
                self
            }

            #[inline]
            pub fn background(mut self, background: Color) -> Self {
                self.background = background;
                self
            }

            #[inline]
            pub fn foreground(mut self, foreground: Color) -> Self {
                self.foreground = foreground;
                self
            }

            #[inline]
            pub fn build(self) -> Box<dyn CellRender> {
                Box::new($cell {
                    border: self.border,
                    border_style: self.border_style,
                    border_radius: self.border_radius,
                    background: self.background,
                    foreground: self.foreground,
                    ty: CellRenderType::$render_type,
                })
            }
        }
    };
}

macro_rules! impl_cell_render_common {
    () => {
        #[inline]
        fn border(&self) -> (f32, f32, f32, f32) {
            self.border
        }

        #[inline]
        fn set_border(&mut self, border: (f32, f32, f32, f32)) {
            self.border = border
        }

        #[inline]
        fn border_style(&self) -> BorderStyle {
            self.border_style
        }

        #[inline]
        fn set_border_style(&mut self, style: BorderStyle) {
            self.border_style = style
        }

        #[inline]
        fn border_radius(&self) -> f32 {
            self.border_radius
        }

        #[inline]
        fn set_border_radius(&mut self, radius: f32) {
            self.border_radius = radius
        }

        #[inline]
        fn background(&self) -> Color {
            self.background
        }

        #[inline]
        fn set_background(&mut self, background: Color) {
            self.background = background
        }

        #[inline]
        fn foreground(&self) -> Color {
            self.foreground
        }

        #[inline]
        fn set_foreground(&mut self, foreground: Color) {
            self.foreground = foreground
        }

        #[inline]
        fn ty(&self) -> CellRenderType {
            self.ty
        }
    };
}

cell_render_struct!(TextCellRender, TextCellRenderBuilder, Text);
cell_render_struct!(ImageCellRender, ImageCellRenderBuilder, Image);

impl CellRender for TextCellRender {
    fn render(&self, painter: &mut Painter, geometry: Rect, val: &Value) {
    }

    impl_cell_render_common!();
}

impl CellRender for ImageCellRender {
    fn render(&self, painter: &mut Painter, geometry: Rect, val: &Value) {
    }

    impl_cell_render_common!();
}
