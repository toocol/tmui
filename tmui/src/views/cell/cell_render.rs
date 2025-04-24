#![allow(dead_code)]
use crate::{graphics::painter::Painter, icons::svg_dom::SvgDom};
use derivative::Derivative;
use log::warn;
use std::fmt::Debug;
use tlib::{
    figure::{Color, FPoint, FRect},
    global::{shown_value_32, shown_value_64},
    namespace::BorderStyle,
    skia_safe::ClipOp,
    Type, Value,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellRenderType {
    Text,
    Image,
    Svg,
}

pub trait CellRender: Debug + 'static + Send + Sync {
    fn render(&self, painter: &mut Painter, geometry: FRect, val: Option<&Value>);

    fn border(&self) -> (f32, f32, f32, f32);

    fn set_border(&mut self, border: (f32, f32, f32, f32));

    fn border_style(&self) -> BorderStyle;

    fn set_border_style(&mut self, style: BorderStyle);

    fn border_radius(&self) -> f32;

    fn set_border_radius(&mut self, radius: f32);

    fn background(&self) -> Option<Color>;

    fn set_background(&mut self, background: Color);

    fn width(&self) -> Option<u32>;

    fn set_width(&mut self, width: u32);

    fn height(&self) -> Option<u32>;

    fn set_height(&mut self, height: u32);

    fn ty(&self) -> CellRenderType;
}

macro_rules! cell_render_struct {
    ( $cell:ident, $builder:ident, $render_type:ident $(, $field:ident:$ty:tt)* ) => {
        #[derive(Debug, Clone)]
        pub struct $cell {
            border: (f32, f32, f32, f32),
            border_style: BorderStyle,
            border_radius: f32,
            background: Option<Color>,
            width: Option<u32>,
            height: Option<u32>,
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
            width: Option<u32>,
            height: Option<u32>,
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

            #[inline]
            pub fn width(mut self, width: u32) -> Self {
                self.width = Some(width);
                self
            }

            #[inline]
            pub fn height(mut self, height: u32) -> Self {
                self.width = Some(height);
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
            pub fn build(self) -> Box<$cell> {
                Box::new($cell {
                    border: self.border,
                    border_style: self.border_style,
                    border_radius: self.border_radius,
                    background: self.background,
                    width: self.width,
                    height: self.height,
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
        fn width(&self) -> Option<u32> {
            self.width
        }

        #[inline]
        fn set_width(&mut self, width: u32) {
            self.width = Some(width)
        }

        #[inline]
        fn height(&self) -> Option<u32> {
            self.height
        }

        #[inline]
        fn set_height(&mut self, height: u32) {
            self.height = Some(height)
        }

        #[inline]
        fn ty(&self) -> CellRenderType {
            self.ty
        }
    };
}

type OptSvgDom = Option<SvgDom>;

cell_render_struct!(TextCellRender, TextCellRenderBuilder, Text, color:Color, letter_spacing:f32);
cell_render_struct!(ImageCellRender, ImageCellRenderBuilder, Image);
cell_render_struct!(SvgCellRender, SvgCellRenderBuilder, Svg, dom:OptSvgDom);

impl CellRender for TextCellRender {
    fn render(&self, painter: &mut Painter, geometry: FRect, val: Option<&Value>) {
        painter.save();
        painter.save_pen();
        painter.clip_rect(geometry, ClipOp::Intersect);

        painter.set_color(self.color);

        if let Some(background) = self.background {
            painter.fill_rect(geometry, background);
        }
        let val = val.as_ref().unwrap();

        let text = match val.ty() {
            Type::STRING => val.get::<String>(),
            Type::BOOL => val.get::<bool>().to_string(),
            Type::U8 => val.get::<u8>().to_string(),
            Type::I8 => val.get::<i8>().to_string(),
            Type::U16 => val.get::<u16>().to_string(),
            Type::I16 => val.get::<i16>().to_string(),
            Type::U32 => val.get::<u32>().to_string(),
            Type::I32 => val.get::<i32>().to_string(),
            Type::U64 => val.get::<u64>().to_string(),
            Type::I64 => val.get::<i64>().to_string(),
            Type::U128 => val.get::<u128>().to_string(),
            Type::I128 => val.get::<i128>().to_string(),
            Type::F32 => shown_value_32(val.get::<f32>()),
            Type::F64 => shown_value_64(val.get::<f64>()),
            _ => "Unkonwn value.".to_string(),
        };
        let origin = geometry.top_left();
        painter.draw_paragraph(
            &text,
            origin,
            self.letter_spacing,
            geometry.width(),
            Some(1),
            true,
        );

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
    fn render(&self, _painter: &mut Painter, _geometry: FRect, _val: Option<&Value>) {}

    impl_cell_render_common!();
}

impl CellRender for SvgCellRender {
    fn render(&self, painter: &mut Painter, rect: FRect, _: Option<&Value>) {
        if let Some(dom) = self.dom.as_ref() {
            let view_size = dom.get_size();
            let (x1, y1, w1, h1) = (rect.x(), rect.y(), rect.width(), rect.height());
            let (w2, h2) = (view_size.width() as f32, view_size.height() as f32);
            let origin = FPoint::new(x1 + (w1 - w2) / 2., y1 + (h1 - h2) / 2.);
            painter.save();
            painter.translate(origin.x(), origin.y());
            painter.draw_dom(dom);
            painter.restore();
        } else {
            warn!("The `dom` of `SvgCellRender` is not assigned.");
        }
    }

    impl_cell_render_common!();
}
