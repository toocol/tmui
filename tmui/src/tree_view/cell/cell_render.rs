#![allow(dead_code)]
use crate::graphics::painter::Painter;
use derivative::Derivative;
use std::fmt::Debug;
use tlib::{
    figure::{Color, FRect},
    namespace::BorderStyle,
    Value, skia_safe::ClipOp,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellRenderType {
    Text,
    Image,
}

pub trait CellRender: Debug {
    fn render(&self, painter: &mut Painter, geometry: FRect, val: &Value);

    fn border(&self) -> (f32, f32, f32, f32);

    fn set_border(&mut self, border: (f32, f32, f32, f32));

    fn border_style(&self) -> BorderStyle;

    fn set_border_style(&mut self, style: BorderStyle);

    fn border_radius(&self) -> f32;

    fn set_border_radius(&mut self, radius: f32);

    fn background(&self) -> Option<Color>;

    fn set_background(&mut self, background: Color);

    fn ty(&self) -> CellRenderType;
}

macro_rules! cell_render_struct {
    ( $cell:ident, $builder:ident, $render_type:ident $(, $field:ident:$ty:tt)* ) => {
        #[derive(Debug)]
        pub struct $cell {
            border: (f32, f32, f32, f32),
            border_style: BorderStyle,
            border_radius: f32,
            background: Option<Color>,
            ty: CellRenderType,
            $($field: $ty),*
        }
        #[derive(Derivative)]
        #[derivative(Default)]
        pub struct $builder {
            border: (f32, f32, f32, f32),
            border_style: BorderStyle,
            border_radius: f32,
            background: Option<Color>,
            $($field: $ty),*
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
                self.background = Some(background);
                self
            }

            $(
            #[inline]
            pub fn $field(mut self, $field: $ty) -> Self {
                self.$field = $field;
                self
            }
            )*

            #[inline]
            pub fn build(self) -> Box<dyn CellRender> {
                Box::new($cell {
                    border: self.border,
                    border_style: self.border_style,
                    border_radius: self.border_radius,
                    background: self.background,
                    ty: CellRenderType::$render_type,
                    $(
                    $field: self.$field
                    ),*
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
        fn background(&self) -> Option<Color> {
            self.background
        }

        #[inline]
        fn set_background(&mut self, background: Color) {
            self.background = Some(background)
        }

        #[inline]
        fn ty(&self) -> CellRenderType {
            self.ty
        }
    };
}

cell_render_struct!(TextCellRender, TextCellRenderBuilder, Text, color:Color, letter_spacing:f32);
cell_render_struct!(ImageCellRender, ImageCellRenderBuilder, Image);

impl CellRender for TextCellRender {
    fn render(&self, painter: &mut Painter, geometry: FRect, val: &Value) {
        painter.save();
        painter.save_pen();
        painter.clip_rect(geometry, ClipOp::Intersect);

        painter.set_color(self.color);

        if let Some(background) = self.background {
            painter.fill_rect(geometry, background);
        } 

        let text = val.get::<String>();
        let origin = geometry.top_left();
        painter.draw_paragraph(&text, origin, self.letter_spacing, geometry.width(), Some(1), true);

        painter.restore_pen();
        painter.restore();
    }

    impl_cell_render_common!();
}
impl TextCellRender {
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn set_letter_spacing(&mut self, letter_spacing: f32) {
        self.letter_spacing = letter_spacing;
    }

    #[inline]
    pub fn letter_spacing(&self) -> f32 {
        self.letter_spacing
    }
}

impl CellRender for ImageCellRender {
    fn render(&self, _painter: &mut Painter, _geometry: FRect, _val: &Value) {}

    impl_cell_render_common!();
}
