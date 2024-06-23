pub mod node_render;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Status {
    #[default]
    Default = 0,
    Selected = 1,
    Hovered = 2,
}