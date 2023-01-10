use tmui::{application::Application, application_window::ApplicationWindow};

fn main() {
    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("win32 widnow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(_window: &mut ApplicationWindow) {
    println!("Hello World");
}
