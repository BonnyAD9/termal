///! Library for working with ansi codes to create beutiful terminal outputs
pub use ansi_codes as codes;

pub use termal_macros;

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
        println!("{}", $crate::termal_macros::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        println!("{}", $crate::termal_macros::colorize!($l, $($e),+));
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
        print!("{}", $crate::termal_macros::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        print!("{}", $crate::termal_macros::colorize!($l, $($e),+));
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
        $crate::termal_macros::colorize!($l);
    };
    ($l:literal, $($e:expr),+) => {
        $crate::termal_macros::colorize!($l, $($e),+);
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
        res.push_str(&codes::fg!(
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
    fn it_works() {
        printcln!(
            "{}{'_}",
            gradient("BonnyAD9", (250, 50, 170), (180, 50, 240))
        );
    }
}
