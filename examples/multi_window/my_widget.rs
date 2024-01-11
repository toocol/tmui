use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
    window::{win_builder::WindowBuilder, win_config::WindowConfig}, label::Label,
};

#[extends(Widget)]
pub struct MyWidget {}

impl ObjectSubclass for MyWidget {
    const NAME: &'static str = "MyWidget";
}

impl ObjectImpl for MyWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_vexpand(true);
        self.set_hexpand(true);
    }
}

impl WidgetImpl for MyWidget {
    fn on_mouse_pressed(&mut self, _: &tlib::events::MouseEvent) {
        self.window().create_window(
            WindowBuilder::new()
                .config(WindowConfig::builder().width(300).height(100).build())
                .on_activate(|window| {
                    println!("{} => Child window created.", std::thread::current().name().unwrap());
                    let label = Label::new(Some("Hello World!"));
                    window.child(label);
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
