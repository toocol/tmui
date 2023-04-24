use crate::widget::WidgetImpl;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Composition {
    #[default]
    Default,
    Overlay,
    Arrange,
    Float,
}

pub trait Layout {
    fn composition(&self) -> Composition;

    fn position_layout(&mut self, previous: &dyn WidgetImpl, parent: &dyn WidgetImpl);
}