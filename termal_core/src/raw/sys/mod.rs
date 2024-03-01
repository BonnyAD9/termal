use crate::error::{Error, Result};

#[cfg(unix)]
mod unix;

pub fn enable_raw_mode() -> Result<()> {
    #[cfg(unix)]
    {
        return unix::enable_raw_mode();
    }

    #[allow(unreachable_code)]
    Err(Error::NotSupportedOnPlatform("raw mode"))
}

pub fn disable_raw_mode() -> Result<()> {
    #[cfg(unix)]
    {
        return unix::disable_raw_mode();
    }

    #[allow(unreachable_code)]
    Err(Error::NotSupportedOnPlatform("raw mode"))
}

pub fn is_raw_mode_enabled() -> bool {
    #[cfg(unix)]
    {
        return unix::is_raw_mode_enabled();
    }

    #[allow(unreachable_code)]
    false
}
