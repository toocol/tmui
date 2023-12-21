use super::{state_holder::ReflectColorHolder, Animatable, Direction};
use crate::{
    animation::state_holder::ReflectRectHolder, prelude::*, primitive::frame::Frame,
    widget::WidgetImpl,
};
use std::ptr::NonNull;

#[reflect_trait]
pub trait Snapshot: WidgetImpl + Animatable {
    fn as_snapshot(&self) -> &dyn Snapshot;

    fn as_snapshot_mut(&mut self) -> &mut dyn Snapshot;

    fn as_widget(&self) -> &dyn WidgetImpl;

    fn as_widget_mut(&mut self) -> &mut dyn WidgetImpl;

    fn start(&mut self, show: bool) {
        let win_id = self.window_id();
        match self.animation() {
            Animation::Linear => {
                let mut start = self.rect();

                match self.animation_model().direction {
                    Direction::NoDirection => panic!("Animiation must specify direction."),
                    Direction::LeftToRight => start.set_width(0),
                    Direction::TopToBottom => start.set_height(0),
                    Direction::RightToLeft => {
                        start.set_x(start.right());
                        start.set_width(0);
                    }
                    Direction::BottomToTop => {
                        // start.set_y(start.y() + start.height());
                        start.set_height(0);
                    }
                    Direction::LeftTopToRightBottom => {}
                    Direction::LeftBottomToRightTop => {}
                    Direction::RightTopToLeftBottom => {}
                    _ => {}
                };

                let end = self.rect();

                let hold =
                    NonNull::new(cast_mut!(self as RectHolder).unwrap().animated_rect_mut());

                self
                    .animation_model_mut()
                    .start(AnimationsHolder::Linear { start, end, hold }, win_id);
            }
            Animation::EaseIn => {}
            Animation::EaseOut => {}
            Animation::FadeLinear => {}
            Animation::FadeEaseIn => {}
            Animation::FadeEaseOut => {}
            Animation::NoAnimation => {}
        }
    }

    fn snapshot(&mut self, frame: Frame) {
        if self.animation_model().is_playing() {
            if let Some(rect_holder) = cast!(self as RectHolder) {
                if let Some(_) = cast!(self as PopupImpl) {
                    let dirty_rect = rect_holder.animated_rect();

                    if dirty_rect.is_valid() {
                        ApplicationWindow::window_of(self.window_id())
                            .invalid_effected_widgets(dirty_rect);
                    }
                }
            }

            self.animation_model_mut().update(frame.timestamp());

            if let Some(rect_holder) = cast!(self as RectHolder) {
                let rect = rect_holder.animated_rect();
                println!("{:?}", rect);
                self.set_fixed_x(rect.x());
                self.set_fixed_y(rect.y());
                self.set_fixed_width(rect.width());
                self.set_fixed_height(rect.height());
                ApplicationWindow::window_of(self.window_id())
                    .layout_change(self.as_widget_mut());
            }
            if let Some(color_holder) = cast!(self as ColorHolder) {}

            self.propagate_update();
        }
    }
}
