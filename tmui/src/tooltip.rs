use tlib::figure::OptionSize;
use crate::{
    graphics::styles::Styles,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[cfg(not(win_popup))]
#[extends(Popup, internal = true)]
#[derive(Childable)]
pub struct Tooltip {
    #[child]
    label: Box<Label>,
}

#[cfg(win_popup)]
pub enum TooltipCrsMsg {
    /// Request window-tooltip showing.
    Show(String, Point, OptionSize, Option<Styles>),
}

#[cfg(win_popup)]
#[extends(Popup, internal = true)]
#[derive(Childable)]
#[tlib::win_widget(TooltipCrsMsg)]
pub struct Tooltip {
    #[child]
    label: Box<Label>,
}

#[cfg(win_popup)]
impl CrossWinMsgHandler for Tooltip {
    type T = TooltipCrsMsg;

    #[inline]
    fn handle(&mut self, msg: Self::T) {
        match msg {
            TooltipCrsMsg::Show(text, position, size, styles) => {
                self.set_props(text.as_str(), position, size, styles)
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

        let window = ApplicationWindow::window();
        self.set_supervisor(window);

        self.label.set_auto_wrap(true);
    }
}

impl WidgetImpl for Tooltip {}

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
}

impl Tooltip {
    pub(crate) fn set_props(
        &mut self,
        text: &str,
        position: Point,
        size: OptionSize,
        styles: Option<Styles>,
    ) {
        self.set_fixed_x(position.x());
        self.set_fixed_y(position.y());

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

#[cfg(not(win_popup))]
impl Tooltip {
    #[inline]
    pub(crate) fn new() -> Box<Self> {
        Object::new(&[])
    }
}

pub(crate) enum TooltipStrat<'a> {
    Show(&'a str, Point, OptionSize, Option<Styles>),
    Hide,
}
