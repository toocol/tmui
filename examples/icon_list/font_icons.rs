use tmui::{
    icons::font_icon::FontIcon,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct FontIcons {
    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f007}')"))]
    icon1: Box<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f118}')"))]
    icon2: Box<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f2b9}')"))]
    icon3: Box<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f0f8}')"))]
    icon4: Box<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f247}')"))]
    icon5: Box<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f25b}')"))]
    icon6: Box<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f58b}')"))]
    icon7: Box<FontIcon>,
}

impl ObjectSubclass for FontIcons {
    const NAME: &'static str = "FontIcons";
}

impl ObjectImpl for FontIcons {
    fn initialize(&mut self) {
        self.set_spacing(20);

        self.icon1.set_margin_left(20);
    }
}

impl WidgetImpl for FontIcons {}
