///! Library for working with ansi codes to create beutiful terminal outputs
pub use ansi_codes;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        printcln!("hello{'ml4 y}{}{'_ mr i} there{'_}","ell");
    }
}
