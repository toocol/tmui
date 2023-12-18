use std::ptr::NonNull;
use super::{Animatable, Direction};
use crate::{
    animation::state_holder::ReflectRectHolder, prelude::*, primitive::frame::Frame,
    widget::WidgetImpl,
};

#[reflect_trait]
pub trait Snapshot: WidgetImpl + Animatable {
    fn as_snapshot(&self) -> &dyn Snapshot;

    fn as_snapshot_mut(&mut self) -> &mut dyn Snapshot;

    fn start(&mut self) {
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
                    Direction::BottomToTop => {}
                    Direction::LeftTopToRightBottom => {}
                    Direction::LeftBottomToRightTop => {}
                    Direction::RightTopToLeftBottom => {}
                    _ => {}
                };

                start.set_x(start.x() - start.width());
                let end = self.rect();

                let widget = self;
                let hold =
                    NonNull::new(cast_mut!(widget as RectHolder).unwrap().animated_rect_mut());

                widget
                    .animation_model_mut()
                    .start(AnimationsHolder::Linear { start, end, hold });
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

                    ApplicationWindow::window_of(widget.window_id())
                        .invalid_effected_widgets(dirty_rect);
                }
            }

            widget.animation_model_mut().update(frame.timestamp());
            widget.update();
        }
    }
}
