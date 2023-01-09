use tlib::{
    object::{IsSubclassable, ObjectImpl, ObjectSubclass},
    prelude::*,
};

#[extends_object]
#[derive(Default)]
pub struct Widget {}

impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";

    type Type = Widget;
    type ParentType = Object;
}

impl ObjectImpl for Widget {
    fn construct(&self) {
        self.parent_construct();
        println!("`Widget` construct")
    }
}

impl IsSubclassable for Widget {}

pub trait WidgetExt: ObjectOperation + ObjectImpl {}

impl WidgetExt for Widget {}

pub trait WidgetImpl: WidgetExt {}
