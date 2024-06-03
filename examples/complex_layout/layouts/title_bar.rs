use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::{app_icon::AppIcon, WinCtrlBtns};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct TitleBar {
    #[children]
    app_icon: Box<AppIcon>,

    #[children]
    widget2: Box<Widget>,

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

        self.app_icon.width_request(30);
        self.app_icon.set_vexpand(true);
        self.app_icon.set_background(Color::WHITE);

        self.widget2.set_hexpand(true);
        self.widget2.set_vexpand(true);
        self.widget2.set_hscale(0.7);
        self.widget2.set_background(Color::GREY_LIGHT);

        self.win_ctrl_btns.set_background(Color::WHITE);
    }
}

impl WidgetImpl for TitleBar {}