use std::time::Duration;

use crate::{Error, Result};

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

/// Size of terminal.
#[derive(Clone, Debug)]
pub struct TermSize {
    /// Width in characters.
    pub char_width: usize,
    /// Height in charaters.
    pub char_height: usize,
    /// Width in pixels.
    pub pixel_width: usize,
    /// Height in pixels.
    pub pixel_height: usize,
}

/// Enables raw terminal.
///
/// # Support
/// - Unix (Linux)
pub fn enable_raw_mode() -> Result<()> {
    #[cfg(unix)]
    return unix::enable_raw_mode();

    #[cfg(windows)]
    return windows::enable_raw_mode();

    #[allow(unreachable_code)]
    Err(Error::NotSupportedOnPlatform("raw mode"))
}

/// Disables raw terminal.
///
/// # Support
/// - Unix (Linux)
/// - Windows (not tested)
pub fn disable_raw_mode() -> Result<()> {
    #[cfg(unix)]
    return unix::disable_raw_mode();

    #[cfg(windows)]
    return windows::disable_raw_mode();

    #[allow(unreachable_code)]
    Err(Error::NotSupportedOnPlatform("raw mode"))
}

/// Checks if raw mode is enabled.
///
/// # Support
/// - Unix (Linux)
/// - Windows (not tested)
pub fn is_raw_mode_enabled() -> bool {
    #[cfg(unix)]
    return unix::is_raw_mode_enabled();

    #[cfg(windows)]
    return windows::is_raw_mode_enabled().unwrap_or_default();

    #[allow(unreachable_code)]
    false
}

/// Gets the terminal size.
///
/// # Support
/// - Unix (Linux)
/// - Windows (not tested)
pub fn term_size() -> Result<TermSize> {
    #[cfg(unix)]
    return unix::window_size();

    #[cfg(windows)]
    return windows::term_size();

    #[allow(unreachable_code)]
    Err(Error::NotSupportedOnPlatform("terminal size"))
}

/// Wait for any event on stdin, but not longer than the timeout.
///
/// # Returns
/// `true` if there is event on stdin. If this returns due to timeout or
/// interrupt, returns `false`.
///
/// # Support
/// - Unix (Linux)
/// - Windows (not tested)
pub fn wait_for_stdin(timeout: Duration) -> Result<bool> {
    #[cfg(unix)]
    return unix::wait_for_stdin(timeout);

    #[cfg(windows)]
    return windows::wait_for_stdin(timeout);

    #[allow(unreachable_code)]
    {
        _ = timeout;
        Err(Error::NotSupportedOnPlatform("stdin timeout"))
    }
}
