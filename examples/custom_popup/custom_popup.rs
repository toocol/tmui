use std::time::Duration;

use tlib::run_after;
use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Popup)]
#[async_task(name = "TestAsyncTask", value = "&'static str")]
#[animatable(ty = "Linear", direction = "BottomToTop", duration = 500, mode = "Flex")]
#[derive(Childable)]
#[run_after]
pub struct CustomPopup {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for CustomPopup {
    const NAME: &'static str = "CustomPopup";
}

impl ObjectImpl for CustomPopup {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_hexpand(true);
        self.set_vexpand(true);

        self.width_request(100);
        self.height_request(40);
        self.label.set_halign(Align::Center);
        self.label.set_valign(Align::Center);

        self.test_async_task(
            async {
                tokio::time::sleep(Duration::from_secs(5)).await;
                "This is a popup."
            },
            Some(|p: &mut CustomPopup, text| {
                p.label.set_text(text);
            }),
        );
    }
}

impl WidgetImpl for CustomPopup {
    fn run_after(&mut self) {
        self.parent_run_after();

        println!("CustomPopup run after.")
    }
}

impl PopupImpl for CustomPopup {}

impl CustomPopup {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
