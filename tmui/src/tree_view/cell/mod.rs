pub mod cell_render;

use self::cell_render::{CellRender, CellRenderType, CellRenderType::*};
use crate::graphics::painter::Painter;
use tlib::{figure::FRect, types::StaticType, values::ToValue, Type, Value};

/// The data cell of a TreeNode, one TreeNode represents one row of an image, 
/// and one Cell corresponds to one column of the image.
#[rustfmt::skip]
pub enum Cell {
    String { expand: bool, val: Value, render: Box<dyn CellRender> },
    Bool { expand: bool, val: Value, render: Box<dyn CellRender> },
    U8 { expand: bool, val: Value, render: Box<dyn CellRender> },
    I8 { expand: bool, val: Value, render: Box<dyn CellRender> },
    U16 { expand: bool, val: Value, render: Box<dyn CellRender> },
    I16 { expand: bool, val: Value, render: Box<dyn CellRender> },
    U32 { expand: bool, val: Value, render: Box<dyn CellRender> },
    I32 { expand: bool, val: Value, render: Box<dyn CellRender> },
    U64 { expand: bool, val: Value, render: Box<dyn CellRender> },
    I64 { expand: bool, val: Value, render: Box<dyn CellRender> },
    U128 { expand: bool, val: Value, render: Box<dyn CellRender> },
    I128 { expand: bool, val: Value, render: Box<dyn CellRender> },
    F32 { expand: bool, val: Value, render: Box<dyn CellRender> },
    F64 { expand: bool, val: Value, render: Box<dyn CellRender> },
    Image { expand: bool, image_address: Value, render: Box<dyn CellRender> },
}

