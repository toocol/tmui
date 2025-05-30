use tmui::{
    graphics::box_shadow::{BoxShadow, ShadowSide},
    input::{
        number::Number,
        password::Password,
        select::{select_option::SelectOption, Select},
        text::Text,
    },
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Popup)]
#[derive(Childable)]
pub struct InputPopup {
    #[child]
    vbox: Tr<VBox>,
}

impl ObjectSubclass for InputPopup {
    const NAME: &'static str = "InputPopup";
}

impl ObjectImpl for InputPopup {
    fn construct(&mut self) {
        self.parent_construct();

        self.width_request(300);
        self.height_request(300);
        self.set_border_radius(6.);
        // self.set_borders(1., 1., 1., 1.);
        self.set_border_color(Color::GREY_LIGHT);
        self.set_background(Color::GREY_LIGHT);
        self.set_box_shadow(BoxShadow::new(
            6.,
            Color::BLACK,
            None,
            Some(ShadowSide::new(&[ShadowSide::RIGHT, ShadowSide::BOTTOM])),
            None,
            None,
        ));

        self.vbox.set_vexpand(true);
        self.vbox.set_hexpand(true);
        self.vbox.set_homogeneous(true);
        self.vbox.set_content_halign(Align::Center);
        self.vbox.set_content_valign(Align::Center);
        self.vbox.set_spacing(10);

        let options = vec![
            SelectOption::new("Apple".to_string(), false),
            SelectOption::new("Banana".to_string(), false),
            SelectOption::new("Cherry".to_string(), true),
            SelectOption::new("Date".to_string(), false),
            SelectOption::new("Apple".to_string(), false),
            SelectOption::new("Banana".to_string(), false),
            SelectOption::new("Cherry".to_string(), false),
            SelectOption::new("Date".to_string(), false),
            SelectOption::new("Apple".to_string(), false),
            SelectOption::new("Banana".to_string(), false),
            SelectOption::new("Cherry".to_string(), false),
            SelectOption::new("Date".to_string(), false),
            SelectOption::new("Apple".to_string(), false),
            SelectOption::new("Banana".to_string(), false),
            SelectOption::new("Cherry".to_string(), false),
            SelectOption::new("Date".to_string(), false),
            SelectOption::new("Apple".to_string(), false),
            SelectOption::new("Banana".to_string(), false),
            SelectOption::new("Cherry".to_string(), false),
            SelectOption::new("Date".to_string(), false),
            SelectOption::new("Apple".to_string(), false),
            SelectOption::new("Banana".to_string(), false),
            SelectOption::new("Cherry".to_string(), false),
            SelectOption::new("Date".to_string(), false),
        ];
        let mut select = Select::new();
        select.set_options(&options);

        self.vbox.add_child(Text::new());
        self.vbox.add_child(Password::new());
        self.vbox.add_child(Number::new());
        self.vbox.add_child(select);
    }
}

impl WidgetImpl for InputPopup {}

impl InputPopup {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}

impl PopupImpl for InputPopup {
    fn calculate_position(&self, _: Rect, point: Point) -> Point {
        point
    }

    fn is_modal(&self) -> bool {
        true
    }
}
