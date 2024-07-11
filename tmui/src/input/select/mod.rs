pub mod dropdown_list;
pub mod select_option;

use dropdown_list::DropdownList;
use select_option::SelectOption;
use super::{Input, InputBounds, InputSignals, InputWrapper};
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

const DEFAULT_BORDER_COLOR: Color = Color::grey_with(96);

pub trait SelectBounds: InputBounds + ToString {}
impl<T: InputBounds + ToString> SelectBounds for T {}

#[extends(Widget)]
#[popupable]
pub struct Select<T: SelectBounds> {
    input_wrapper: InputWrapper<T>,
}

impl<T: SelectBounds> ObjectSubclass for Select<T> {
    const NAME: &'static str = "Select";
}

impl<T: SelectBounds> ObjectImpl for Select<T> {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_border_radius(2.);
        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(DEFAULT_BORDER_COLOR);

        self.input_wrapper.init(self.id());

        let dropdown_list = DropdownList::new();
        self.add_popup(dropdown_list);
    }
}

impl<T: SelectBounds> WidgetImpl for Select<T> {
    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }
}

impl<T: SelectBounds> InputSignals for Select<T> {}

impl<T: SelectBounds> Input for Select<T> {
    type Value = T;

    #[inline]
    fn input_type(&self) -> super::InputType {
        super::InputType::Select
    }

    #[inline]
    fn input_wrapper(&self) -> &super::InputWrapper<Self::Value> {
        &self.input_wrapper
    }

    #[inline]
    fn required_handle(&mut self) -> bool {
        true
    }
}

impl<T: SelectBounds> Select<T> {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn set_options(&mut self, options: &[SelectOption<T>]) {
        if options.is_empty() {
            return
        }
        self.dropdown_list().clear_options();

        let default_val = options.first().unwrap().value();
        let mut max_len = default_val.to_string().chars().count();
        let mut idx = 0;
        self.set_value(default_val);

        for (i, option) in options.iter().enumerate() {
            self.dropdown_list().add_option(option);
            
            if option.is_selected() {
                let value = option.value();
                max_len = max_len.max(value.to_string().chars().count());
                idx = i;

                self.set_value(value);
            }
        }

        self.dropdown_list().scroll_to(idx);

        self.update();
    }
}

impl<T: SelectBounds> Select<T> {
    #[inline]
    fn dropdown_list(&mut self) -> &mut DropdownList {
        self.get_popup_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<DropdownList>()
            .unwrap()
    }
}
