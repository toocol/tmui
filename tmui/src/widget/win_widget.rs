use super::{WidgetExt, WidgetImpl, WidgetSignals};
use crate::{
    prelude::ApplicationWindow,
    window::{win_builder::WindowBuilder, win_config::WindowConfig},
};
use std::ptr::NonNull;
use tlib::{prelude::*, reflect_trait, winit::window::WindowLevel};

pub(crate) type WinWidgetHnd = Option<NonNull<dyn WinWidget>>;
pub(crate) type CrsWinMsgHnd = Option<NonNull<dyn CrossWinMsgHandlerInner>>;

pub trait CrsWinMsgRequire: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> CrsWinMsgRequire for T {}

#[reflect_trait]
pub trait WinWidget: WidgetImpl + WidgetSignals {
    fn child_process_fn(&mut self) -> Box<dyn FnOnce(&mut ApplicationWindow) + Send>;
}

#[reflect_trait]
pub trait CrossWinWidget {}

#[reflect_trait]
pub trait CrossWinMsgHandlerInner: WidgetImpl {
    fn handle_inner(&mut self);
}

pub trait CrossWinMsgHandlerRequire: CrossWinMsgHandler {}

pub trait CrossWinMsgHandler {
    type T: CrsWinMsgRequire;

    fn handle(&mut self, msg: Self::T);
}

pub trait CrossWinMsgSender {
    type T: CrsWinMsgRequire;

    fn send_cross_win_msg(&self, msg: Self::T);
}

pub(crate) fn handle_win_widget_create(win_widget: &mut dyn WinWidget, inner: bool) {
    let mut rect = win_widget.borderless_rect();
    if rect.width() == 0 {
        rect.set_width(100)
    }
    if rect.height() == 0 {
        rect.set_height(100)
    }

    let window = ApplicationWindow::window();
    let pos = if inner {
        rect.top_left()
    } else {
        window.map_to_client(&rect.top_left())
    };

    let child_proc_fn = win_widget.child_process_fn();
    window.create_window(
        WindowBuilder::new()
            .config(
                WindowConfig::builder()
                    .position(pos)
                    .width(rect.width() as u32)
                    .height(rect.height() as u32)
                    .decoration(false)
                    .skip_taskbar(true)
                    .transparent(true)
                    .visible(win_widget.visible())
                    .win_level(WindowLevel::AlwaysOnTop)
                    .build(),
            )
            .inner_window(inner)
            .win_widget_id(win_widget.id())
            .on_activate(move |win| {
                win.set_transparency(0);
                child_proc_fn(win);
            }),
    )
}
