use crate::{
    application_window::ApplicationWindow,
    prelude::SharedWidget,
    primitive::global_watch::{GlobalWatchEvent},
    widget::widget_ext::WidgetExt,
};
use std::ptr::NonNull;
use tlib::{
    events::{downcast_event, Event, EventType, KeyEvent, MouseEvent, ResizeEvent},
    nonnull_mut,
    object::ObjectOperation,
    types::StaticType,
};

pub(crate) fn win_evt_dispatch(window: &mut ApplicationWindow, evt: Event) -> Option<Event> {
    let mut event: Option<Event> = None;
    match evt.event_type() {
        // Window Resize.
        EventType::Resize => {
            let evt = downcast_event::<ResizeEvent>(evt).unwrap();
            window.resize(Some(evt.width()), Some(evt.height()));
        }

        // Mouse pressed.
        EventType::MouseButtonPress => {
            let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            let pos = evt.position().into();

            window.handle_global_watch(GlobalWatchEvent::MousePress, |handle| {
                handle.on_global_mouse_pressed(&evt);
            });

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if widget.point_effective(&pos) {
                    let widget_point = widget.map_to_widget(&pos);
                    evt.set_position((widget_point.x(), widget_point.y()));

                    widget.on_mouse_pressed(evt.as_ref());
                    widget.inner_mouse_pressed(evt.as_ref(), false);

                    if widget.super_type().is_a(SharedWidget::static_type()) {
                        event = Some(evt);
                    }
                    break;
                }
            }
        }

        // Mouse released.
        EventType::MouseButtonRelease => {
            let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            let pos = evt.position().into();

            window.handle_global_watch(GlobalWatchEvent::MouseRelease, |handle| {
                handle.on_global_mouse_released(&evt);
            });

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if window.pressed_widget() != 0 {
                    if widget.id() == window.pressed_widget() {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));

                        widget.on_mouse_released(evt.as_ref());
                        widget.inner_mouse_released(evt.as_ref(), false);

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                } else {
                    if widget.point_effective(&pos) {
                        let widget_point = widget.map_to_widget(&pos);
                        evt.set_position((widget_point.x(), widget_point.y()));

                        widget.on_mouse_released(evt.as_ref());
                        widget.inner_mouse_released(evt.as_ref(), false);

                        if widget.super_type().is_a(SharedWidget::static_type()) {
                            event = Some(evt);
                        }
                        break;
                    }
                }
            }
        }

        // Mouse moved.
        EventType::MouseMove => {
            let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            let pos = evt.position().into();

            window.handle_global_watch(GlobalWatchEvent::MouseMove, |handle| {
                handle.on_global_mouse_move(&evt);
            });

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                let widget_position = widget.map_to_widget(&pos);

                if widget.point_effective(&evt.position().into()) {
                    // process the `mouse_enter`,`mouse_leave` events:
                    if window.mouse_over_widget().is_none() {
                        window.set_mouse_over_widget(NonNull::new(widget));
                        let mouse_enter = MouseEvent::new(
                            EventType::MouseEnter,
                            (widget_position.x(), widget_position.y()),
                            evt.mouse_button(),
                            evt.modifier(),
                            evt.n_press(),
                            evt.delta(),
                            evt.delta_type(),
                        );

                        widget.on_mouse_enter(&mouse_enter);
                        widget.inner_mouse_enter(&mouse_enter);
                    } else {
                        let mouse_over_widget = nonnull_mut!(window.mouse_over_widget());
                        if widget.id() != mouse_over_widget.id() {
                            window.set_mouse_over_widget(NonNull::new(widget));

                            let mouse_leave = MouseEvent::new(
                                EventType::MouseLeave,
                                (widget_position.x(), widget_position.y()),
                                evt.mouse_button(),
                                evt.modifier(),
                                evt.n_press(),
                                evt.delta(),
                                evt.delta_type(),
                            );

                            mouse_over_widget.on_mouse_leave(&mouse_leave);
                            mouse_over_widget.inner_mouse_leave(&mouse_leave);

                            let mouse_enter = MouseEvent::new(
                                EventType::MouseEnter,
                                (widget_position.x(), widget_position.y()),
                                evt.mouse_button(),
                                evt.modifier(),
                                evt.n_press(),
                                evt.delta(),
                                evt.delta_type(),
                            );
                            widget.inner_mouse_enter(&mouse_enter);
                            widget.on_mouse_enter(&mouse_enter);
                        }
                    }

                    if !widget.mouse_tracking() {
                        break;
                    }

                    evt.set_position((widget_position.x(), widget_position.y()));
                    widget.on_mouse_move(evt.as_ref());
                    widget.inner_mouse_move(evt.as_ref());

                    if widget.super_type().is_a(SharedWidget::static_type()) {
                        event = Some(evt);
                    }
                    break;
                }
            }
        }

        // Mouse wheeled.
        EventType::MouseWhell => {
            let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            let pos = evt.position().into();

            window.handle_global_watch(GlobalWatchEvent::MouseWhell, |handle| {
                handle.on_global_mouse_whell(&evt);
            });

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if widget.point_effective(&evt.position().into()) {
                    let widget_point = widget.map_to_widget(&pos);
                    evt.set_position((widget_point.x(), widget_point.y()));
                    widget.on_mouse_wheel(evt.as_ref());
                    widget.inner_mouse_wheel(evt.as_ref());

                    if widget.super_type().is_a(SharedWidget::static_type()) {
                        event = Some(evt);
                    }
                    break;
                }
            }
        }

        EventType::MouseEnter => event = Some(evt),

        EventType::MouseLeave => event = Some(evt),

        // Key pressed.
        EventType::KeyPress => {
            let evt = downcast_event::<KeyEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());

            window.handle_global_watch(GlobalWatchEvent::KeyPress, |handle| {
                handle.on_global_key_pressed(&evt);
            });

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if widget.id() == window.focused_widget() {
                    widget.on_key_pressed(&evt);
                    widget.inner_key_pressed(&evt);

                    if widget.super_type().is_a(SharedWidget::static_type()) {
                        event = Some(evt);
                    }
                    break;
                }
            }
        }

        // Key released.
        EventType::KeyRelease => {
            let evt = downcast_event::<KeyEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());

            window.handle_global_watch(GlobalWatchEvent::KeyRelease, |handle| {
                handle.on_global_key_released(&evt)
            });

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if widget.id() == window.focused_widget() {
                    widget.on_key_released(&evt);
                    widget.inner_key_released(&evt);

                    if widget.super_type().is_a(SharedWidget::static_type()) {
                        event = Some(evt);
                    }
                    break;
                }
            }
        }

        EventType::FocusIn => event = Some(evt),
        EventType::FocusOut => event = Some(evt),
        EventType::Moved => {}
        EventType::DroppedFile => {}
        EventType::HoveredFile => {}
        EventType::HoveredFileCancelled => {}
        EventType::ReceivedCharacter => {}
        EventType::InputMethod => {}
        EventType::None => {}
    }

    event
}
