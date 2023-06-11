use tlib::namespace::ImageOption;
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
    let window_size = window.size();
    let mut image = Image::new("tmui/examples/resources/rust.png");
    image.width_request(window_size.width());
    image.height_request(window_size.height());
    image.set_valign(Align::Center);
    image.set_halign(Align::Center);
    image.set_paddings(20, 20, 20, 20);
    image.set_background(Color::CYAN);
    image.set_image_option(ImageOption::Tile);

    window.child(image)
}