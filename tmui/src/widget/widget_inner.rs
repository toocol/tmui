use std::collections::HashSet;

use tlib::object::ObjectId;

use super::{EventBubble, WidgetImpl};

pub(crate) trait WidgetInnerExt {
    fn set_fixed_width_ration(&mut self, ration: f32);

    fn set_fixed_height_ration(&mut self, ration: f32);

    fn cancel_fixed_width(&mut self);

    fn cancel_fixed_height(&mut self);

    fn event_bubble(&self) -> EventBubble;

    fn set_event_bubble(&mut self, event_bubble: EventBubble);

    fn detecting_width(&self) -> i32;

    fn detecting_height(&self) -> i32;

    fn set_detecting_width(&mut self, detecting_width: i32);

    fn set_detecting_height(&mut self, detecting_height: i32);

    fn set_resize_redraw(&mut self, is: bool);

    fn is_manage_by_container(&self) -> bool;

    fn set_manage_by_container(&mut self, manage_by_container: bool);

    fn children_index(&self) -> &HashSet<ObjectId>;

    fn children_index_mut(&mut self) -> &mut HashSet<ObjectId>;
}

macro_rules! widget_inner_ext_impl {
    () => {
        #[inline]
        fn set_fixed_width_ration(&mut self, ration: f32) {
            self.widget_props_mut().fixed_width_ration = ration;
        }

        #[inline]
        fn set_fixed_height_ration(&mut self, ration: f32) {
            self.widget_props_mut().fixed_height_ration = ration;
        }

        #[inline]
        fn cancel_fixed_width(&mut self) {
            self.set_fixed_width(0);
            self.widget_props_mut().fixed_width = false;
            self.widget_props_mut().width_request = 0;
        }

        #[inline]
        fn cancel_fixed_height(&mut self) {
            self.set_fixed_height(0);
            self.widget_props_mut().fixed_height = false;
            self.widget_props_mut().height_request = 0;
        }

        #[inline]
        fn event_bubble(&self) -> EventBubble {
            self.widget_props().event_bubble
        }

        #[inline]
        fn set_event_bubble(&mut self, event_bubble: EventBubble) {
            self.widget_props_mut().event_bubble = event_bubble;
        }

        #[inline]
        fn detecting_width(&self) -> i32 {
            self.widget_props().detecting_width
        }

        #[inline]
        fn detecting_height(&self) -> i32 {
            self.widget_props().detecting_height
        }

        #[inline]
        fn set_detecting_width(&mut self, detecting_width: i32) {
            self.widget_props_mut().detecting_width = detecting_width
        }

        #[inline]
        fn set_detecting_height(&mut self, detecting_height: i32) {
            self.widget_props_mut().detecting_height = detecting_height
        }

        #[inline]
        fn set_resize_redraw(&mut self, is: bool) {
            self.widget_props_mut().resize_redraw = is
        }

        #[inline]
        fn is_manage_by_container(&self) -> bool {
            self.widget_props().manage_by_container
        }

        #[inline]
        fn set_manage_by_container(&mut self, manage_by_container: bool) {
            self.widget_props_mut().manage_by_container = manage_by_container
        }

        #[inline]
        fn children_index(&self) -> &HashSet<ObjectId> {
            &self.widget_props().children_index
        }

        #[inline]
        fn children_index_mut(&mut self) -> &mut HashSet<ObjectId> {
            &mut self.widget_props_mut().children_index
        }
    };
}

impl<T: WidgetImpl> WidgetInnerExt for T {
    widget_inner_ext_impl!();
}
impl WidgetInnerExt for dyn WidgetImpl {
    widget_inner_ext_impl!();
}
