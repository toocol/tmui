#![allow(dead_code)]
use crate::{graphics::painter::Painter, icons::svg_dom::SvgDom, views::node::Status};
use derivative::Derivative;
use log::warn;
use std::fmt::Debug;
use tlib::{
    figure::{Color, FPoint, FRect},
    global::{shown_value_32, shown_value_64},
    namespace::BorderStyle,
    prelude::Align,
    skia_safe::{
        textlayout::{
            FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider,
        },
        ClipOp,
    },
    Type, Value,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellRenderType {
    Text,
    Image,
    Svg,
}

pub trait CellRender: Debug + 'static + Send + Sync {
    fn render(&self, painter: &mut Painter, geometry: FRect, val: Option<&Value>, node_status: Status);

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

    fn halign(&self) -> Align;

    fn set_halign(&mut self, halign: Align);

    fn valign(&self) -> Align;

    fn set_valign(&mut self, valign: Align);

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
            halign: Align,
            valign: Align,
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
            halign: Align,
            valign: Align,
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

            #[inline]
            pub fn halign(mut self, halign: Align) -> Self {
                self.halign = halign;
                self
            }

            #[inline]
            pub fn valign(mut self, valign: Align) -> Self {
                self.valign = valign;
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
                    halign: self.halign,
                    valign: self.valign,
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
        fn halign(&self) -> Align {
            self.halign
        }

        #[inline]
        fn set_halign(&mut self, halign: Align) {
            self.halign = halign
        }

        #[inline]
        fn valign(&self) -> Align {
            self.valign
        }

        #[inline]
        fn set_valign(&mut self, valign: Align) {
            self.valign = valign
        }

        #[inline]
        fn ty(&self) -> CellRenderType {
            self.ty
        }
    };
}

type OptSvgDom = Option<SvgDom>;
type OptColor = Option<Color>;

cell_render_struct!(TextCellRender, TextCellRenderBuilder, Text, color:Color, hover_color:OptColor, selection_color:OptColor, letter_spacing:f32);
cell_render_struct!(ImageCellRender, ImageCellRenderBuilder, Image);
cell_render_struct!(SvgCellRender, SvgCellRenderBuilder, Svg, dom:OptSvgDom, hover_dom:OptSvgDom, selection_dom:OptSvgDom);

impl CellRender for TextCellRender {
    fn render(&self, painter: &mut Painter, geometry: FRect, val: Option<&Value>, status: Status) {
        painter.save();
        painter.save_pen();
        painter.clip_rect(geometry, ClipOp::Intersect);

        if status.contains(Status::Selected) {
            if let Some(color) = self.selection_color {
                painter.set_color(color);
            } else {
                painter.set_color(self.color);
            }
        } else if status.contains(Status::Hovered) {
            if let Some(color) = self.hover_color {
                painter.set_color(color);
            } else {
                painter.set_color(self.color);
            }
        } else {
            painter.set_color(self.color);
        }

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
        let mut draw_point = geometry.top_left();
        let (paragraph_width, paragraph_height) =
            self.calc_paragraph_size(painter, &text, geometry);
        match self.halign() {
            Align::Start => {}
            Align::Center => {
                let offset = (geometry.width() - paragraph_width) / 2.;
                if offset > 0. {
                    draw_point.set_x(draw_point.x() + offset);
                }
            }
            Align::End => {
                let offset = geometry.width() - paragraph_width;
                if offset > 0. {
                    draw_point.set_x(draw_point.x() + offset);
                }
            }
        };
        match self.valign() {
            Align::Start => {}
            Align::Center => {
                let offset = (geometry.height() - paragraph_height) / 2.;
                if offset > 0. {
                    draw_point.set_y(draw_point.y() + offset);
                }
            }
            Align::End => {
                let offset = geometry.height() - paragraph_height;
                if offset > 0. {
                    draw_point.set_y(draw_point.y() + offset);
                }
            }
        };
        painter.draw_paragraph(
            &text,
            draw_point,
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

    fn calc_paragraph_size(&self, painter: &mut Painter, text: &str, rect: FRect) -> (f32, f32) {
        let font = painter.font().unwrap();

        let mut typeface_provider = TypefaceFontProvider::new();
        let mut families = vec![];
        for tf in font.typefaces() {
            let typeface = tf.to_skia_typeface(font);

            if let Some(typeface) = typeface {
                families.push(tf.family());
                let family = typeface.family_name();
                typeface_provider.register_typeface(typeface, Some(family.as_str()));
            }
        }

        let mut font_collection = FontCollection::new();
        font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

        // define text style
        let mut style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        text_style.set_font_size(font.size());
        text_style.set_font_families(&families);
        text_style.set_letter_spacing(self.letter_spacing);
        style.set_text_style(&text_style);
        style.set_max_lines(Some(1));
        style.set_ellipsis("\u{2026}");

        // layout the paragraph
        let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
        paragraph_builder.add_text(text);
        let mut paragraph = paragraph_builder.build();

        let width = rect.width();
        let layout = if width == 0. { f32::MAX } else { width };
        paragraph.layout(layout);

        (
            paragraph.max_intrinsic_width().ceil(),
            paragraph.height().ceil(),
        )
    }
}

impl CellRender for ImageCellRender {
    fn render(&self, _painter: &mut Painter, _geometry: FRect, _val: Option<&Value>, _status: Status) {}

    impl_cell_render_common!();
}

impl CellRender for SvgCellRender {
    fn render(&self, painter: &mut Painter, rect: FRect, _: Option<&Value>, status: Status) {
        let dom = if status.contains(Status::Selected) {
            if self.selection_dom.is_some() {
                self.selection_dom.as_ref()
            } else {
                self.dom.as_ref()
            }
        } else if status.contains(Status::Hovered) {
            if self.hover_dom.is_some() {
                self.hover_dom.as_ref()
            } else {
                self.dom.as_ref()
            }
        } else {
            self.dom.as_ref()
        };
        if let Some(dom) = dom {
            let view_size = dom.get_size();
            let (x1, y1, w1, h1) = (rect.x(), rect.y(), rect.width(), rect.height());
            let (w2, h2) = (view_size.width() as f32, view_size.height() as f32);
            let origin = FPoint::new(x1 + (w1 - w2) / 2., y1 + (h1 - h2) / 2.);
            painter.save();
            painter.clip_rect(rect, ClipOp::Intersect);
            painter.translate(origin.x(), origin.y());
            painter.draw_dom(dom);
            painter.restore();
        } else {
            warn!("The `dom` of `SvgCellRender` is not assigned.");
        }
    }

    impl_cell_render_common!();
}
