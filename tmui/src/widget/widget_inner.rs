use super::{Container, EventBubble, WidgetImpl, WidgetSignals};
use crate::graphics::painter::Painter;
use std::collections::HashSet;
use tlib::{
    emit,
    figure::{Rect, Size},
    namespace::Overflow,
    object::ObjectId,
    prelude::*,
    typedef::SkiaClipOp,
    types::StaticType,
};

pub(crate) trait WidgetInnerExt {
    fn set_initialized(&mut self, initialized: bool);

    fn first_rendered(&self) -> bool;

    fn set_first_rendered(&mut self);

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

    fn child_image_rect_union(&self) -> &Rect;

    fn child_image_rect_union_mut(&mut self) -> &mut Rect;

    fn need_update_geometry(&self) -> bool;

    fn child_overflow_rect(&self) -> &Rect;

    fn child_overflow_rect_mut(&mut self) -> &mut Rect;

    fn image_rect_record(&self) -> Rect;

    fn set_image_rect_record(&mut self, image_rect: Rect);

    fn clip_rect(&self, painter: &mut Painter, op: SkiaClipOp);

    fn handle_child_overflow_hidden(&mut self, child_size: Size);
}

macro_rules! widget_inner_ext_impl {
    () => {
        #[inline]
        fn set_initialized(&mut self, initialized: bool) {
            self.widget_props_mut().initialized = initialized
        }

        #[inline]
        fn first_rendered(&self) -> bool {
            self.widget_props().first_rendered
        }

        #[inline]
        fn set_first_rendered(&mut self) {
            self.widget_props_mut().first_rendered = true
        }

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

        #[inline]
        fn child_image_rect_union(&self) -> &Rect {
            &self.widget_props().child_image_rect_union
        }

        #[inline]
        fn child_image_rect_union_mut(&mut self) -> &mut Rect {
            &mut self.widget_props_mut().child_image_rect_union
        }

        #[inline]
        fn need_update_geometry(&self) -> bool {
            self.widget_props().need_update_geometry
        }

        #[inline]
        fn child_overflow_rect(&self) -> &Rect {
            &self.widget_props().child_overflow_rect
        }

        #[inline]
        fn child_overflow_rect_mut(&mut self) -> &mut Rect {
            &mut self.widget_props_mut().child_overflow_rect
        }

        #[inline]
        fn image_rect_record(&self) -> Rect {
            self.widget_props().old_image_rect
        }

        #[inline]
        fn set_image_rect_record(&mut self, image_rect: Rect) {
            self.widget_props_mut().old_image_rect = image_rect
        }

        #[inline]
        fn clip_rect(&self, painter: &mut Painter, op: SkiaClipOp) {
            if self.border_ref().border_radius != 0. {
                painter.clip_round_rect_global(self.rect(), self.border_ref().border_radius, op);
            } else {
                painter.clip_rect_global(self.rect(), op);
            }
        }

        fn handle_child_overflow_hidden(&mut self, child_size: Size) {
            let size = self.borderless_size();
            if !self.super_type().is_a(Container::static_type()) {
                if let Some(c) = self.get_child_mut() {
                    if c.overflow() != Overflow::Hidden {
                        return
                    }
                    let mut resized = false;

                    if child_size.width() > size.width() {
                        c.set_fixed_width(size.width());
                        resized = true;
                    }

                    if child_size.height() > size.height() {
                        c.set_fixed_height(size.height());
                        resized = true;
                    }

                    if resized {
                        emit!(WidgetInnerExt::handle_child_overflow_hidden => c.size_changed(), c.size())
                    }
                }
            }
        }
    };
}

impl<T: WidgetImpl> WidgetInnerExt for T {
    widget_inner_ext_impl!();
}
impl WidgetInnerExt for dyn WidgetImpl {
    widget_inner_ext_impl!();
}
