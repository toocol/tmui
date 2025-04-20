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
    icon1: Tr<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f118}')"))]
    icon2: Tr<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f2b9}')"))]
    icon3: Tr<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f0f8}')"))]
    icon4: Tr<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f247}')"))]
    icon5: Tr<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f25b}')"))]
    icon6: Tr<FontIcon>,

    #[children]
    #[derivative(Default(value = "FontIcon::new('\u{f58b}')"))]
    icon7: Tr<FontIcon>,
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
