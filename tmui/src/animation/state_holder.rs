use tlib::{
    figure::FRect,
    prelude::*,
    reflect_trait,
};

#[reflect_trait]
pub trait TransparencyHolder {
    fn animated_transparency(&self) -> i32;

    fn animated_transparency_mut(&mut self) -> &mut i32;
}

#[reflect_trait]
pub trait RectHolder {
    fn animated_rect(&self) -> FRect;

    fn animated_rect_mut(&mut self) -> &mut FRect;
}
