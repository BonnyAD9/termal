//! Core library of termal, contains the implementation.

use std::{
    io::{self, Write},
    panic,
};

pub mod codes;
pub mod error;
#[cfg(feature = "term_image")]
pub mod image;
#[cfg(feature = "proc")]
pub mod proc;
#[cfg(feature = "raw")]
pub mod raw;
#[cfg(feature = "term_text")]
pub mod term_text;

/// Appends linear gradient to the given string
pub fn write_gradient(
    res: &mut String,
    s: impl AsRef<str>,
    s_len: usize,
    start: (u8, u8, u8),
    end: (u8, u8, u8),
) {
    let len = s_len as f32 - 1.;

    let step = if s_len == 1 {
        (0., 0., 0.)
    } else {
        (
            (end.0 as f32 - start.0 as f32) / len,
            (end.1 as f32 - start.1 as f32) / len,
            (end.2 as f32 - start.2 as f32) / len,
        )
    };

    for (i, c) in s.as_ref().chars().take(s_len).enumerate() {
        res.push_str(&fg!(
            start.0 as f32 + step.0 * i as f32,
            start.1 as f32 + step.1 * i as f32,
            start.2 as f32 + step.2 * i as f32,
        ));
        res.push(c);
    }
}

/// Generates linear color gradient with the given text
pub fn gradient(
    s: impl AsRef<str>,
    start: (u8, u8, u8),
    end: (u8, u8, u8),
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
