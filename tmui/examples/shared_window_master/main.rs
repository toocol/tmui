use tmui::{application::Application, application_window::ApplicationWindow};

pub const IPC_NAME: &'static str = "shared_mem";

fn main() {
    log4rs::init_file("tmui/examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::<(), ()>::shared_builder(IPC_NAME)
        .width(1280)
        .height(800)
        .title("Shared Window")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(_window: &mut ApplicationWindow) {}
