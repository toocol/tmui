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

                let widget = self;
                let hold =
                    NonNull::new(cast_mut!(widget as RectHolder).unwrap().animated_rect_mut());

                widget
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
            let widget = self;
            if let Some(rect_holder) = cast!(widget as RectHolder) {
                if let Some(_) = cast!(widget as PopupImpl) {
                    let dirty_rect = rect_holder.animated_rect();

                    if dirty_rect.is_valid() {
                        ApplicationWindow::window_of(widget.window_id())
                            .invalid_effected_widgets(dirty_rect);
                    }
                }
            }

            widget.animation_model_mut().update(frame.timestamp());

            if let Some(rect_holder) = cast!(widget as RectHolder) {
                let rect = rect_holder.animated_rect();
                println!("{:?}", rect);
                widget.set_fixed_x(rect.x());
                widget.set_fixed_y(rect.y());
                widget.set_fixed_width(rect.width());
                widget.set_fixed_height(rect.height());
                ApplicationWindow::window_of(widget.window_id())
                    .layout_change(widget.as_widget_mut());
            }
            if let Some(color_holder) = cast!(widget as ColorHolder) {}

            widget.propagate_update();
        }
    }
}
