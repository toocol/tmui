use crate::asset::Asset;
use tmui::{
    icons::{svg_icon::SvgIcon, svg_toggle_icon::SvgToggleIcon},
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct WinCtrlBtns {
    #[derivative(Default(value = "{
        let file = Asset::get(\"minimize.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    minimize: Box<SvgIcon>,

    #[derivative(Default(value = "{
        let maximize = Asset::get(\"large.svg\").unwrap();
        let restore = Asset::get(\"restore.svg\").unwrap();
        SvgToggleIcon::from_bytes(&[maximize.data.as_ref(), restore.data.as_ref()])
    }"))]
    #[children]
    large_restore: Box<SvgToggleIcon>,

    #[derivative(Default(value = "{
        let file = Asset::get(\"close.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    close: Box<SvgIcon>,
}

impl ObjectSubclass for WinCtrlBtns {
    const NAME: &'static str = "WinCtrlBtns";
}

impl ObjectImpl for WinCtrlBtns {
    fn initialize(&mut self) {
        self.set_halign(Align::End);
        self.set_vexpand(true);
        self.width_request(138);

        self.minimize.width_request(46);
        self.minimize.height_request(30);

        self.large_restore.width_request(46);
        self.large_restore.height_request(30);

        self.close.width_request(46);
        self.close.height_request(30);
    }
}

impl WidgetImpl for WinCtrlBtns {}
