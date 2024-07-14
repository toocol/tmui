use tlib::{connect, run_after};
use tmui::{
    input::{
        password::Password, select::{select_option::SelectOption, Select}, text::{Text, TextExt, TextSignals}, Input, InputSignals
    },
    prelude::*,
    tlib::{
        namespace::MouseButton,
        object::{ObjectImpl, ObjectSubclass},
    },
    widget::{callbacks::CallbacksRegister, WidgetImpl},
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[run_after]
pub struct Holder {
    #[children]
    text1: Box<Text>,

    #[children]
    text2: Box<Text>,

    #[children]
    text3: Box<Text>,

    #[children]
    password: Box<Password>,

    #[children]
    select1: Box<Select<String>>,

    #[children]
    select2: Box<Select<String>>,
}

impl ObjectSubclass for Holder {
    const NAME: &'static str = "Holder";
}

impl ObjectImpl for Holder {
    fn initialize(&mut self) {
        // self.text1.set_background(Color::RED);
        self.text1.width_request(400);
        self.text1.height_request(25);
        self.text1.set_margin_left(20);
        self.text1.set_margin_top(10);
        self.text1.set_hexpand(true);
        self.text1.set_value("Contents of text-1.中文".to_string());
        // self.text1.set_vexpand(true);
        connect!(self.text1, value_changed(), self, text_value_changed());
        connect!(
            self.text1,
            selection_changed(),
            self,
            text_selection_changed()
        );

        // self.text2.set_background(Color::BLUE);
        // self.text2.width_request(200);
        // self.text2.height_request(25);
        self.text2.set_margin_left(20);
        self.text2.set_margin_top(10);
        self.text2
            .set_value("Contents of disabled text-2, some text may not be seen.".to_string());
        self.text2.disable();
        // self.text2.set_vexpand(true);

        // self.text3.width_request(200);
        // self.text3.height_request(25);
        self.text3.set_margin_left(20);
        self.text3.set_margin_top(10);
        self.text3
            .set_placeholder("Placeholder of text-3/中文提示符");
        // self.text3.set_vexpand(true);

        self.password.set_margin_left(20);
        self.password.set_margin_top(10);
        self.password.register_mouse_released(|w, evt| {
            if evt.mouse_button() == MouseButton::RightButton {
                let pwd = w.downcast_mut::<Password>().unwrap();
                let v = !pwd.is_password_visible();
                pwd.set_password_visible(v);
            }
        });
        self.password.set_required(true);
        self.password.check_required();
        self.password.set_require_invalid_border_color(Color::grey_with(210));
        self.password
            .set_customize_require_invalid_render(move |painter, mut rect| {
                rect.set_width(rect.width() - 1.);
                rect.set_height(rect.height() - 1.);

                painter.set_color(Color::from_hex("#ff6b6b"));
                painter.draw_rect_global(rect);
            });

        let options = vec![
            SelectOption::new("Apple".to_string(), false),
            SelectOption::new("Banana".to_string(), false),
            SelectOption::new("Cherry".to_string(), true),
            SelectOption::new("Date".to_string(), false),
        ];
        self.select1.set_options(&options);
        self.select1.set_margin_left(20);

        self.select2.set_options(&options);
        self.select2.set_margin_left(20);
        self.select2.set_margin_bottom(20);
        self.select2.set_valign(Align::End);

        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_spacing(30);
        self.set_homogeneous(false);
    }
}

impl WidgetImpl for Holder {
    fn run_after(&mut self) {
        self.text1.set_focus(true);
    }
}

impl Holder {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn text_value_changed(&self) {}

    #[inline]
    pub fn text_selection_changed(&self) {}
}
