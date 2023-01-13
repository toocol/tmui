use tmui::{application::Application, application_window::ApplicationWindow};

fn main() {
    log4rs::init_file("tmui/examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("win32 widnow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(_window: &ApplicationWindow) {
    println!("Hello World");
}
