use tlib::{
    figure::{Color, Rect},
    prelude::*,
    reflect_trait,
};

#[reflect_trait]
pub trait ColorHolder {
    fn animated_color(&self) -> Color;

    fn animated_color_mut(&mut self) -> &mut Color;
}

#[reflect_trait]
pub trait RectHolder {
    fn animated_rect(&self) -> Rect;

    fn animated_rect_mut(&mut self) -> &mut Rect;
}
