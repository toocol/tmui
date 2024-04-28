use tmui::{
    icons::svg_icon::SvgIcon, prelude::*, tlib::object::{ObjectImpl, ObjectSubclass}, widget::WidgetImpl
};

use crate::ctrl_button::CtrlButtons;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct SvgIcons {
    #[children]
    #[derivative(Default(value = "SvgIcon::from_file(\"examples/resources/search.svg\")"))]
    icon1: Box<SvgIcon>,

    #[children]
    #[derivative(Default(value = "SvgIcon::from_file(\"examples/resources/toggle_on.svg\")"))]
    icon2: Box<SvgIcon>,

    #[children]
    #[derivative(Default(value = "SvgIcon::from_file(\"examples/resources/globe_asia.svg\")"))]
    icon3: Box<SvgIcon>,

    #[children]
    #[derivative(Default(value = "SvgIcon::from_file(\"examples/resources/sword_rose.svg\")"))]
    icon4: Box<SvgIcon>,

    #[children]
    icon5: Box<CtrlButtons>,
}

impl ObjectSubclass for SvgIcons {
    const NAME: &'static str = "SvgIcons";
}

impl ObjectImpl for SvgIcons {
    fn initialize(&mut self) {
        self.set_spacing(30);

        self.icon1.set_margin_left(20);
    }
}

impl WidgetImpl for SvgIcons {}