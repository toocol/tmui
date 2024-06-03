use lazy_static::lazy_static;
use tlib::{connect, skia_safe::ImageInfo};

use crate::{
    application,
    backend::create_image_info,
    platform::PlatformType,
    prelude::*,
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        run_after,
    },
    widget::WidgetImpl, opti::tracker::Tracker,
};

lazy_static! {
    static ref SUPPORTED_CHARACTER: Vec<u8> = {
        let mut chs = vec![];
        for c in b'0'..=b'9' {
            chs.push(c)
        }
        for c in b'a'..=b'z' {
            chs.push(c)
        }
        for c in b'A'..=b'Z' {
            chs.push(c)
        }
        for c in b"!@#$%^&*()-_+=/\\" {
            chs.push(*c)
        }
        chs
    };
}

#[extends(Widget)]
#[run_after]
pub struct SharedWidget {
    shared_id: Option<&'static str>,
    image_info: ImageInfo,
    run_aftered: bool,
}

impl ObjectSubclass for SharedWidget {
    const NAME: &'static str = "SharedWidget";
}

impl ObjectImpl for SharedWidget {
    fn construct(&mut self) {
        if !application::is_shared() {
            panic!("`SharedWidget` can only used in shared memory application.");
        }

        self.parent_construct();

        connect!(self, geometry_changed(), self, on_geometry_changed(FRect));
    }
}

impl WidgetImpl for SharedWidget {
    fn run_after(&mut self) {
        if ApplicationWindow::window_of(self.window_id()).platform_type() == PlatformType::Ipc {
            panic!("`SharedWidget` can not be used on `PlatformType::Ipc`")
        }
        let bridge = self.window().ipc_bridge().unwrap();

        let size = self.size();
        bridge.create_buffer(size.width() as u32, size.height() as u32);

        bridge.add_shared_region(self.shared_id(), self.rect());

        self.image_info = create_image_info((size.width(), size.height()));
        self.run_aftered = true;
    }
}

pub trait SharedWidgetExt {
    fn shared_id(&self) -> &'static str;

    fn set_shared_id(&mut self, id: &'static str);

    fn image_info(&self) -> &ImageInfo;

    fn is_shared_invalidate(&self) -> bool;

    fn shared_validate(&self);

    fn pixels_render(&mut self, painter: &mut Painter);
}

impl SharedWidgetExt for SharedWidget {
    #[inline]
    fn shared_id(&self) -> &'static str {
        self.shared_id
            .expect("`SharedWidget` should set the `shared_id`")
    }

    #[inline]
    fn set_shared_id(&mut self, id: &'static str) {
        self.check_shared_id(id);
        self.shared_id = Some(id)
    }

    #[inline]
    fn image_info(&self) -> &ImageInfo {
        &self.image_info
    }

    #[inline]
    fn is_shared_invalidate(&self) -> bool {
        ApplicationWindow::window_of(self.window_id())
            .ipc_bridge()
            .unwrap()
            .is_invalidate()
    }

    #[inline]
    fn shared_validate(&self) {
        ApplicationWindow::window_of(self.window_id())
            .ipc_bridge()
            .unwrap()
            .set_invalidate(false)
    }

    fn pixels_render(&mut self, painter: &mut Painter) {
        let bridge = ApplicationWindow::window_of(self.window_id())
            .ipc_bridge()
            .unwrap();

        let tracker = Tracker::start("shared_widget_wait_prepared");
        bridge.wait_prepared();
        drop(tracker);

        let tracker = Tracker::start("shared_widget_locked_read_buffer");
        let (buffer, _guard) = bridge.buffer();
        drop(tracker);

        let size = self.size();
        let row_bytes = size.width() as usize * 4;

        let _tracker = Tracker::start("shared_widget_draw_pixels");
        painter.draw_pixels(self.image_info(), buffer, row_bytes, (0, 0));
    }
}

impl SharedWidget {
    #[inline]
    fn on_geometry_changed(&mut self, _: FRect) {
        self.window()
            .ipc_bridge()
            .unwrap()
            .add_shared_region(self.shared_id(), self.rect());

        if self.run_aftered {
            let size = self.size();

            let bridge = self.window().ipc_bridge().unwrap();

            bridge.pretreat_resize(size.width(), size.height());

            self.image_info = create_image_info((size.width(), size.height()));
            self.window().set_shared_widget_size_changed(true);
        }
    }

    #[inline]
    fn check_shared_id(&self, shared_id: &str) {
        if shared_id.len() > 18 {
            panic!("The maximum length of `shared_id` of SharedWidget is 18.")
        }
        for b in shared_id.as_bytes() {
            if !SUPPORTED_CHARACTER.contains(b) {
                panic!("Unsupported character {}, `shared_id` only support character: [0-9][a-z][A-Z][!@#$%^&*()-_+=/\\]", *b as char)
            }
        }
    }
}

#[reflect_trait]
pub trait SharedWidgetImpl: WidgetImpl + SharedWidgetExt {}
impl SharedWidgetImpl for SharedWidget {}
