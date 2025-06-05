use std::{
    io::{Stdin, StdinLock},
    time::Duration,
};

use crate::Result;

use super::wait_for_stdin;

/// The type can wait for input with the given timeout.
pub trait WaitForIn {
    fn wait_for_in(&self, timeout: Duration) -> Result<bool>;
}

impl WaitForIn for Stdin {
    fn wait_for_in(&self, timeout: Duration) -> Result<bool> {
        wait_for_stdin(timeout)
    }
}

impl WaitForIn for StdinLock<'static> {
    fn wait_for_in(&self, timeout: Duration) -> Result<bool> {
        wait_for_stdin(timeout)
    }
}
