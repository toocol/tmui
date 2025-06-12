use crate::{
    graphics::styles::Styles,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use tlib::figure::OptionSize;

#[cfg(not(win_tooltip))]
#[extends(Popup)]
#[derive(Childable)]
pub struct Tooltip {
    #[child]
    label: Tr<Label>,
}

#[cfg(win_tooltip)]
pub enum TooltipCrsMsg {
    /// Request window-tooltip showing.
    Show(String, OptionSize, Option<Styles>),
}

#[cfg(win_tooltip)]
#[extends(Popup)]
#[derive(Childable)]
#[tlib::win_widget(TooltipCrsMsg)]
pub struct Tooltip {
    #[child]
    label: Tr<Label>,
}

#[cfg(win_tooltip)]
impl CrossWinMsgHandler for Tooltip {
    type T = TooltipCrsMsg;

    #[inline]
    fn handle(&mut self, msg: Self::T) {
        match msg {
            TooltipCrsMsg::Show(text, size, styles) => {
                self.set_props(text.as_str(), size, styles);

                ApplicationWindow::window().layout_change(self);
            }
        }
    }
}

impl ObjectSubclass for Tooltip {
    const NAME: &'static str = "Tooltip";
}

impl ObjectImpl for Tooltip {
    #[inline]
    fn construct(&mut self) {
        self.parent_construct();

        #[cfg(not(win_tooltip))]
        {
            let window = ApplicationWindow::window();
            self.set_supervisor(window);
        }

        self.label.set_auto_wrap(true);
    }
}

impl WidgetImpl for Tooltip {}

#[cfg(not(win_tooltip))]
impl PopupImpl for Tooltip {}

impl Tooltip {
    #[inline]
    pub fn show(text: &str, position: Point, size: OptionSize, styles: Option<Styles>) {
        ApplicationWindow::window().tooltip(TooltipStrat::Show(text, position, size, styles))
    }

    #[inline]
    pub fn hide() {
        ApplicationWindow::window().tooltip(TooltipStrat::Hide)
    }

    #[inline]
    pub fn visible() -> bool {
        ApplicationWindow::window().tooltip_visible()
    }

    #[inline]
    pub fn hide_on_window_resize(on: bool) {
        ApplicationWindow::window().tooltip(TooltipStrat::HideOnWindowReisze(on));
    }
}

impl Tooltip {
    pub(crate) fn set_props(&mut self, text: &str, size: OptionSize, styles: Option<Styles>) {
        if let Some(width) = size.width() {
            self.label.width_request(width)
        }
        if let Some(height) = size.height() {
            self.label.height_request(height)
        }
        if let Some(styles) = styles {
            if let Some(halign) = styles.halign() {
                self.label.set_halign(halign)
            }
            if let Some(valign) = styles.valign() {
                self.label.set_valign(valign)
            }
            if let Some(color) = styles.color() {
                self.label.set_color(color)
            }
            self.set_styles(styles);
        }
        self.label.set_text(text);
    }
}

#[cfg(not(win_tooltip))]
impl Tooltip {
    #[inline]
    pub(crate) fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum TooltipStrat<'a> {
    Show(&'a str, Point, OptionSize, Option<Styles>),
    Hide,
    HideOnWindowReisze(bool),
}
