use crate::{
    application_window::ApplicationWindow, prelude::SharedWidget,
    primitive::global_watch::GlobalWatchEvent, shortcut::mgr::ShortcutMgr,
};
use std::ptr::NonNull;
use tlib::{
    events::{downcast_event, Event, EventType, KeyEvent, MouseEvent, ResizeEvent},
    nonnull_mut,
    object::ObjectOperation,
    types::StaticType,
};

use super::ElementExt;

pub(crate) fn win_evt_dispatch(window: &mut ApplicationWindow, evt: Event) -> Option<Event> {
    let mut event: Option<Event> = None;
    match evt.event_type() {
        // Window Resize.
        EventType::Resize => {
            let evt = downcast_event::<ResizeEvent>(evt).unwrap();
            window.set_fixed_width(evt.width());
            window.set_fixed_height(evt.height());
            window.when_size_change(evt.size());
        }

        // Mouse pressed.
        EventType::MouseButtonPress => {
            let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            let pos = evt.position().into();

            let prevent = window.handle_global_watch(GlobalWatchEvent::MousePressed, |handle| {
                handle.on_global_mouse_pressed(&evt)
            });
            if prevent {
                return event;
            }

            let modal_widget = window.modal_widget();

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if let Some(ref modal) = modal_widget {
                    if widget.id() != modal.id() && !modal.ancestor_of(widget.id()) {
                        continue;
                    }
                }

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

            let pressed_widget = window.pressed_widget();
            let prevent = window.handle_global_watch(GlobalWatchEvent::MouseReleased, |handle| {
                if pressed_widget != 0 {
                    return false;
                }
                handle.on_global_mouse_released(&evt)
            });
            if prevent {
                window.set_pressed_widget(0);
                return event;
            }

            if pressed_widget != 0 {
                let widget = nonnull_mut!(widgets_map.get_mut(&pressed_widget).unwrap());
                if widget.visible() {
                    let widget_point = widget.map_to_widget(&pos);
                    evt.set_position((widget_point.x(), widget_point.y()));

                    widget.on_mouse_released(evt.as_ref());
                    widget.inner_mouse_released(evt.as_ref(), false);

                    if widget.super_type().is_a(SharedWidget::static_type()) {
                        event = Some(evt);
                    }
                }
                window.set_pressed_widget(0);
            } else {
                let modal_widget = window.modal_widget();
                for (_name, widget_opt) in widgets_map.iter_mut() {
                    let widget = nonnull_mut!(widget_opt);
                    let visible = widget.visible();

                    if let Some(ref modal) = modal_widget {
                        if widget.id() != modal.id() && !modal.ancestor_of(widget.id()) {
                            continue;
                        }
                    }

                    if widget.point_effective(&pos) && visible {
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
            let evt_bak = evt.clone();
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            let shadow_mouse_watch = ApplicationWindow::window_of(window.id()).shadow_mouse_watch();
            let pos = evt.position().into();

            let prevent = window.handle_global_watch(GlobalWatchEvent::MouseMove, |handle| {
                handle.on_global_mouse_move(&evt)
            });
            if prevent {
                return event;
            }

            window.check_mouse_leave(&pos, &evt);

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if let Some(ref modal) = window.modal_widget() {
                    if widget.id() != modal.id() && !modal.ancestor_of(widget.id()) {
                        continue;
                    }
                }

                if !widget.visible() {
                    continue;
                }
                window.check_mouse_enter(widget, &pos, &evt);

                let widget_position = widget.map_to_widget(&pos);

                if widget.point_effective(&evt.position().into()) {
                    // process the `mouse_enter`,`mouse_leave` events:
                    if window.mouse_over_widget().is_none() {
                        window.set_mouse_over_widget(NonNull::new(widget));
                        let mouse_over = MouseEvent::new(
                            EventType::MouseOver,
                            (widget_position.x(), widget_position.y()),
                            evt.mouse_button(),
                            evt.modifier(),
                            evt.n_press(),
                            evt.delta(),
                            evt.delta_type(),
                        );

                        widget.on_mouse_over(&mouse_over);
                        widget.inner_mouse_over(&mouse_over);
                    } else {
                        let mouse_over_widget = nonnull_mut!(window.mouse_over_widget());
                        if widget.id() != mouse_over_widget.id() {
                            window.set_mouse_over_widget(NonNull::new(widget));

                            let mouse_out = MouseEvent::new(
                                EventType::MouseOut,
                                (widget_position.x(), widget_position.y()),
                                evt.mouse_button(),
                                evt.modifier(),
                                evt.n_press(),
                                evt.delta(),
                                evt.delta_type(),
                            );
                            mouse_over_widget.inner_mouse_out(&mouse_out);
                            mouse_over_widget.on_mouse_out(&mouse_out);

                            let mouse_over = MouseEvent::new(
                                EventType::MouseOver,
                                (widget_position.x(), widget_position.y()),
                                evt.mouse_button(),
                                evt.modifier(),
                                evt.n_press(),
                                evt.delta(),
                                evt.delta_type(),
                            );
                            widget.inner_mouse_over(&mouse_over);
                            widget.on_mouse_over(&mouse_over);
                        }
                    }

                    if !widget.mouse_tracking() {
                        continue;
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

            for hnd in shadow_mouse_watch.iter_mut() {
                let widget = nonnull_mut!(hnd);
                window.check_mouse_enter(widget, &pos, &evt_bak);
            }
        }

        // Mouse wheeled.
        EventType::MouseWhell => {
            let mut evt = downcast_event::<MouseEvent>(evt).unwrap();
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            let pos = evt.position().into();

            let prevent = window.handle_global_watch(GlobalWatchEvent::MouseWhell, |handle| {
                handle.on_global_mouse_whell(&evt)
            });
            if prevent {
                return event;
            }

            let modal_widget = window.modal_widget();

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if let Some(ref modal) = modal_widget {
                    if widget.id() != modal.id() && !modal.ancestor_of(widget.id()) {
                        continue;
                    }
                }

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

            let prevent = window.handle_global_watch(GlobalWatchEvent::KeyPressed, |handle| {
                handle.on_global_key_pressed(&evt)
            });
            if prevent {
                return event;
            }
            let global_shorcut_triggered = ShortcutMgr::with(|shortcut_manager| {
                let mut shortcut_manager = shortcut_manager.borrow_mut();
                shortcut_manager.receive_key_event(&evt);
                shortcut_manager.trigger_global()
            });
            if global_shorcut_triggered {
                return None;
            }

            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                if widget.id() == window.focused_widget() {
                    if !ShortcutMgr::with(|shortcut_manager| {
                        shortcut_manager.borrow_mut().trigger(widget.id())
                    }) {
                        widget.on_key_pressed(&evt);
                        widget.inner_key_pressed(&evt);
                    }

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

            let prevent = window.handle_global_watch(GlobalWatchEvent::KeyReleased, |handle| {
                handle.on_global_key_released(&evt)
            });
            if prevent {
                return event;
            }
            ShortcutMgr::with(|shortcut_manager| {
                shortcut_manager.borrow_mut().receive_key_event(&evt)
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

        EventType::WindowMaximized => {
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                widget.on_window_maximized();

                if let Some(ref f) = widget.callbacks().window_maximized {
                    f(nonnull_mut!(widget_opt))
                }
            }
        }

        EventType::WindowMinimized => {
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                widget.on_window_minimized();

                if let Some(ref f) = widget.callbacks().window_minimized {
                    f(nonnull_mut!(widget_opt))
                }
            }
        }

        EventType::WindowRestored => {
            let widgets_map = ApplicationWindow::widgets_of(window.id());
            for (_name, widget_opt) in widgets_map.iter_mut() {
                let widget = nonnull_mut!(widget_opt);

                widget.on_window_restored();

                if let Some(ref f) = widget.callbacks().window_restored {
                    f(nonnull_mut!(widget_opt))
                }
            }
        }

        EventType::FocusIn => {
            window.restore_focus();
            event = Some(evt);
        }

        EventType::FocusOut => {
            window.temp_lose_focus();
            event = Some(evt);
        }

        EventType::Moved => {}
        EventType::DroppedFile => {}
        EventType::HoveredFile => {}
        EventType::HoveredFileCancelled => {}
        EventType::ReceivedCharacter => {}
        EventType::InputMethod => {}
        EventType::None => {}

        _ => {}
    }

    event
}
