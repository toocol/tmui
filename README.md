# tmui
**_Simple ui kit base on Skia._**  

## Example
```rust
use tmui::prelude::*;
use tmui::{application::Application, application_window::ApplicationWindow, label::Label};

fn main() {
    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("win32 widnow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let label = Label::new(Some("Hello World!"));
    window.child(label);
}
```
