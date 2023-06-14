use tmui::{
    prelude::*,
    skia_safe::{self, Path},
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct SkiaPaint {}

impl ObjectSubclass for SkiaPaint {
    const NAME: &'static str = "SkiaPaint";
}

impl ObjectImpl for SkiaPaint {}

impl WidgetImpl for SkiaPaint {
    fn paint(&mut self, mut painter: tmui::graphics::painter::Painter) {
        let rect = self.contents_rect(Some(Coordinate::Widget));
        painter.set_color(Color::RED);
        painter.set_antialiasing();

        let half_width = rect.width() as f32 / 2.0;
        let half_height = rect.height() as f32 / 2.0;
        let radius = half_width.min(half_height) * 0.8;

        let mut path = painter.path();

        path.add_circle(
            skia_safe::Point::new(half_width - radius * 0.35, half_height - radius * 0.35),
            radius * 0.7,
            None,
        );

        let mut right_heart = Path::new();
        right_heart.move_to(skia_safe::Point::new(
            half_width + radius * 0.35,
            half_height - radius * 0.35,
        ));
        right_heart.cubic_to(
            skia_safe::Point::new(half_width + radius * 0.5, half_height - radius * 0.7),
            skia_safe::Point::new(half_width + radius * 0.7, half_height - radius * 0.5),
            skia_safe::Point::new(half_width + radius * 0.35, half_height + radius * 0.35),
        );
        right_heart.close();
        path.add_path(&right_heart, skia_safe::Point::new(1., 1.), None);

        let mut left_heart = Path::new();
        left_heart.move_to(skia_safe::Point::new(
            half_width - radius * 0.35,
            half_height - radius * 0.35,
        ));
        left_heart.cubic_to(
            skia_safe::Point::new(half_width - radius * 0.5, half_height - radius * 0.7),
            skia_safe::Point::new(half_width - radius * 0.7, half_height - radius * 0.5),
            skia_safe::Point::new(half_width - radius * 0.35, half_height + radius * 0.35),
        );
        left_heart.close();
        path.add_path(&left_heart, skia_safe::Point::new(1., 1.), None);

        painter.draw_path(&mut path);

        painter.set_color((0, 0, 0, 50).into());
        painter.set_style(skia_safe::paint::Style::Stroke);
        painter.set_line_width(radius * 0.05);
        painter.set_stroke_cap(skia_safe::PaintCap::Round);

        let mut shadow_path = painter.path();
        shadow_path.add_circle(
            skia_safe::Point::new(half_width, half_height),
            radius * 0.7,
            None,
        );
        shadow_path.add_path(&right_heart, skia_safe::Point::new(1., 1.), None);
        shadow_path.add_path(&left_heart, skia_safe::Point::new(1., 1.), None);
        shadow_path.close();

        painter.draw_path(&mut shadow_path);
    }
}
