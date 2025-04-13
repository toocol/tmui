use super::{Shortcut, ShortcutTrigger};
use crate::widget::{WidgetHnd, WidgetImpl};
use nohash_hasher::IntMap;
use std::{cell::RefCell, ptr::NonNull};
use tlib::{
    events::{EventTrait, EventType, KeyEvent},
    nonnull_mut, nonnull_ref,
    object::ObjectId,
};

thread_local! {
    static INSTANCE: RefCell<ShortcutMgr> = RefCell::new(ShortcutMgr::new());
}

type ShortcutMap = IntMap<Shortcut, Vec<(WidgetHnd, Box<dyn Fn(&mut dyn WidgetImpl)>)>>;

pub(crate) struct ShortcutMgr {
    shortcut: Shortcut,

    shortcuts: ShortcutMap,

    global_shortcuts: ShortcutMap,
}

impl ShortcutMgr {
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
        F: FnOnce(&RefCell<ShortcutMgr>) -> R,
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
    pub(crate) fn remove_shortcut_all(&mut self, id: ObjectId) {
        for shortcuts in self.shortcuts.values_mut() {
            shortcuts.retain(|(r, _)| nonnull_ref!(r).id() != id);
        }
        for shortcuts in self.global_shortcuts.values_mut() {
            shortcuts.retain(|(r, _)| nonnull_ref!(r).id() != id);
        }
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
                self.shortcut = Shortcut::empty();
            }
            _ => {}
        }
    }
}
