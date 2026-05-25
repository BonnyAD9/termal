use std::{
    io::{Stdin, StdinLock},
    time::Duration,
};

use crate::{
    Result,
    raw::{SysEvent, wait_for_event},
};

/// The type can wait for system event with the given timeout.
pub trait WaitForEvent {
    /// Waits for input with the given timeout.
    ///
    /// - [`Duration::ZERO`] means no blocking.
    /// - [`Duration::MAX`] means wait indefinitely.
    fn wait_for_event(&self, timeout: Duration) -> Result<SysEvent>;
}

impl WaitForEvent for Stdin {
    /// Wait for any event on stdin or other supported event, but not longer than
    /// the timeout.
    ///
    /// If timeout is [`Duration::MAX`], this will wait indefinitely.
    ///
    /// # Returns
    /// The kind of event detected or `SysEvent::Timeout` if timeout was reached.
    ///
    /// # Support
    /// - Unix (Linux)
    ///     - `SysEvent::Stdin`
    ///     - `SysEvent::Resize`
    /// - Windows (not tested)
    ///     - `SysEvent::Stdin`
    ///
    /// # Errors
    /// - [`Error::NotSupportedOnPlatform`] on unsupported platforms.
    /// - [`Error::Io`] on io error.
    /// - [`Error::WaitAbandoned`] when unexpected state happens. See error
    ///   description.
    /// - [`Error::IntConvert`] when timeout value is too large (but not
    ///   [`Duration::MAX`])
    fn wait_for_event(&self, timeout: Duration) -> Result<SysEvent> {
        wait_for_event(timeout)
    }
}

impl WaitForEvent for StdinLock<'static> {
    /// Wait for any event on stdin or other supported event, but not longer than
    /// the timeout.
    ///
    /// If timeout is [`Duration::MAX`], this will wait indefinitely.
    ///
    /// # Returns
    /// The kind of event detected or `SysEvent::Timeout` if timeout was reached.
    ///
    /// # Support
    /// - Unix (Linux)
    ///     - `SysEvent::Stdin`
    ///     - `SysEvent::Resize`
    /// - Windows (not tested)
    ///     - `SysEvent::Stdin`
    ///
    /// # Errors
    /// - [`Error::NotSupportedOnPlatform`] on unsupported platforms.
    /// - [`Error::Io`] on io error.
    /// - [`Error::WaitAbandoned`] when unexpected state happens. See error
    ///   description.
    /// - [`Error::IntConvert`] when timeout value is too large (but not
    ///   [`Duration::MAX`])
    fn wait_for_event(&self, timeout: Duration) -> Result<SysEvent> {
        wait_for_event(timeout)
    }
}
