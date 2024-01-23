//! Library for working with ansi codes to create beutiful terminal outputs
pub use termal_core::codes;
pub use termal_proc as proc;

use termal_core as core;

/// Works as [`println!`], in addition can generate ansi escape codes.
/// To generate the ansi codes use `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' in yellow:
/// printcln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! printcln {
    ($l:literal) => {
        println!("{}", $crate::proc::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        println!("{}", $crate::proc::colorize!($l, $($e),+));
    };
}

/// Works as [`print!`], in addition can generate ansi escape codes.
/// To generate the ansi codes use `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' in yellow:
/// printc!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! printc {
    ($l:literal) => {
        print!("{}", $crate::proc::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        print!("{}", $crate::proc::colorize!($l, $($e),+));
    };
}

/// Works as [`eprintln!`], in addition can generate ansi escape codes.
/// To generate the ansi codes use `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' in yellow:
/// eprintcln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! eprintcln {
    ($l:literal) => {
        eprintln!("{}", $crate::proc::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        eprintln!("{}", $crate::proc::colorize!($l, $($e),+));
    };
}

/// Works as [`eprint!`], in addition can generate ansi escape codes.
/// To generate the ansi codes use `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' in yellow:
/// eprintc!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! eprintc {
    ($l:literal) => {
        eprint!("{}", $crate::proc::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        eprint!("{}", $crate::proc::colorize!($l, $($e),+));
    };
}

/// Works as [`format!`], in addition can generate ansi escape codes.
/// To generate the ansi codes use `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Generate 'hello' in yellow:
/// formatc!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! formatc {
    ($l:literal) => {
        $crate::proc::colorize!($l);
    };
    ($l:literal, $($e:expr),+) => {
        $crate::proc::colorize!($l, $($e),+);
    };
}

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
        res.push_str(&core::fg!(
            start.0 as f32 + step.0 * i as f32,
            start.1 as f32 + step.1 * i as f32,
            start.2 as f32 + step.2 * i as f32
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient() {
        print!("Expect 'BonnyAD9' as pink to magenta gradient: ");
        printcln!(
            "{}{'_}",
            gradient("BonnyAD9", (250, 50, 170), (180, 50, 240))
        );
    }

    #[test]
    fn test_printcln_println() {
        let s = "Hello";
        let num = 4;
        print!("Expect 'Hello 4' in yellow: ");
        printcln!("{'y}{s} {num}{'_}");
    }
}
