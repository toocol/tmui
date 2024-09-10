use tlib::{bitflags::bitflags, figure::{Color, FRect}};

pub mod node_render;

// #[repr(u8)]
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
// pub enum Status {
//     #[default]
//     Default = 0,
//     Selected = 1,
//     Hovered = 2,
// }

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Status: u8 {
        const Selected = 1;
        const Hovered = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MouseEffect: u8 {
        const Hovered = 1;
        const Selected = 1 << 1;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RenderCtx {
    pub(crate) geometry: FRect,
    pub(crate) background: Color,
    pub(crate) mouse_effect: MouseEffect,
}
impl RenderCtx {
    #[inline]
    pub(crate) fn new(geometry: FRect, background: Color, mouse_effect: MouseEffect) -> Self {
        Self {
            geometry,
            background,
            mouse_effect,
        }
    }
}