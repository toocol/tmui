use tlib::object::{ObjectImpl, ObjectSubclass};
use tmui::container::{ContainerImpl, ContainerImplExt};
use tmui::prelude::*;
use tmui::widget::{Widget, WidgetImpl};

#[extends(Container)]
#[derive(Default)]
pub struct TestContainer {}

impl ObjectSubclass for TestContainer {
    const NAME: &'static str = "TestContainer";
}

impl ObjectImpl for TestContainer {}

impl WidgetImpl for TestContainer {}

impl ContainerImpl for TestContainer {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.container.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.container.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for TestContainer {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.container.children.push(Box::new(child))
    }
}

impl Layout for TestContainer {
    fn composition(&self) -> tmui::layout::Composition {
        tmui::layout::Composition::HorizontalArrange
    }

    fn position_layout(&mut self, _: &dyn WidgetImpl, _: &dyn WidgetImpl, _: bool) {}
}

#[test]
fn main() {
    let r = TypeRegistry::instance();
    let mut c = Object::new::<TestContainer>(&[]);
    c.inner_type_register(r);
    cast_test(&c);
    cast_mut_test(&mut c);
    cast_boxed_test(Box::new(c));
}

fn cast_test(widget: &dyn WidgetImpl) {
    let mut c = false;
    if let Some(_) = cast!(widget as ContainerImpl) {
        c = true;
    }
    assert!(c);
}

fn cast_mut_test(widget: &mut dyn WidgetImpl) {
    let mut c = false;
    if let Some(_) = cast_mut!(widget as ContainerImpl) {
        c = true;
    }
    assert!(c);
}

fn cast_boxed_test(widget: Box<dyn WidgetImpl>) {
    let mut c = false;
    if let Some(_) = cast_boxed!(widget as ContainerImpl) {
        c = true;
    }
    assert!(c);
}