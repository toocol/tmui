use tlib::figure::OptionSize;

use crate::{
    graphics::styles::Styles,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Popup, internal = true)]
#[derive(Childable)]
pub struct Tooltip {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for Tooltip {
    const NAME: &'static str = "Tooltip";
}

impl ObjectImpl for Tooltip {
    #[inline]
    fn construct(&mut self) {
        self.parent_construct();

        let window = ApplicationWindow::window();
        self.set_supervisor(window);

        self.label.set_auto_wrap(true);
    }
}

impl WidgetImpl for Tooltip {}

impl PopupImpl for Tooltip {}

impl Tooltip {
    #[inline]
    pub fn show<'a>(text: &'a str, position: Point, size: OptionSize, styles: Option<Styles>) {
        ApplicationWindow::window().tooltip(TooltipStrat::Show(text, position, size, styles))
    }

    #[inline]
    pub fn hide() {
        ApplicationWindow::window().tooltip(TooltipStrat::Hide)
    }
}

impl Tooltip {
    #[inline]
    pub(crate) fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub(crate) fn set_text<'a>(&mut self, text: &'a str) {
        self.label.set_text(text)
    }

    #[inline]
    pub(crate) fn set_color(&mut self, color: Color) {
        self.label.set_color(color)
    }

    #[inline]
    pub(crate) fn label(&mut self) -> &mut Label {
        self.label.as_mut()
    }
}

pub(crate) enum TooltipStrat<'a> {
    Show(&'a str, Point, OptionSize, Option<Styles>),
    Hide,
}