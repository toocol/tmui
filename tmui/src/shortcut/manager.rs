use std::{collections::HashMap, ptr::NonNull};

use crate::widget::WidgetImpl;

use super::Shortcut;

#[derive(Default)]
pub(crate) struct ShortcutManager {
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
    pub(crate) fn register_shortcut<F: Fn(&mut dyn WidgetImpl)>(
        &mut self,
        shortcut: Shortcut,
        widget: &mut dyn WidgetImpl,
        f: F,
    ) {
    }

    pub(crate) fn register_global_shortcut<F: Fn(&mut dyn WidgetImpl)>(
        &mut self,
        shortcut: Shortcut,
        widget: &mut dyn WidgetImpl,
        f: F,
    ) {
    }
}
