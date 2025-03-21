use tmui::{
    graphics::{
        box_shadow::{BoxShadow, ShadowSide},
        render_difference::{CustomRenderChildDiff, ReflectCustomRenderChildDiff},
    },
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Childable)]
pub struct NonStrictClipWidget {}

impl ObjectSubclass for NonStrictClipWidget {
    const NAME: &'static str = "NonStrictClipWidget";
}

impl ObjectImpl for NonStrictClipWidget {
    fn initialize(&mut self) {
        self.width_request(400);
        self.height_request(230);
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);
        self.set_border_radius_sep(10., 0., 0., 0.);
        self.set_box_shadow(BoxShadow::new(
            18.,
            Color::BLACK,
            None,
            Some(ShadowSide::new(&[
                ShadowSide::TOP,
                ShadowSide::RIGHT,
                ShadowSide::LEFT,
            ])),
            Some(3),
            None,
        ));

        self.set_background(Color::CYAN);
        self.set_strict_clip_widget(false);
    }

    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectCustomRenderChildDiff>();
    }
}

impl WidgetImpl for NonStrictClipWidget {
    fn paint(&mut self, painter: &mut Painter) {
        let rect = self.contents_rect(Some(Coordinate::Widget));

        let y = rect.y() + rect.height() / 2;
        painter.draw_line(rect.x() - 10, y, rect.x() + rect.width() + 10, y);
    }
}

impl CustomRenderChildDiff for NonStrictClipWidget {
    fn custom_render_diff(&self, painter: &mut Painter, parent_background: Color) {
        painter.save_pen();
        let rect = self.rect_record();

        let y = rect.y() + rect.height() / 2.;
        painter.set_color(parent_background);
        painter.draw_line_f_global(rect.x() - 10., y, rect.x() + rect.width() + 10., y);
        painter.restore_pen();
    }
}

impl NonStrictClipWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
