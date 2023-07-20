use tlib::object::{ObjectImpl, ObjectSubclass};
use tmui::{
    container::{ContainerImpl, ContainerImplExt, ContainerScaleCalculate, SCALE_ADAPTION},
    label::Label,
    prelude::*,
    widget::{Widget, WidgetImpl},
};

#[extends(Container)]
#[derive(Childrenable)]
pub struct TestContainer {
    #[derivative(Default(value = "99"))]
    _num: i32,
    #[children]
    _label: Box<Label>,
}

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
        self.container
            .children
            .iter_mut()
            .map(|c| c.as_mut())
            .collect()
    }
}

impl ContainerImplExt for TestContainer {
    fn add_child<T>(&mut self, child: Box<T>)
    where
        T: WidgetImpl,
    {
        self.container.children.push(child)
    }
}

impl Layout for TestContainer {
    fn composition(&self) -> tmui::layout::Composition {
        tmui::layout::Composition::HorizontalArrange
    }

    fn position_layout(&mut self, _: Option<&dyn WidgetImpl>, _: Option<&dyn WidgetImpl>, _: bool) {
    }
}

impl ContainerScaleCalculate for TestContainer {
    fn container_hscale_calculate(&self) -> f32 {
        SCALE_ADAPTION
    }

    fn container_vscale_calculate(&self) -> f32 {
        SCALE_ADAPTION
    }
}

#[test]
fn main() {
    let r = TypeRegistry::instance();
    let mut c = Object::new::<TestContainer>(&[]);
    c.inner_type_register(r);
    cast_test(c.as_ref());
    cast_mut_test(c.as_mut());
    cast_boxed_test(c);
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
