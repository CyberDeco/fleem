//! OS appearance detection

/// Check if the system is in dark mode.
///
/// On macOS, reads `AppleInterfaceStyle` from user defaults.
/// On other platforms, defaults to dark.
#[cfg(target_os = "macos")]
pub fn is_system_dark_mode() -> bool {
    use std::process::Command;
    Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("Dark"))
        .unwrap_or(false)
}

#[cfg(target_os = "linux")]
pub fn is_system_dark_mode() -> bool {
    use std::process::Command;
    // Try freedesktop portal color-scheme setting
    Command::new("dbus-send")
        .args([
            "--session",
            "--print-reply=literal",
            "--dest=org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings.Read",
            "string:org.freedesktop.appearance",
            "string:color-scheme",
        ])
        .output()
        .map(|o| {
            let s = String::from_utf8_lossy(&o.stdout);
            // color-scheme: 1 = prefer dark, 2 = prefer light
            s.contains("uint32 1")
        })
        .unwrap_or(true)
}

#[cfg(target_os = "windows")]
pub fn is_system_dark_mode() -> bool {
    use std::process::Command;
    // Check Windows registry for AppsUseLightTheme
    Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "/v",
            "AppsUseLightTheme",
        ])
        .output()
        .map(|o| {
            let s = String::from_utf8_lossy(&o.stdout);
            // Value 0x0 means dark mode
            s.contains("0x0")
        })
        .unwrap_or(true)
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub fn is_system_dark_mode() -> bool {
    true // Default to dark on unknown platforms
}