impl Cell {
    pub(crate) fn render_cell(&self, painter: &mut Painter, geometry: FRect) {
        match self {
            Self::String { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::Bool { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::U8 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::I8 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::U16 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::I16 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::U32 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::I32 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::U64 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::I64 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::U128 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::I128 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::F32 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::F64 { val, render, .. } => {
                render.render(painter, geometry, val);
            }
            Self::Image {
                image_address,
                render,
                ..
            } => {
                render.render(painter, geometry, image_address);
            }
        }
    }

    pub(crate) fn type_(&self) -> Type {
        match self {
            Self::String { .. } => String::static_type(),
            Self::Bool { .. } => bool::static_type(),
            Self::U8 { .. } => u8::static_type(),
            Self::I8 { .. } => i8::static_type(),
            Self::U16 { .. } => u16::static_type(),
            Self::I16 { .. } => i16::static_type(),
            Self::U32 { .. } => u32::static_type(),
            Self::I32 { .. } => i32::static_type(),
            Self::U64 { .. } => u64::static_type(),
            Self::I64 { .. } => i64::static_type(),
            Self::U128 { .. } => u128::static_type(),
            Self::I128 { .. } => i128::static_type(),
            Self::F32 { .. } => f32::static_type(),
            Self::F64 { .. } => f64::static_type(),
            Self::Image { .. } => String::static_type(),
        }
    }

    pub(crate) fn value(&self) -> &Value {
        match self {
            Self::String { val, .. } => val,
            Self::Bool { val, .. } => val,
            Self::U8 { val, .. } => val,
            Self::I8 { val, .. } => val,
            Self::U16 { val, .. } => val,
            Self::I16 { val, .. } => val,
            Self::U32 { val, .. } => val,
            Self::I32 { val, .. } => val,
            Self::U64 { val, .. } => val,
            Self::I64 { val, .. } => val,
            Self::U128 { val, .. } => val,
            Self::I128 { val, .. } => val,
            Self::F32 { val, .. } => val,
            Self::F64 { val, .. } => val,
            Self::Image { image_address, .. } => image_address,
        }
    }

    pub(crate) fn set_value(&mut self, value: Value) {
        match self {
            Self::String { val, .. } => *val = value,
            Self::Bool { val, .. } => *val = value,
            Self::U8 { val, .. } => *val = value,
            Self::I8 { val, .. } => *val = value,
            Self::U16 { val, .. } => *val = value,
            Self::I16 { val, .. } => *val = value,
            Self::U32 { val, .. } => *val = value,
            Self::I32 { val, .. } => *val = value,
            Self::U64 { val, .. } => *val = value,
            Self::I64 { val, .. } => *val = value,
            Self::U128 { val, .. } => *val = value,
            Self::I128 { val, .. } => *val = value,
            Self::F32 { val, .. } => *val = value,
            Self::F64 { val, .. } => *val = value,
            Self::Image { image_address, .. } => *image_address = value,
        }
    }

    pub(crate) fn get_render(&self) -> &dyn CellRender {
        match self {
            Self::String { render, .. } => render.as_ref(),
            Self::Bool { render, .. } => render.as_ref(),
            Self::U8 { render, .. } => render.as_ref(),
            Self::I8 { render, .. } => render.as_ref(),
            Self::U16 { render, .. } => render.as_ref(),
            Self::I16 { render, .. } => render.as_ref(),
            Self::U32 { render, .. } => render.as_ref(),
            Self::I32 { render, .. } => render.as_ref(),
            Self::U64 { render, .. } => render.as_ref(),
            Self::I64 { render, .. } => render.as_ref(),
            Self::U128 { render, .. } => render.as_ref(),
            Self::I128 { render, .. } => render.as_ref(),
            Self::F32 { render, .. } => render.as_ref(),
            Self::F64 { render, .. } => render.as_ref(),
            Self::Image { render, .. } => render.as_ref(),
        }
    }

    pub(crate) fn get_render_mut(&mut self) -> &mut dyn CellRender {
        match self {
            Self::String { render, .. } => render.as_mut(),
            Self::Bool { render, .. } => render.as_mut(),
            Self::U8 { render, .. } => render.as_mut(),
            Self::I8 { render, .. } => render.as_mut(),
            Self::U16 { render, .. } => render.as_mut(),
            Self::I16 { render, .. } => render.as_mut(),
            Self::U32 { render, .. } => render.as_mut(),
            Self::I32 { render, .. } => render.as_mut(),
            Self::U64 { render, .. } => render.as_mut(),
            Self::I64 { render, .. } => render.as_mut(),
            Self::U128 { render, .. } => render.as_mut(),
            Self::I128 { render, .. } => render.as_mut(),
            Self::F32 { render, .. } => render.as_mut(),
            Self::F64 { render, .. } => render.as_mut(),
            Self::Image { render, .. } => render.as_mut(),
        }
    }

    pub(crate) fn support_render_types(&self) -> Vec<CellRenderType> {
        match self {
            Self::String { .. } => vec![Text],
            Self::Bool { .. } => vec![Text],
            Self::U8 { .. } => vec![Text],
            Self::I8 { .. } => vec![Text],
            Self::U16 { .. } => vec![Text],
            Self::I16 { .. } => vec![Text],
            Self::U32 { .. } => vec![Text],
            Self::I32 { .. } => vec![Text],
            Self::U64 { .. } => vec![Text],
            Self::I64 { .. } => vec![Text],
            Self::U128 { .. } => vec![Text],
            Self::I128 { .. } => vec![Text],
            Self::F32 { .. } => vec![Text],
            Self::F64 { .. } => vec![Text],
            Self::Image { .. } => vec![Image],
        }
    }
}

macro_rules! cell_builder {
    ( $name:ident, $ty:ident, $cty:ident ) => {
        #[derive(Default, Debug)]
        pub struct $name {
            expand: bool,
            val: $ty,
            cell_render: Option<Box<dyn CellRender>>,
        }
        impl $name {
            #[inline]
            pub fn expand(mut self, expand: bool) -> Self {
                self.expand = expand;
                self
            }

            #[inline]
            pub fn value(mut self, val: $ty) -> Self {
                self.val = val;
                self
            }

            #[inline]
            pub fn cell_render(mut self, render: Box<dyn CellRender>) -> Self {
                self.cell_render = Some(render);
                self
            }

            #[inline]
            pub fn build(self) -> Cell {
                if self.cell_render.is_none() {
                    panic!("`cell_render` of Cell can not be None.");
                }
                let cell_render = self.cell_render.unwrap();
                let render_ty = cell_render.ty();
                let cell = Cell::$cty {
                    expand: self.expand,
                    val: self.val.to_value(),
                    render: cell_render,
                };
                if !cell.support_render_types().contains(&render_ty) {
                    panic!("Unsupported cell render type.");
                }
                cell
            }
        }
    };
}

cell_builder!(CellStringBuilder, String, String);
cell_builder!(CellBoolBuilder, bool, Bool);
cell_builder!(CellU8Builder, u8, U8);
cell_builder!(CellI8Builder, i8, I8);
cell_builder!(CellU16Builder, u16, U16);
cell_builder!(CellI16Builder, i16, I16);
cell_builder!(CellU32Builder, u32, U32);
cell_builder!(CellI32Builder, i32, I32);
cell_builder!(CellU64Builder, u64, U64);
cell_builder!(CellI64Builder, i64, I64);
cell_builder!(CellU128Builder, u128, U128);
cell_builder!(CellI128Builder, i128, I128);
cell_builder!(CellF32Builder, f32, F32);
cell_builder!(CellF64Builder, f64, F64);

#[derive(Default, Debug)]
pub struct CellImageBuilder {
    expand: bool,
    image_address: String,
    cell_render: Option<Box<dyn CellRender>>,
}
impl CellImageBuilder {
    #[inline]
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    #[inline]
    pub fn image_address(mut self, image_address: String) -> Self {
        self.image_address = image_address;
        self
    }

    #[inline]
    pub fn cell_render(mut self, render: Box<dyn CellRender>) -> Self {
        self.cell_render = Some(render);
        self
    }

    #[inline]
    pub fn build(self) -> Cell {
        if self.cell_render.is_none() {
            panic!("`cell_render` of Cell can not be None.");
        }
        let cell_render = self.cell_render.unwrap();
        let render_ty = cell_render.ty();
        let cell = Cell::Image {
            expand: self.expand,
            image_address: self.image_address.to_value(),
            render: cell_render,
        };
        if !cell.support_render_types().contains(&render_ty) {
            panic!("Unsupported cell render type.");
        }
        cell
    }
}

macro_rules! cell_builder_func {
    ( $name:ident, $builder:ident ) => {
        #[inline]
        pub fn $name() -> $builder {
            $builder::default()
        }
    };
}

impl Cell {
    cell_builder_func!(string, CellStringBuilder);
    cell_builder_func!(bool, CellBoolBuilder);
    cell_builder_func!(u8, CellU8Builder);
    cell_builder_func!(i8, CellI8Builder);
    cell_builder_func!(u16, CellU16Builder);
    cell_builder_func!(i16, CellI16Builder);
    cell_builder_func!(u32, CellU32Builder);
    cell_builder_func!(i32, CellI32Builder);
    cell_builder_func!(u64, CellU64Builder);
    cell_builder_func!(i64, CellI64Builder);
    cell_builder_func!(u128, CellU128Builder);
    cell_builder_func!(i128, CellI128Builder);
    cell_builder_func!(f32, CellF32Builder);
    cell_builder_func!(f64, CellF64Builder);
    cell_builder_func!(image, CellImageBuilder);
}
