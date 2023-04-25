use tlib::object::{ObjectImpl, ObjectSubclass};
use crate::{
    container::{ContainerImpl, ContainerImplExt},
    prelude::*,
};

#[extends(Container)]
#[derive(Default)]
pub struct Stack {}

impl ObjectSubclass for Stack {
    const NAME: &'static str = "Stack";

    type Type = Stack;

    type ParentType = Container;
}

impl ObjectImpl for Stack {}

impl WidgetImpl for Stack {}

impl ContainerImpl for Stack {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for Stack {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.children.push(Box::new(child))
    }
}

impl Layout for Stack {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Overlay
    }

    fn position_layout(&mut self, previous: &dyn WidgetImpl, _: &dyn WidgetImpl) {
        let widget_rect = self.rect();
        let previous_rect = previous.rect();

        let halign = self.get_property("halign").unwrap().get::<Align>();
        let valign = self.get_property("valign").unwrap().get::<Align>();

        match halign {
            Align::Start => self.set_fixed_x(previous_rect.x() as i32 + self.margin_left()),
            Align::Center => {
                let offset = (previous_rect.width() - self.rect().width()) as i32 / 2
                    + self.margin_left();
                self.set_fixed_x(previous_rect.x() as i32 + offset)
            }
            Align::End => {
                let offset = previous_rect.width() as i32 - self.rect().width() as i32
                    + self.margin_left();
                self.set_fixed_x(previous_rect.x() as i32 + offset)
            }
        }

        match valign {
            Align::Start => self.set_fixed_y(
                previous_rect.y() as i32 + widget_rect.y() as i32 + self.margin_top(),
            ),
            Align::Center => {
                let offset = (previous_rect.height() - self.rect().height()) as i32 / 2
                    + self.margin_top();
                self.set_fixed_y(previous_rect.y() as i32 + offset)
            }
            Align::End => {
                let offset = previous_rect.height() as i32 - self.rect().height() as i32
                    + self.margin_top();
                self.set_fixed_y(previous_rect.y() as i32 + offset)
            }
        }
    }
}

impl Stack {
    pub fn new() -> Self {
        Object::new(&[])
    }
}