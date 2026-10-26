//! "Start with Windows": a value under HKCU\...\Run that starts the app in
//! the background (next to the clock) when the user signs in.

#[cfg(windows)]
mod imp {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    const KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    const NAME: &str = "EzooDownload";
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    fn reg(args: &[&str]) -> bool {
        Command::new(swiftget_engine::system::windows_tool("reg.exe"))
            .args(args)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn is_enabled() -> bool {
        reg(&["query", KEY, "/v", NAME])
    }

    /// Installed with the setup program (which asked about autostart itself).
    pub fn installed_by_setup() -> bool {
        reg(&["query", r"HKCU\Software\Ezoo\EzooDownload", "/v", "Installed"])
    }

    pub fn set(enabled: bool) -> bool {
        if enabled {
            let Ok(exe) = std::env::current_exe() else { return false };
            let value = format!("\"{}\" --background", exe.display());
            reg(&["add", KEY, "/v", NAME, "/t", "REG_SZ", "/d", &value, "/f"])
        } else {
            !is_enabled() || reg(&["delete", KEY, "/v", NAME, "/f"])
        }
    }
}

#[cfg(not(windows))]
mod imp {
    //! "Start at login" on Linux/Manjaro: an XDG autostart .desktop entry
    //! (~/.config/autostart), honored by KDE Plasma, GNOME, XFCE and others.
    use std::fs;
    use std::path::PathBuf;

    const DESKTOP_NAME: &str = "ezoo-download.desktop";

    fn autostart_file() -> Option<PathBuf> {
        let config_dir = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))?;
        Some(config_dir.join("autostart").join(DESKTOP_NAME))
    }

    pub fn is_enabled() -> bool {
        autostart_file().map(|p| p.exists()).unwrap_or(false)
    }

    pub fn set(enabled: bool) -> bool {
        let Some(path) = autostart_file() else { return false };
        if !enabled {
            // already disabled counts as success
            return fs::remove_file(&path).is_ok() || !path.exists();
        }
        let Ok(exe) = std::env::current_exe() else { return false };
        let content = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=Ezoo Download\n\
             Comment=Start Ezoo Download in the background at login\n\
             Exec=\"{}\" --background\n\
             Terminal=false\n\
             X-GNOME-Autostart-enabled=true\n",
            exe.display()
        );
        let Some(parent) = path.parent() else { return false };
        if fs::create_dir_all(parent).is_err() {
            return false;
        }
        fs::write(&path, content).is_ok()
    }

    /// Installed with the setup program (a Windows idea); never on Linux.
    pub fn installed_by_setup() -> bool {
        false
    }
}

pub use imp::{installed_by_setup, is_enabled, set};
