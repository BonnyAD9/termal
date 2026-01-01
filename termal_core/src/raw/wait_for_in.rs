use std::{
    io::{Stdin, StdinLock},
    time::Duration,
};

use crate::Result;

use super::wait_for_stdin;

/// The type can wait for input with the given timeout.
pub trait WaitForIn {
    /// Waits for input with the given timeout.
    ///
    /// - [`Duration::ZERO`] means no blocking.
    /// - [`Duration::MAX`] means wait indefinitely.
    fn wait_for_in(&self, timeout: Duration) -> Result<bool>;
}

impl WaitForIn for Stdin {
    /// Wait for any event on stdin, but not longer than the timeout.
    ///
    /// Calls [`wait_for_stdin`].
    ///
    /// If timeout is [`Duration::MAX`], this will wait indefinitely.
    ///
    /// # Returns
    /// `true` if there is event on stdin. If this returns due to timeout or
    /// interrupt, returns `false`.
    ///
    /// # Support
    /// - Unix (Linux)
    /// - Windows (not tested)
    fn wait_for_in(&self, timeout: Duration) -> Result<bool> {
        wait_for_stdin(timeout)
    }
}

impl WaitForIn for StdinLock<'static> {
    /// Wait for any event on stdin, but not longer than the timeout.
    ///
    /// Calls [`wait_for_stdin`].
    ///
    /// If timeout is [`Duration::MAX`], this will wait indefinitely.
    ///
    /// # Returns
    /// `true` if there is event on stdin. If this returns due to timeout or
    /// interrupt, returns `false`.
    ///
    /// # Support
    /// - Unix (Linux)
    /// - Windows (not tested)
    fn wait_for_in(&self, timeout: Duration) -> Result<bool> {
        wait_for_stdin(timeout)
    }
}
