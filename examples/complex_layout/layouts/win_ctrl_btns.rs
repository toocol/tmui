use crate::asset::Asset;
use tmui::{
    icons::{svg_icon::SvgIcon, svg_toggle_icon::SvgToggleIcon},
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{callbacks::CallbacksRegister, WidgetImpl},
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct WinCtrlBtns {
    #[derivative(Default(value = "{
        let file = Asset::get(\"minimize.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    minimize: Tr<SvgIcon>,

    #[derivative(Default(value = "{
        let maximize = Asset::get(\"large.svg\").unwrap();
        let restore = Asset::get(\"restore.svg\").unwrap();
        SvgToggleIcon::from_bytes(&[maximize.data.as_ref(), restore.data.as_ref()])
    }"))]
    #[children]
    large_restore: Tr<SvgToggleIcon>,

    #[derivative(Default(value = "{
        let file = Asset::get(\"close.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    close: Tr<SvgIcon>,
}

impl ObjectSubclass for WinCtrlBtns {
    const NAME: &'static str = "WinCtrlBtns";
}

impl ObjectImpl for WinCtrlBtns {
    fn initialize(&mut self) {
        self.set_halign(Align::End);
        self.set_vexpand(true);
        self.width_request(138);

        let background = self.background();
        const CTRL_BTN_GREY: Color = Color::grey_with(225);
        const CTRL_BTN_RED: Color = Color::rgb(245, 40, 40);

        self.minimize.width_request(46);
        self.minimize.height_request(30);
        self.minimize
            .register_mouse_enter(|w| w.set_background(CTRL_BTN_GREY));
        self.minimize
            .register_mouse_leave(move |w| w.set_background(background));
        self.minimize
            .register_mouse_released(|w, _| w.window().minimize());

        self.large_restore.width_request(46);
        self.large_restore.height_request(30);
        self.large_restore
            .register_mouse_enter(|w| w.set_background(CTRL_BTN_GREY));
        self.large_restore
            .register_mouse_leave(move |w| w.set_background(background));
        self.large_restore.register_mouse_released(|w, _| {
            let icon = w.downcast_mut::<SvgToggleIcon>().unwrap();
            match icon.current_icon() {
                0 => icon.window().maximize(),
                1 => icon.window().restore(),
                _ => unreachable!(),
            }
        });
        self.large_restore.register_window_maximized(|w| {
            w.downcast_mut::<SvgToggleIcon>()
                .unwrap()
                .set_current_icon(1)
        });
        self.large_restore.register_window_restored(|w| {
            w.downcast_mut::<SvgToggleIcon>()
                .unwrap()
                .set_current_icon(0)
        });

        self.close.width_request(46);
        self.close.height_request(30);
        self.close
            .register_mouse_enter(|w| w.set_background(CTRL_BTN_RED));
        self.close
            .register_mouse_leave(move |w| w.set_background(background));
        self.close
            .register_mouse_released(|w, _| w.window().close());
    }
}

impl WidgetImpl for WinCtrlBtns {}
