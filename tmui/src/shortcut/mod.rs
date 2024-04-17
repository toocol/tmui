pub mod manager;

use self::manager::ShortcutManager;
use crate::widget::WidgetImpl;
use tlib::{
    bitflags::bitflags,
    events::KeyEvent,
    namespace::{KeyCode, KeyboardModifier},
};

bitflags! {
    /// To simplify construction, use proc-macro [`shortcut!`](crate::tlib::shortcut).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Shortcut: u64 {
        // Modifiers.
        const Control = 1;
        const Alt = 1 << 1;
        const Shift = 1 << 2;
        const Meta = 1 << 3;

        // Letters.
        const A = 1 << 4;
        const B = 1 << 5;
        const C = 1 << 6;
        const D = 1 << 7;
        const E = 1 << 8;
        const F = 1 << 9;
        const G = 1 << 10;
        const H = 1 << 11;
        const I = 1 << 12;
        const J = 1 << 13;
        const K = 1 << 14;
        const L = 1 << 15;
        const M = 1 << 16;
        const N = 1 << 17;
        const O = 1 << 18;
        const P = 1 << 19;
        const Q = 1 << 20;
        const R = 1 << 21;
        const S = 1 << 22;
        const T = 1 << 23;
        const U = 1 << 24;
        const V = 1 << 25;
        const W = 1 << 26;
        const X = 1 << 27;
        const Y = 1 << 28;
        const Z = 1 << 29;

        // The number keys over the letters.
        const Key0 = 1 << 30;
        const Key1 = 1 << 31;
        const Key2 = 1 << 32;
        const Key3 = 1 << 33;
        const Key4 = 1 << 34;
        const Key5 = 1 << 35;
        const Key6 = 1 << 36;
        const Key7 = 1 << 37;
        const Key8 = 1 << 38;
        const Key9 = 1 << 39;

        const F1 = 1 << 40;
        const F2 = 1 << 41;
        const F3 = 1 << 42;
        const F4 = 1 << 43;
        const F5 = 1 << 44;
        const F6 = 1 << 45;
        const F7 = 1 << 46;
        const F8 = 1 << 47;
        const F9 = 1 << 48;
        const F10 = 1 << 49;
        const F11 = 1 << 50;
        const F12 = 1 << 51;

        // Some funcional keys:
        const Insert = 1 << 52;
        const Delete = 1 << 53;
        const Home = 1 << 54;
        const End = 1 << 55;
        const PageUp = 1 << 56;
        const PageDown = 1 << 57;
        const Left = 1 << 58;
        const Up = 1 << 59;
        const Right = 1 << 60;
        const Down = 1 << 61;
    }
}

pub trait ShortcutRegister: WidgetImpl + Sized {
    /// Widget register the shortcut handdle. <br>
    /// `Notice: the widget must be `enable_focus`, if not, the shortcut register will be ignored.`
    #[inline]
    fn register_shortcut<F: Fn(&mut dyn WidgetImpl) + 'static>(
        &mut self,
        shortcut: Shortcut,
        f: F,
    ) {
        ShortcutManager::with(|shortcut_manager| {
            shortcut_manager
                .borrow_mut()
                .register_shortcut(shortcut, self, f)
        });
    }

    /// Widget register the global shortcut handdle.
    #[inline]
    fn register_global_shortcut<F: Fn(&mut dyn WidgetImpl) + 'static>(
        &mut self,
        shortcut: Shortcut,
        f: F,
    ) {
        ShortcutManager::with(|shortcut_manager| {
            shortcut_manager
                .borrow_mut()
                .register_global_shortcut(shortcut, self, f)
        });
    }
}
impl<T: WidgetImpl> ShortcutRegister for T {}

