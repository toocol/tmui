use super::{Shortcut, ShortcutTrigger};
use crate::widget::WidgetImpl;
use std::{cell::RefCell, collections::HashMap, ptr::NonNull};
use tlib::{
    events::{EventTrait, EventType, KeyEvent},
    nonnull_mut,
    object::ObjectId,
};

thread_local! {
    static INSTANCE: RefCell<ShortcutManager> = RefCell::new(ShortcutManager::new());
}

pub(crate) struct ShortcutManager {
    shortcut: Shortcut,

    shortcuts: HashMap<
        Shortcut,
        Vec<(
            Option<NonNull<dyn WidgetImpl>>,
            Box<dyn Fn(&mut dyn WidgetImpl)>,
        )>,
    >,

    global_shortcuts: HashMap<
        Shortcut,
        Vec<(
            Option<NonNull<dyn WidgetImpl>>,
            Box<dyn Fn(&mut dyn WidgetImpl)>,
        )>,
    >,
}

impl ShortcutManager {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            shortcut: Shortcut::empty(),
            shortcuts: Default::default(),
            global_shortcuts: Default::default(),
        }
    }

    #[inline]
    pub(crate) fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&RefCell<ShortcutManager>) -> R,
    {
        INSTANCE.with(f)
    }

    #[inline]
    pub(crate) fn register_shortcut<F: Fn(&mut dyn WidgetImpl) + 'static>(
        &mut self,
        shortcut: Shortcut,
        widget: &mut dyn WidgetImpl,
        f: F,
    ) {
        self.shortcuts
            .entry(shortcut)
            .or_default()
            .push((NonNull::new(widget), Box::new(f)));
    }

    #[inline]
    pub(crate) fn register_global_shortcut<F: Fn(&mut dyn WidgetImpl) + 'static>(
        &mut self,
        shortcut: Shortcut,
        widget: &mut dyn WidgetImpl,
        f: F,
    ) {
        self.global_shortcuts
            .entry(shortcut)
            .or_default()
            .push((NonNull::new(widget), Box::new(f)));
    }

    #[inline]
    pub(crate) fn trigger(&mut self, id: ObjectId) -> bool {
        let mut trigged = false;

        if let Some(widgets) = self.shortcuts.get_mut(&self.shortcut) {
            widgets.iter_mut().for_each(|(widget, f)| {
                let widget = nonnull_mut!(widget);
                if id == widget.id() {
                    f(widget);
                    trigged = true;
                }
            })
        }

        trigged
    }

    #[inline]
    pub(crate) fn trigger_global(&mut self) -> bool {
        let mut trigged = false;

        if let Some(widgets) = self.global_shortcuts.get_mut(&self.shortcut) {
            widgets.iter_mut().for_each(|(widget, f)| {
                f(nonnull_mut!(widget));
                trigged = true;
            })
        }

        trigged
    }

    #[inline]
    pub(crate) fn receive_key_event(&mut self, evt: &KeyEvent) {
        match evt.event_type() {
            EventType::KeyPress => {
                self.shortcut.insert(evt.trigger_shortcut());
            }
            EventType::KeyRelease => {
                self.shortcut.remove(evt.trigger_shortcut());
            }
            _ => {}
        }
    }
}
