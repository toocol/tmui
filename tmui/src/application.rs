#[cfg(target_os = "linux")]
use crate::platform::PlatformLinux;
#[cfg(target_os = "macos")]
use crate::platform::PlatformMacos;
#[cfg(target_os = "windows")]
use crate::platform::PlatformWin32;
use crate::{
    backend::{
        mental_backend::MentalBackend, opengl_backend::OpenGLBackend,
        raster_backend::RasterBackend, Backend, BackendType, BackendWrapper,
    },
    platform::{PlatformContext, PlatformContextWrapper, PlatformIpc, PlatformType},
};
use tlib::{
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
    backend: Option<Box<dyn BackendWrapper>>,
}

impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";

    type Type = Application;

    type ParentType = Object;
}

impl ObjectImpl for Application {}

impl Application {
    pub fn run(&mut self) {
        loop {}
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

        // Create the [`Backend`] based on the backend type specified by the user.
        let backend;
        match self.backend_type {
            BackendType::Raster => {
                backend = RasterBackend::new().wrap()
            }
            BackendType::OpenGL => {
                backend = OpenGLBackend::new().wrap()
            }
            BackendType::Mental => {
                backend = MentalBackend::new().wrap()
            }
        }
        self.backend = Some(backend);
    }
}

/// The builder to create the [`Application`]
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
