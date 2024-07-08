use tmui::{
    icons::{svg_icon::SvgIcon, svg_toggle_icon::SvgToggleIcon},
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{callbacks::CallbacksRegister, WidgetImpl},
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct CtrlButtons {
    #[children]
    #[derivative(Default(value = "SvgIcon::from_file(\"examples/resources/minimize.svg\")"))]
    minimize: Box<SvgIcon>,

    #[children]
    #[derivative(Default(
        value = "SvgToggleIcon::from_files(&[\"examples/resources/restore.svg\",\"examples/resources/large.svg\"])"
    ))]
    maximize_restore: Box<SvgToggleIcon>,

    #[children]
    #[derivative(Default(value = "SvgIcon::from_file(\"examples/resources/close.svg\")"))]
    close: Box<SvgIcon>,
}

impl ObjectSubclass for CtrlButtons {
    const NAME: &'static str = "CtrlButtons";
}

impl ObjectImpl for CtrlButtons {
    fn initialize(&mut self) {
        self.minimize.set_background(Color::GREY_LIGHT);
        self.minimize.width_request(30);
        self.minimize.height_request(20);
        self.minimize
            .register_hover_in(|w| w.set_background(Color::grey_with(223)));
        self.minimize
            .register_hover_out(|w| w.set_background(Color::GREY_LIGHT));

        self.maximize_restore.set_background(Color::GREY_LIGHT);
        self.maximize_restore.width_request(30);
        self.maximize_restore.height_request(20);
        self.maximize_restore
            .register_hover_in(|w| w.set_background(Color::grey_with(223)));
        self.maximize_restore
            .register_hover_out(|w| w.set_background(Color::GREY_LIGHT));
        self.maximize_restore
            .register_mouse_released(|w, _| w.downcast_mut::<SvgToggleIcon>().unwrap().toggle());

        self.close.set_background(Color::GREY_LIGHT);
        self.close.width_request(30);
        self.close.height_request(20);
        self.close
            .register_hover_in(|w| w.set_background(Color::RED));
        self.close
            .register_hover_out(|w| w.set_background(Color::GREY_LIGHT));
    }
}

impl WidgetImpl for CtrlButtons {}
