use std::io::{stdin, stdout, IsTerminal, StdinLock, StdoutLock};

use crate::error::Result;

use super::{
    is_raw_mode_enabled, wait_for_stdin, IoProvider, ValueOrMut, WaitForIn,
};

/// Zero size IoProvider with stdin and stdout.
#[derive(Copy, Clone, Default, Debug)]
pub struct StdioProvider();

impl WaitForIn for StdioProvider {
    fn wait_for_in(&self, timeout: std::time::Duration) -> Result<bool> {
        wait_for_stdin(timeout)
    }
}

impl IoProvider for StdioProvider {
    type Out = StdoutLock<'static>;
    type In = StdinLock<'static>;

    fn get_out(&mut self) -> ValueOrMut<'_, Self::Out> {
        ValueOrMut::Value(stdout().lock())
    }

    fn get_in(&mut self) -> ValueOrMut<'_, Self::In> {
        ValueOrMut::Value(stdin().lock())
    }

    fn is_out_terminal(&self) -> bool {
        stdout().is_terminal()
    }

    fn is_in_terminal(&self) -> bool {
        stdin().is_terminal()
    }

    fn is_out_raw(&self) -> bool {
        is_raw_mode_enabled()
    }
}
