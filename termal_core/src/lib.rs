//! Core library of termal, contains the implementation.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod rgb;

use std::{
    io::{self, Write},
    panic,
};

pub use self::{err::*, rgb::*};

pub mod codes;
mod err;
#[cfg(feature = "term_image")]
pub mod image;
#[cfg(feature = "proc")]
pub mod proc;
#[cfg(feature = "raw")]
pub mod raw;
#[cfg(feature = "term_text")]
pub mod term_text;

#[deprecated(note = "use directly `termal::Error` or `termal::Result`.")]
pub mod error {
    pub use crate::err::*;
}

/// Appends linear gradient to the given string.
///
/// The gradient consists of characters given by `s`. `s_len` is the length of
/// `s` in characters. The result is written to `res`. `start` and `end` are
/// the starting and ending colors of the gradient.
///
/// If you don't know the length of the string and you would like to allocate
/// new string for the resulting gradient, consider using [`gradient`].
///
/// # Example
/// ```no_run
/// use termal_core::{codes, write_gradient};
///
/// let mut buf = codes::CLEAR.to_string();
///
/// let text = "gradient";
/// write_gradient(
///     &mut buf,
///     text,
///     text.len(),
///     (0xFD, 0xB9, 0x75),
///     (0x57, 0x9B, 0xDF)
/// );
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/write_gradient.png)
pub fn write_gradient(
    res: &mut String,
    s: impl AsRef<str>,
    s_len: usize,
    start: impl Into<Rgb>,
    end: impl Into<Rgb>,
) {
    let len = s_len as f32 - 1.;
    let start = start.into().cast::<f32>();
    let end = end.into().cast::<f32>();

    let step = if s_len == 1 {
        Rgb::<f32>::ZERO
    } else {
        (end - start) / len
    };

    for (i, c) in s.as_ref().chars().take(s_len).enumerate() {
        let col = (start + step * i as f32).cast::<u8>();
        res.push_str(&fg!(col.r(), col.g(), col.b()));
        res.push(c);
    }
}

/// Generates linear color gradient with the given text
pub fn gradient(
    s: impl AsRef<str>,
    start: impl Into<Rgb>,
    end: impl Into<Rgb>,
) -> String {
    let mut res = String::new();
    let len = s.as_ref().chars().count();
    write_gradient(&mut res, s, len, start, end);
    res
}

/// Resets terminal modes. This should in most cases restore terminal to state
/// before your app started. Useful for example in case of panic.
///
/// The reset works on best-effort bases - it may not be fully reliable in all
/// cases, but it should work in most cases as long as you use this crate to
/// enable the terminal features.
pub fn reset_terminal() {
    #[cfg(feature = "raw")]
    if raw::is_raw_mode_enabled() {
        _ = raw::disable_raw_mode();
    }
    let s = [
        codes::RESET,
        codes::ENABLE_LINE_WRAP,
        codes::SHOW_CURSOR,
        codes::DISABLE_MOUSE_XY_UTF8_EXT,
        codes::DISABLE_MOUSE_XY_EXT,
        codes::DISABLE_MOUSE_XY_URXVT_EXT,
        codes::DISABLE_MOUSE_XY_PIX_EXT,
        codes::DISABLE_MOUSE_XY_TRACKING,
        codes::DISABLE_MOUSE_XY_PR_TRACKING,
        codes::DISABLE_MOUSE_XY_DRAG_TRACKING,
        codes::DISABLE_MOUSE_XY_ALL_TRACKING,
        codes::DISABLE_FOCUS_EVENT,
        codes::CUR_SAVE,
        codes::RESET_SCROLL_REGION,
        codes::CUR_LOAD,
        codes::DISABLE_ALTERNATIVE_BUFFER,
        codes::DISABLE_REVERSE_COLOR,
        codes::DISABLE_BRACKETED_PASTE_MODE,
        codes::RESET_ALL_COLOR_CODES,
        codes::RESET_DEFAULT_FG_COLOR,
        codes::RESET_DEFAULT_BG_COLOR,
        codes::RESET_CURSOR_COLOR,
    ]
    .concat();
    print!("{}", s);
    _ = io::stdout().flush();
}

/// Registers panic hook that will prepend terminal reset before the current
/// panic hook. Useful for tui apps.
///
/// This will make sure that the terminal is set to reasonable state even when
/// your app panics.
pub fn register_reset_on_panic() {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |pci| {
        reset_terminal();
        hook(pci)
    }));
}
