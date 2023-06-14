mod color_convert;

use color_convert::ColorConvert;
use tlib::Object;
use tmui::{
    application::Application,
    application_window::ApplicationWindow, widget::WidgetImplExt,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("win32 widnow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let color_convert: ColorConvert = Object::new(&[]);
    window.child(color_convert)
}
