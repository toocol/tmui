pub mod manager;

use tlib::{bitflags::bitflags, events::KeyEvent};

bitflags! {
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
        const Key6 = 1 << 34;
        const Key7 = 1 << 35;
        const Key8 = 1 << 36;
        const Key9 = 1 << 37;

        const F1 = 1 << 38;
        const F2 = 1 << 39;
        const F3 = 1 << 40;
        const F4 = 1 << 41;
        const F5 = 1 << 42;
        const F6 = 1 << 43;
        const F7 = 1 << 44;
        const F8 = 1 << 45;
        const F9 = 1 << 46;
        const F10 = 1 << 47;
        const F11 = 1 << 48;
        const F12 = 1 << 49;
    }
}

pub(crate) trait ShortcutTrigger {
    fn trigger_shortcut(&self) -> Shortcut;
}
impl ShortcutTrigger for KeyEvent {
    fn trigger_shortcut(&self) -> Shortcut {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use tlib::shortcut;
    use super::Shortcut;

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
}