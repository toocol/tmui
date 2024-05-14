use tmui::{
    input::text::Text, prelude::*, tlib::object::{ObjectImpl, ObjectSubclass}, widget::WidgetImpl, window::{win_builder::WindowBuilder, win_config::WindowConfig}
};

use crate::input_dialog::InputDialog;

#[extends(Widget)]
#[derive(Childable)]
pub struct MyWidget {
    #[child]
    text: Box<Text>
}

impl ObjectSubclass for MyWidget {
    const NAME: &'static str = "MyWidget";
}

impl ObjectImpl for MyWidget {
    fn initialize(&mut self) {
        self.set_vexpand(true);
        self.set_hexpand(true);

        self.text.set_margins(10, 0, 0, 10);
    }
}

impl WidgetImpl for MyWidget {
    fn on_mouse_pressed(&mut self, _: &tlib::events::MouseEvent) {
        self.window().create_window(
            WindowBuilder::new()
                .config(WindowConfig::builder().width(300).height(100).build())
                .modal(true)
                .on_activate(|window| {
                    println!(
                        "{} => Child window created.",
                        std::thread::current().name().unwrap(),
                    );
                    window.child(InputDialog::new());
                }),
        )
    }
}

impl MyWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
