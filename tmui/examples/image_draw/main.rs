use tmui::{
   prelude::*,
   application::Application,
   application_window::ApplicationWindow, image::Image,
};

fn main() {
   log4rs::init_file("tmui/examples/log4rs.yaml", Default::default()).unwrap();

   let app = Application::builder()
       .width(1280)
       .height(800)
       .title("Image draw")
       .build();

   app.connect_activate(build_ui);

   app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut image = Image::new("tmui/examples/resources/rust.png");
    image.width_request(300);
    image.height_request(300);
    image.set_valign(Align::Center);
    image.set_halign(Align::Center);
    image.set_paddings(20, 20, 20, 20);
    image.set_background(Color::CYAN);

    window.child(image)
}