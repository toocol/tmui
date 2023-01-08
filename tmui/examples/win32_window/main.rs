use tmui::application::Application;

fn main() {
    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("win32 widnow")
        .build();
    app.run();
}
