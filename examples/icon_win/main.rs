use tmui::{application::Application, application_window::ApplicationWindow, graphics::icon::Icon};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Icon Window")
        .icon(Icon::from_file("examples/icon_win/app.png").unwrap())
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(_: &mut ApplicationWindow) {}
