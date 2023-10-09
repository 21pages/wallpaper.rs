use crate::{get_stdout, run, Error::UnsupportedDesktop, Mode, Result};

/// Returns the current wallpaper.
pub fn get() -> Result<String> {
    get_stdout(
        "osascript",
        &[
            "-e",
            r#"tell application "System Events" to picture of current desktop"#,
        ],
    )
}

// Sets the wallpaper from a file.
pub fn set_from_path(path: &str) -> Result<()> {
    run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "System Events" to tell every desktop to set picture to {}"#,
                enquote::enquote('"', path),
            ),
        ],
    )
}

/// No-op. Unable to change with AppleScript.
pub fn set_mode(_: Mode) -> Result<()> {
    Err(UnsupportedDesktop)
}

pub fn get_mode(_: Mode) -> Result<()> {
    Err(UnsupportedDesktop)
}
