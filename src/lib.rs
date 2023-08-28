///! Library for working with ansi codes to create beutiful terminal outputs
pub use ansi_codes;

pub use termal_macros;

#[macro_export]
macro_rules! printcln {
    ($l:literal) => {
        println!("{}", $crate::termal_macros::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        println!("{}", $crate::termal_macros::colorize!($l, $($e),+));
    };
}

#[macro_export]
macro_rules! printc {
    ($l:literal) => {
        print!("{}", $crate::termal_macros::colorize!($l));
    };
    ($l:literal, $($e:expr),+) => {
        print!("{}", $crate::termal_macros::colorize!($l, $($e),+));
    };
}

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
        printcln!("{'red}{}{'reset}","hello");
    }
}
