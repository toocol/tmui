use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        // Systems.
        macos_platform: { target_os = "macos" },
        ios_platform: { target_os = "ios" },
        windows_platform: { target_os = "windows" },
        apple: { any(target_os = "ios", target_os = "macos") },
        free_unix: { all(unix, not(apple)) },
        redox: { target_os = "redox" },

        // Native displays.
        x11_platform: { all(feature = "x11_platform", free_unix, not(redox)) },
        wayland_platform: { all(feature = "wayland_platform", free_unix, not(redox)) },
        orbital_platform: { redox },

        // Others:
        font_awesome: { feature = "font_awesome" },
        verbose_logging: { feature = "verbose_logging" },
        win_popup: { feature = "win_popup" },
        win_tooltip: { feature = "win_tooltip" },
        win_dialog: { feature = "win_dialog" },
        win_select: { feature = "win_select" },
    }
}
