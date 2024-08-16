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

        self.label.set_hexpand(true);
        self.label.set_vexpand(true);
    }
}

impl WidgetImpl for Tooltip {}

impl PopupImpl for Tooltip {}

impl Tooltip {
    #[inline]
    pub fn show<'a>(text: &'a str, geometry: Rect, styles: Option<Styles>) {
        ApplicationWindow::window().tooltip(TooltipStrat::Show(text, geometry, styles))
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
}

pub(crate) enum TooltipStrat<'a> {
    Show(&'a str, Rect, Option<Styles>),
    Hide,
}