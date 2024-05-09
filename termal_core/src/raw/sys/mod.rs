use crate::error::{Error, Result};

#[cfg(unix)]
mod unix;

#[derive(Clone, Debug)]
pub struct TermSize {
    pub char_width: usize,
    pub char_height: usize,
    pub pixel_width: usize,
    pub pixel_height: usize,
}

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

pub fn term_size() -> Result<TermSize> {
    #[cfg(unix)]
    {
        return unix::window_size();
    }

    #[allow(unreachable_code)]
    Err(Error::NotSupportedOnPlatform("terminal size"))
}
