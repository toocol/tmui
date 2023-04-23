use tlib::object::{ObjectImpl, ObjectSubclass};
use tmui::container::{ContainerImpl, ContainerImplExt};
use tmui::prelude::*;
use tmui::widget::{Widget, WidgetImpl};

#[extends_container]
#[derive(Default)]
pub struct TestContainer {}

impl ObjectSubclass for TestContainer {
    const NAME: &'static str = "TestContainer";

    type Type = TestContainer;

    type ParentType = Container;
}

impl ObjectImpl for TestContainer {}

impl WidgetImpl for TestContainer {}

impl ContainerImpl for TestContainer {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for TestContainer {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.children.push(Box::new(child))
    }
}

#[test]
fn main() {
    let mut r = TypeRegistry::new();
    r.initialize();
    let c = Object::new::<TestContainer>(&[]);
    c.inner_type_register(r.as_mut());
    cast_test(&c)
}

fn cast_test(widget: &dyn WidgetImpl) {
    let mut c = false;
    if let Some(reflect) = TypeRegistry::get_type_data::<ReflectContainerImpl>(widget.as_reflect())
    {
        let _ = (reflect.get_func)(widget.as_reflect());
        c = true;
    }
    assert!(c)
    // let container = cast!(widget as ContainerImpl);
}
