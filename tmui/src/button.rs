use tlib::{events::MouseEvent, namespace::Overflow};

use crate::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

const DEFAULT_BORDER_COLOR: Color = Color::grey_with(96);
const DEFAULT_BACKGROUND_NORMAL: Color = Color::grey_with(235);
const DEFAULT_BACKGROUND_HOVER: Color = Color::grey_with(225);

#[extends(Widget)]
#[derive(Childable)]
pub struct Button {
    #[child]
    label: Tr<Label>,
}

impl ObjectSubclass for Button {
    const NAME: &'static str = "Button";
}

impl ObjectImpl for Button {
    fn initialize(&mut self) {
        self.set_border_radius(2.);
        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(DEFAULT_BORDER_COLOR);
        self.set_background(DEFAULT_BACKGROUND_NORMAL);
        self.enable_bubble(EventBubble::MOUSE_PRESSED);
        self.enable_bubble(EventBubble::MOUSE_RELEASED);
        self.enable_bubble(EventBubble::MOUSE_MOVE);

        self.label.set_overflow(Overflow::Hidden);
        self.label.set_halign(Align::Center);
        self.label.set_valign(Align::Center);
    }
}

impl WidgetImpl for Button {
    #[inline]
    fn on_mouse_enter(&mut self, _: &MouseEvent) {
        self.set_background(DEFAULT_BACKGROUND_HOVER);
    }

    #[inline]
    fn on_mouse_leave(&mut self, _: &MouseEvent) {
        self.set_background(DEFAULT_BACKGROUND_NORMAL);
    }
}

impl Button {
    #[inline]
    pub fn new(text: Option<&str>) -> Tr<Self> {
        let mut button = Self::new_alloc();
        if let Some(text) = text {
            button.label.set_text(text);
        }
        button
    }
}
