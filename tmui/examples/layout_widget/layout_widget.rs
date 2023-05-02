#![allow(dead_code)]
use tlib::object::ObjectSubclass;
use tmui::{
    graphics::figure::{FontTypeface, FontWidth},
    label::Label,
    prelude::*,
};

#[extends(Widget, Layout(Stack))]
#[derive(Default, Layout)]
pub struct CustomWidget {
    #[children]
    label: Label,
}

impl ObjectSubclass for CustomWidget {
    const NAME: &'static str = "CustomWidget";
}

impl ObjectImpl for CustomWidget {
    fn construct(&mut self) {
        self.parent_construct();
        self.label.set_background(Color::CYAN);
        self.label.construct();
        self.label.set_text("Hello World");
        let mut font = self.label.font();
        font.set_typeface(
            FontTypeface::builder()
                .family("Consolas")
                .width(FontWidth::UltraCondensed)
                .italic(true)
                .build(),
        );
        font.set_size(20.);
        self.label.set_font(font);
        self.label.set_text_halign(Align::Center);
        self.label.set_text_valign(Align::Center);
        self.label.set_halign(Align::Center);
        self.label.set_valign(Align::Center);
        self.label.width_request(200);
        self.label.height_request(100);

        self.set_background(Color::RED);
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);
        self.width_request(500);
        self.height_request(300);
    }
}

impl WidgetImpl for CustomWidget {}

impl CustomWidget {
    pub fn new() -> Self {
        Object::new(&[])
    }
}
