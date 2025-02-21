mod predicate;
mod read_conf;
mod term_read;
mod vec2;

use std::io::{self, Write};

use crate::error::Result;

pub(crate) use self::vec2::*;

pub use self::{predicate::*, read_conf::*, term_read::*};

/// Read one line from standard input. This will use custom readline if
/// supported. Otherwise it will fallback to the default readline function.
pub fn read_line() -> Result<String> {
    prompt("")
}

/// Read one line from standard input. This will use custom readline if
/// supported. Otherwise it will fallback to the default readline function.
pub fn read_line_to(res: &mut String) -> Result<()> {
    prompt_to(res, "")
}

/// Prompt the user with the given prompt. This will use custom readline if
/// supported. Otherwise it will fallback to the default readline function.
pub fn prompt(prompt: impl AsRef<str>) -> Result<String> {
    let mut res = String::new();
    prompt_to(&mut res, prompt)?;
    Ok(res)
}

/// Prompt the user with better read line capabilities.
#[cfg(any(windows, unix))]
pub fn prompt_to(res: &mut String, prompt: impl AsRef<str>) -> Result<()> {
    prompt_to_inner(res, prompt.as_ref())
        .or_else(|_| prompt_to_fallback(res, prompt))
}

/// Prompt the user. Better readline is not supported on this platform, so this
/// will just fallback to default readline.
#[cfg(not(any(windows, unix)))]
pub fn prompt_to(res: &mut String, prompt: impl AsRef<str>) -> Result<()> {
    prompt_to_fallback(res, prompt)
}

#[cfg(any(windows, unix))]
fn prompt_to_inner(res: &mut String, prompt: impl AsRef<str>) -> Result<()> {
    use super::{
        Terminal, disable_raw_mode, enable_raw_mode, is_raw_mode_enabled,
    };

    let raw = is_raw_mode_enabled();
    if !raw {
        enable_raw_mode()?;
    }

    let mut i = prompt.as_ref().split('\n').peekable();
    let mut prompt = "";
    while let Some(s) = i.next() {
        if i.peek().is_none() {
            prompt = s;
            break;
        }
        println!("{s}\r");
    }

    let r = Terminal::stdio().prompt_to(res, prompt);

    if !raw {
        _ = disable_raw_mode();
    }

    r
}

fn prompt_to_fallback(
    res: &mut String,
    prompt: impl AsRef<str>,
) -> Result<()> {
    print!("{}", prompt.as_ref());
    io::stdout().flush()?;
    io::stdin().read_line(res)?;
    if res.ends_with('\n') {
        res.pop();
    }
    Ok(())
}
