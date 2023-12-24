use std::time::Duration;

use tlib::run_after;
use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

/// Transparency based animation popup.
#[extends(Popup)]
#[async_task(name = "TestAsyncTask", value = "&'static str")]
#[animatable(ty = "FadeLinear", duration = 350)]
#[derive(Childable)]
#[run_after]
pub struct TbaPopup {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for TbaPopup {
    const NAME: &'static str = "RbaPopup";
}

impl ObjectImpl for TbaPopup {
    fn construct(&mut self) {
        self.parent_construct();

        self.width_request(100);
        self.height_request(40);
        self.label.set_halign(Align::Center);
        self.label.set_valign(Align::Center);

        self.test_async_task(
            async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                "This is a popup."
            },
            Some(|p: &mut TbaPopup, text| {
                p.label.set_text(text);
            }),
        );
    }
}

impl WidgetImpl for TbaPopup {
    fn run_after(&mut self) {
        self.parent_run_after();

        println!("CustomPopup run after.")
    }
}

impl PopupImpl for TbaPopup {}

impl TbaPopup {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
