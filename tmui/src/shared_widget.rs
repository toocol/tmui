use lazy_static::lazy_static;
use std::sync::atomic::Ordering;
use tlib::connect;

use crate::{
    application::{self, PLATFORM_CONTEXT},
    platform::PlatformType,
    prelude::*,
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        run_after,
    },
    widget::WidgetImpl,
};

lazy_static! {
    static ref SUPPORTED_CHARACTER: Vec<u8> = {
        let mut chs = vec![];
        for c in b'0'..b'9' {
            chs.push(c)
        }
        for c in b'a'..b'z' {
            chs.push(c)
        }
        for c in b'A'..b'z' {
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

        connect!(self, geometry_changed(), self, on_geometry_changed(Rect));
    }
}

impl WidgetImpl for SharedWidget {
    fn run_after(&mut self) {
        if ApplicationWindow::window_of(self.window_id()).platform_type() == PlatformType::Ipc {
            panic!("`SharedWidget` can not be used on `PlatformType::Ipc`")
        }
        self.parent_run_after();

        let platform_context = unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_mut().unwrap() };
        platform_context.add_shared_region(self.shared_id(), self.rect());
    }
}

pub trait SharedWidgetExt {
    fn shared_id(&self) -> &'static str;

    fn set_shared_id(&mut self, id: &'static str);
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
}

impl SharedWidget {
    #[inline]
    fn on_geometry_changed(&self, _: Rect) {
        let platform_context = unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_mut().unwrap() };
        platform_context.add_shared_region(self.shared_id(), self.rect());
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
