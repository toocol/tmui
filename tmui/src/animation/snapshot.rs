use super::{
    state_holder::ReflectTransparencyHolder, Animatable, AnimationEffect, AnimationMode,
    AnimationState, Direction,
};
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

        self.animation_model_mut().set_shown(show);
        self.propagate_animation_progressing(true);
        self.handle_dirty_rect(false);

        let animation = self.animation();

        match animation {
            Animation::Linear | Animation::EaseIn | Animation::EaseOut => {
                let rect = self.rect();
                self.animation_model_mut().set_origin_rect(rect);

                let mut start = self.rect();
                let mut end = self.rect();
                let modify_target = if show { &mut start } else { &mut end };

                let is_appearance =
                    self.animation_model().effect().unwrap() == AnimationEffect::Appearance;

                match self.animation_model().direction.unwrap() {
                    Direction::LeftToRight => {
                        if is_appearance {
                            modify_target.set_width(0);
                        } else {
                            modify_target.set_x(modify_target.left() - modify_target.width());
                        }
                    }
                    Direction::TopToBottom => {
                        if is_appearance {
                            modify_target.set_height(0);
                        } else {
                            modify_target.set_y(modify_target.top() - modify_target.height());
                        }
                    }
                    Direction::RightToLeft => {
                        modify_target.set_x(modify_target.right());
                        if is_appearance {
                            modify_target.set_width(0);
                        }
                    }
                    Direction::BottomToTop => {
                        modify_target.set_y(modify_target.top() + modify_target.height());
                        if is_appearance {
                            modify_target.set_height(0);
                        }
                    }
                    Direction::LeftTopToRightBottom => {
                        if is_appearance {
                            modify_target.set_width(0);
                            modify_target.set_height(0);
                        } else {
                            modify_target.set_x(modify_target.left() - modify_target.width());
                            modify_target.set_y(modify_target.top() - modify_target.height());
                        }
                    }
                    Direction::LeftBottomToRightTop => {
                        modify_target.set_y(modify_target.top() + modify_target.height());

                        if is_appearance {
                            modify_target.set_width(0);
                            modify_target.set_height(0);
                        } else {
                            modify_target.set_x(modify_target.left() - modify_target.width());
                        }
                    }
                    Direction::RightTopToLeftBottom => {
                        modify_target.set_x(modify_target.right());

                        if is_appearance {
                            modify_target.set_width(0);
                            modify_target.set_height(0);
                        } else {
                            modify_target.set_y(modify_target.top() - modify_target.height());
                        }
                    }
                    Direction::RightBottomToLeftTop => {
                        modify_target.set_x(modify_target.right());
                        modify_target.set_y(modify_target.top() + modify_target.height());

                        if is_appearance {
                            modify_target.set_width(0);
                            modify_target.set_height(0);
                        }
                    }
                };

                let hold = NonNull::new(cast_mut!(self as RectHolder).unwrap().animated_rect_mut());

                self.animation_model_mut()
                    .start(animation.create_rect_holder(start, end, hold), win_id);
            }
            Animation::FadeLinear | Animation::FadeEaseIn | Animation::FadeEaseOut => {
                if self.animation_model().origin_transparency().is_none() {
                    let origin = self.transparency() as i32;
                    self.animation_model_mut().set_origin_transparency(origin);
                }

                let transparency = self.transparency() as i32;
                let mut start = transparency;
                let mut end = self.animation_model().origin_transparency().unwrap();

                if show {
                    start = 0;
                } else {
                    end = 0;
                }

                let hold = NonNull::new(
                    cast_mut!(self as TransparencyHolder)
                        .unwrap()
                        .animated_transparency_mut(),
                );

                self.animation_model_mut().start(
                    animation.create_transparency_holder(start, end, hold),
                    win_id,
                )
            }
        }
    }

    fn snapshot(&mut self, frame: Frame) {
        match self.animation_model().state() {
            AnimationState::Playing => {
                self.handle_dirty_rect(false);

                self.animation_model_mut().update(frame.timestamp());

                if let Some(rect_holder) = cast!(self as RectHolder) {
                    let rect = rect_holder.animated_rect();
                    self.set_fixed_x(rect.x());
                    self.set_fixed_y(rect.y());
                    self.set_fixed_width(rect.width());
                    self.set_fixed_height(rect.height());
                    if self.animation_model().mode() == AnimationMode::Flex {
                        ApplicationWindow::window_of(self.window_id())
                            .animation_layout_change(self.as_widget_mut());
                    }
                }
                if let Some(transparency_holder) = cast!(self as TransparencyHolder) {
                    self.propagate_set_transparency(
                        transparency_holder.animated_transparency() as u8
                    )
                }

                self.propagate_update_styles_rect(CoordRect::new(self.rect(), Coordinate::World));
                self.set_render_styles(true);
            }

            AnimationState::Pending => {
                if !self.animation_model().shown() {
                    self.handle_dirty_rect(true)
                }

                if let Some(origin) = self.animation_model().origin_rect() {
                    self.set_fixed_x(origin.x());
                    self.set_fixed_y(origin.y());
                    self.set_fixed_width(origin.width());
                    self.set_fixed_height(origin.height());
                }

                if let Some(transparency) = self.animation_model().origin_transparency() {
                    self.propagate_set_transparency(transparency as u8)
                }

                self.propagate_animation_progressing(false);
                self.animation_model_mut()
                    .set_state(AnimationState::Stopped);
                self.animation_model_mut().pending_clear();
            }

            _ => {}
        }
    }
}

pub(crate) trait EffectedWidget: WidgetImpl {
    fn handle_dirty_rect(&mut self, reset: bool) {
        if cast!(self as PopupImpl).is_some() {
            let win_id = self.window_id();
            let id = self.id();

            if let Some(rect_holder) = cast_mut!(self as RectHolder) {
                let dirty_rect = rect_holder.animated_rect();

                if dirty_rect.is_valid() {
                    ApplicationWindow::window_of(win_id).invalid_effected_widgets(dirty_rect, id);
                }

                if reset {
                    rect_holder.animated_rect_mut().clear()
                }
            } else {
                let dirty_rect = self.rect_record();

                if dirty_rect.is_valid() {
                    ApplicationWindow::window_of(win_id).invalid_effected_widgets(dirty_rect.into(), id);
                }
            }
        }
    }
}
impl<T: ?Sized + Snapshot> EffectedWidget for T {}
