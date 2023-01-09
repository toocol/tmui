use std::{thread, time::Duration, sync::mpsc::{channel, Sender}};

#[cfg(target_os = "linux")]
use crate::platform::PlatformLinux;
#[cfg(target_os = "macos")]
use crate::platform::PlatformMacos;
#[cfg(target_os = "windows")]
use crate::platform::PlatformWin32;
use crate::{
    backend::{
        opengl_backend::OpenGLBackend,
        raster_backend::RasterBackend, Backend, BackendType,
    },
    graphics::bitmap::Bitmap,
    platform::{PlatformContext, PlatformContextWrapper, PlatformIpc, PlatformType, Message},
};
use skia_safe::{Path, Paint, Color, Font};
use tlib::{
    actions::ActionHub,
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
    Object,
};

#[extends_object]
#[derive(Default)]
pub struct Application {
    platform_type: PlatformType,
    backend_type: BackendType,

    platform_context: Option<Box<dyn PlatformContextWrapper>>,
}

impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";

    type Type = Application;

    type ParentType = Object;
}

impl ObjectImpl for Application {}

impl Application {
    /// Get the builder [`ApplicationBuilder`] of `Application`.
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub fn run(&self) {
        let platform_context = self.platform_context.as_ref().unwrap();
        let (sender, receiver) = channel::<Message>();

        // Create the `UI` main thread.
        let bitmap = platform_context.context_bitmap().clone();
        let backend_type = self.backend_type;
        thread::spawn(move || Self::ui_main(backend_type, bitmap, sender));

        loop {
            if let Ok(msg) = receiver.try_recv() {
                platform_context.send_message(msg);
            }
            platform_context.handle_platform_event();
            thread::sleep(Duration::from_nanos(1));
        }
    }

    fn startup_initialize(&mut self) {
        let title = self.get_property("title").unwrap().get::<String>();
        let width = self.get_property("width").unwrap().get::<i32>();
        let height = self.get_property("height").unwrap().get::<i32>();

        // Create the [`PlatformContext`] based on the platform type specified by the user.
        let platform_context;
        match self.platform_type {
            PlatformType::Ipc => platform_context = PlatformIpc::new(&title, width, height).wrap(),
            #[cfg(target_os = "windows")]
            PlatformType::Win32 => {
                platform_context = PlatformWin32::new(&title, width, height).wrap()
            }
            #[cfg(target_os = "linux")]
            PlatformType::Linux => {
                platform_context = PlatformLinux::new(&title, width, height).wrap()
            }
            #[cfg(target_os = "macos")]
            PlatformType::Macos => {
                platform_context = PlatformMacos::new(&title, width, height).wrap()
            }
        }
        self.platform_context = Some(platform_context);
    }

    fn ui_main(backend_type: BackendType, bitmap: Bitmap, sender: Sender<Message>) {
        let _pixels = bitmap.get_pixels();
        // Create and initialize the `ActionHub`.
        let mut action_hub = ActionHub::new();
        action_hub.initialize();

        // Create the [`Backend`] based on the backend type specified by the user.
        let backend;
        match backend_type {
            BackendType::Raster => backend = RasterBackend::new(bitmap).wrap(),
            BackendType::OpenGL => backend = OpenGLBackend::new(bitmap).wrap(),
        }

        // let mut surface = backend.surface();
        // let canvas = surface.canvas();
        // let mut paint = Paint::default();
        // let font = Font::default();
        // canvas.clear(Color::BLUE);
        // paint.set_color(Color::BLACK);
        // paint.set_anti_alias(true);
        // paint.set_stroke_width(1.0);

        // canvas.scale((1.2, 1.2));
        // let mut path = Path::new();
        // path.move_to((36., 48.));
        // path.quad_to((330., 440.), (600., 180.));
        // canvas.translate((10., 10.));
        // paint.set_stroke_width(10.);
        // paint.set_style(skia_safe::PaintStyle::Stroke);
        // canvas.draw_path(&path, &paint);
        // canvas.draw_str("Hello wrold", (0, 0), &font, &paint);
        // sender.send(Message::MESSAGE_PIXELS_UPDATE).unwrap();

        // Create the `Board`.
        // let _board = Board::new(backend.surface(), backend.width(), backend.height());

        loop {
            action_hub.process_multi_thread_actions();
            thread::sleep(Duration::from_nanos(1));
        }
    }
}

/// The builder to create the [`Application`]
#[derive(Default)]
pub struct ApplicationBuilder {
    platform: Option<PlatformType>,
    backend: Option<BackendType>,
    title: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
}

impl ApplicationBuilder {
    /// Build the [`Application`]
    pub fn build(self) -> Application {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];

        if let Some(ref title) = self.title {
            properties.push(("title", title))
        }
        if let Some(ref width) = self.width {
            properties.push(("width", width))
        }
        if let Some(ref height) = self.height {
            properties.push(("height", height))
        }

        let mut app: Application = Object::new(&properties);
        if let Some(platform) = self.platform {
            app.platform_type = platform
        }
        if let Some(backend) = self.backend {
            app.backend_type = backend
        }
        app.startup_initialize();
        app
    }

    pub fn platform(mut self, platform: PlatformType) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn backend(mut self, backend: BackendType) -> Self {
        self.backend = Some(backend);
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }
}