pub(crate) trait ShortcutTrigger {
    fn trigger_shortcut(&self) -> Shortcut;
}
impl ShortcutTrigger for KeyEvent {
    fn trigger_shortcut(&self) -> Shortcut {
        let mut shortcut = Shortcut::empty();

        let modifier = self.modifier();
        if modifier.has(KeyboardModifier::ShiftModifier) {
            shortcut.insert(Shortcut::Shift)
        }
        if modifier.has(KeyboardModifier::ControlModifier) {
            shortcut.insert(Shortcut::Control)
        }
        if modifier.has(KeyboardModifier::AltModifier) {
            shortcut.insert(Shortcut::Alt)
        }
        if modifier.has(KeyboardModifier::MetaModifier) {
            shortcut.insert(Shortcut::Meta)
        }

        match self.key_code() {
            KeyCode::KeyA => shortcut.insert(Shortcut::A),
            KeyCode::KeyB => shortcut.insert(Shortcut::B),
            KeyCode::KeyC => shortcut.insert(Shortcut::C),
            KeyCode::KeyD => shortcut.insert(Shortcut::D),
            KeyCode::KeyE => shortcut.insert(Shortcut::E),
            KeyCode::KeyF => shortcut.insert(Shortcut::F),
            KeyCode::KeyG => shortcut.insert(Shortcut::G),
            KeyCode::KeyH => shortcut.insert(Shortcut::H),
            KeyCode::KeyI => shortcut.insert(Shortcut::I),
            KeyCode::KeyJ => shortcut.insert(Shortcut::J),
            KeyCode::KeyK => shortcut.insert(Shortcut::K),
            KeyCode::KeyL => shortcut.insert(Shortcut::L),
            KeyCode::KeyM => shortcut.insert(Shortcut::M),
            KeyCode::KeyN => shortcut.insert(Shortcut::N),
            KeyCode::KeyO => shortcut.insert(Shortcut::O),
            KeyCode::KeyP => shortcut.insert(Shortcut::P),
            KeyCode::KeyQ => shortcut.insert(Shortcut::Q),
            KeyCode::KeyR => shortcut.insert(Shortcut::R),
            KeyCode::KeyS => shortcut.insert(Shortcut::S),
            KeyCode::KeyT => shortcut.insert(Shortcut::T),
            KeyCode::KeyU => shortcut.insert(Shortcut::U),
            KeyCode::KeyV => shortcut.insert(Shortcut::V),
            KeyCode::KeyW => shortcut.insert(Shortcut::W),
            KeyCode::KeyX => shortcut.insert(Shortcut::X),
            KeyCode::KeyY => shortcut.insert(Shortcut::Y),
            KeyCode::KeyZ => shortcut.insert(Shortcut::Z),

            KeyCode::Key0 => shortcut.insert(Shortcut::Key0),
            KeyCode::Key1 => shortcut.insert(Shortcut::Key1),
            KeyCode::Key2 => shortcut.insert(Shortcut::Key2),
            KeyCode::Key3 => shortcut.insert(Shortcut::Key3),
            KeyCode::Key4 => shortcut.insert(Shortcut::Key4),
            KeyCode::Key5 => shortcut.insert(Shortcut::Key5),
            KeyCode::Key6 => shortcut.insert(Shortcut::Key6),
            KeyCode::Key7 => shortcut.insert(Shortcut::Key7),
            KeyCode::Key8 => shortcut.insert(Shortcut::Key8),
            KeyCode::Key9 => shortcut.insert(Shortcut::Key9),

            KeyCode::KeyF1 => shortcut.insert(Shortcut::F1),
            KeyCode::KeyF2 => shortcut.insert(Shortcut::F2),
            KeyCode::KeyF3 => shortcut.insert(Shortcut::F3),
            KeyCode::KeyF4 => shortcut.insert(Shortcut::F4),
            KeyCode::KeyF5 => shortcut.insert(Shortcut::F5),
            KeyCode::KeyF6 => shortcut.insert(Shortcut::F6),
            KeyCode::KeyF7 => shortcut.insert(Shortcut::F7),
            KeyCode::KeyF8 => shortcut.insert(Shortcut::F8),
            KeyCode::KeyF9 => shortcut.insert(Shortcut::F9),
            KeyCode::KeyF10 => shortcut.insert(Shortcut::F10),
            KeyCode::KeyF11 => shortcut.insert(Shortcut::F11),
            KeyCode::KeyF12 => shortcut.insert(Shortcut::F12),

            KeyCode::KeyInsert => shortcut.insert(Shortcut::Insert),
            KeyCode::KeyDelete => shortcut.insert(Shortcut::Delete),
            KeyCode::KeyHome => shortcut.insert(Shortcut::Home),
            KeyCode::KeyEnd => shortcut.insert(Shortcut::End),
            KeyCode::KeyPageUp => shortcut.insert(Shortcut::PageUp),
            KeyCode::KeyPageDown => shortcut.insert(Shortcut::PageDown),
            KeyCode::KeyLeft => shortcut.insert(Shortcut::Left),
            KeyCode::KeyUp => shortcut.insert(Shortcut::Up),
            KeyCode::KeyRight => shortcut.insert(Shortcut::Right),
            KeyCode::KeyDown => shortcut.insert(Shortcut::Down),
            _ => {}
        }

        shortcut
    }
}

#[macro_export]
macro_rules! cast_do {
    ( $ty:ident::$fn:ident($($arg:expr),*) ) => {
        |w| {
            w.downcast_mut::<$ty>().unwrap().$fn($($arg),*);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::shortcut::ShortcutTrigger;

    use super::Shortcut;
    use tlib::{
        events::{EventType, KeyEvent},
        namespace::{KeyCode, KeyboardModifier},
        shortcut,
    };

    #[test]
    fn test_shortcut_macro() {
        let shortcut = shortcut!(Control + Alt + A + F1 + 0);

        assert!(shortcut.contains(Shortcut::Control));
        assert!(shortcut.contains(Shortcut::Alt));
        assert!(shortcut.contains(Shortcut::A));
        assert!(shortcut.contains(Shortcut::F1));
        assert!(shortcut.contains(Shortcut::Key0));

        assert!(!shortcut.contains(Shortcut::Meta));
        assert!(!shortcut.contains(Shortcut::Shift));
        assert!(!shortcut.contains(Shortcut::Q));
        assert!(!shortcut.contains(Shortcut::F11));
        assert!(!shortcut.contains(Shortcut::Key1));
    }

    #[test]
    fn test_key_event_convert() {
        let key_code = KeyCode::KeyA;
        let modifier = KeyboardModifier::ControlModifier
            .or(KeyboardModifier::AltModifier)
            .or(KeyboardModifier::ShiftModifier);
        let evt = KeyEvent::new(EventType::KeyPress, key_code, modifier, "A");

        assert_eq!(evt.trigger_shortcut(), shortcut!(Control + Alt + Shift + A))
    }
}
