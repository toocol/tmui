use tlib::object::{ObjectSubclass, ObjectImpl};
use crate::{prelude::*, widget::WidgetImpl};

#[extends(Widget)]
#[derive(Default)]
pub struct Container {}

impl ObjectSubclass for Container {
    const NAME: &'static str = "Container";
}

impl ObjectImpl for Container {}

impl WidgetImpl for Container {}

pub trait ContainerAcquire: ContainerImpl + ContainerImplExt + Default {}

#[reflect_trait]
pub trait ContainerImpl: WidgetImpl {
    /// Go to[`Function defination`](ContainerImpl::children) (Defined in [`ContainerImpl`])
    /// Get all the children ref in `Container`
    fn children(&self) -> Vec<&dyn WidgetImpl>;

    /// Go to[`Function defination`](ContainerImpl::children_mut) (Defined in [`ContainerImpl`])
    /// Get all the mut children ref in `Container`
    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl>;
}

pub trait ContainerImplExt: ContainerImpl {
    /// Go to[`Function defination`](ContainerImplExt::add_child) (Defined in [`ContainerImplExt`])
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>;
}