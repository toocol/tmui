use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[animatable(ty = "Linear", direction = "LeftToRight", duration = 300)]
pub struct AnimatedWidget {}

impl ObjectSubclass for AnimatedWidget {
    const NAME: &'static str = "AnimatedWidget";
}

impl ObjectImpl for AnimatedWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_animation(Animation::EaseIn);
    }
}

impl WidgetImpl for AnimatedWidget {}

impl AnimatedWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
