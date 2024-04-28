use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::WinCtrlBtns;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct TitleBar {
    #[children]
    win_ctrl_btns: Box<WinCtrlBtns>
}

impl ObjectSubclass for TitleBar {
    const NAME: &'static str = "TitleBar";
}

impl ObjectImpl for TitleBar {
    fn initialize(&mut self) {
        self.set_homogeneous(false);
        self.set_background(Color::GREY_LIGHT);
        self.set_hexpand(true);
        self.height_request(30);
    }
}

impl WidgetImpl for TitleBar {}