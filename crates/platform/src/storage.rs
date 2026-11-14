//! Where the port keeps the player's files: saved games (and later the
//! high scores) in the platform's data directory, and its settings
//! ([`crate::settings`]) in the config directory. The
//! original writes its saves into the game's own `MAPS\` directory; the
//! port leaves the game's files alone and only reads from there. In the
//! browser both are [`WEB_STORAGE`], where the browser's storage is
//! mounted (`crate::fs`, `docs/web.md`).

use std::path::{Path, PathBuf};

/// Where the browser build mounts the browser's storage: its data and
/// config directory.
pub const WEB_STORAGE: &str = "/storage";

/// The port's directory names: lower case on Linux and the other Unixes,
/// as written on Windows and macOS.
const NAME_UNIX: &str = "vacationdestination";
const NAME: &str = "VacationDestination";

/// The names the port's directories had before it was VacationDestination.
/// A directory still under its old name moves to the new one the first
/// time the port asks for it.
const OLD_NAME_UNIX: &str = "holiday-island";
const OLD_NAME: &str = "Holiday Island";

/// The directory the port's own files go in: `$XDG_DATA_HOME/vacationdestination`
/// (or `~/.local/share/vacationdestination`) on Linux and the other Unixes,
/// `~/Library/Application Support/VacationDestination` on macOS and
/// `%APPDATA%\VacationDestination` on Windows. `None` if the environment
/// doesn't say where the home directory is. [`WEB_STORAGE`] in the
/// browser.
#[must_use]
pub fn data_dir() -> Option<PathBuf> {
    if cfg!(target_arch = "wasm32") {
        return Some(PathBuf::from(WEB_STORAGE));
    }
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    let parent = if cfg!(target_os = "windows") {
        var("APPDATA")
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|h| h.join("Library/Application Support"))
    } else {
        var("XDG_DATA_HOME").or_else(|| var("HOME").map(|h| h.join(".local/share")))
    };
    parent.as_deref().map(named)
}

/// Where saved games go: `MAPS` in [`data_dir`], like the game's own
/// `MAPS\`.
#[must_use]
pub fn save_dir() -> Option<PathBuf> {
    data_dir().map(|d| d.join("MAPS"))
}

/// The directory the port's settings go in: `$XDG_CONFIG_HOME/vacationdestination`
/// (or `~/.config/vacationdestination`) on Linux and the other Unixes,
/// `~/Library/Preferences/VacationDestination` on macOS and
/// `%APPDATA%\VacationDestination` on Windows. `None` if the environment
/// doesn't say where the home directory is. [`WEB_STORAGE`] in the
/// browser.
#[must_use]
pub fn config_dir() -> Option<PathBuf> {
    if cfg!(target_arch = "wasm32") {
        return Some(PathBuf::from(WEB_STORAGE));
    }
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    let parent = if cfg!(target_os = "windows") {
        var("APPDATA")
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|h| h.join("Library/Preferences"))
    } else {
        var("XDG_CONFIG_HOME").or_else(|| var("HOME").map(|h| h.join(".config")))
    };
    parent.as_deref().map(named)
}

/// The port's directory in `parent`, moved there from its old name if
/// only that one exists (a failed move leaves it where it is, and the
/// port starts afresh under the new name).
fn named(parent: &Path) -> PathBuf {
    let unix = !cfg!(any(target_os = "windows", target_os = "macos"));
    let (name, old) = if unix {
        (NAME_UNIX, OLD_NAME_UNIX)
    } else {
        (NAME, OLD_NAME)
    };
    let dir = parent.join(name);
    let old = parent.join(old);
    if !dir.exists() && old.is_dir() {
        let _ = std::fs::rename(&old, &dir);
    }
    dir
}
