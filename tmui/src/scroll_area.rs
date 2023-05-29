use crate::{layout::LayoutManager, prelude::*, scroll_bar::ScrollBar};
use derivative::Derivative;
use tlib::{namespace::Orientation, object::ObjectSubclass, prelude::extends};

#[extends(Container)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct ScrollArea {
    #[derivative(Default(value = "Object::new(&[])"))]
    scroll_bar: ScrollBar,
    area: Option<Box<dyn WidgetImpl>>,
}

impl ScrollArea {
    pub fn set_area<T: WidgetImpl>(&mut self, area: T) {
        self.area = Some(Box::new(area));
    }
}

impl ObjectSubclass for ScrollArea {
    const NAME: &'static str = "ScrollArea";
}

impl ObjectImpl for ScrollArea {}

impl WidgetImpl for ScrollArea {}

impl ContainerImpl for ScrollArea {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        vec![&self.scroll_bar, self.area.as_ref().unwrap().as_ref()]
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        vec![&mut self.scroll_bar, self.area.as_mut().unwrap().as_mut()]
    }
}

impl ContainerImplExt for ScrollArea {
    fn add_child<T>(&mut self, _: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        panic!("Please use `set_area()` instead in `ScrollArea`")
    }
}

impl Layout for ScrollArea {
    fn composition(&self) -> Composition {
        match self.scroll_bar.orientation() {
            Orientation::Horizontal => Composition::HorizontalArrange,
            Orientation::Vertical => Composition::VerticalArrange,
        }
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(self, previous, parent, manage_by_container);

        // Deal with the area and scroll bar's position:
        match self.scroll_bar.orientation() {
            Orientation::Horizontal => layout_position_horizontal(self),
            Orientation::Vertical => layout_position_vertical(self),
        }
    }
}

fn layout_position_horizontal(widget: &mut ScrollArea) {

}

fn layout_position_vertical(widget: &mut ScrollArea) {}
