//! Library for working with ansi codes to create beutiful terminal outputs.
//!
//! The main focus of this library are the macros [`formatc`], [`printc`],
//! [`printcln`], [`eprintc`] and [`eprintcln`]. They can be used in the same
//! way as you would use the standard rust macros [`format`], [`print`],
//! [`println`], [`eprint`] and [`eprintln`]. In addition the macros in this
//! crate have special syntax for encoding terminal commands.
//!
//! ## The macros
//! For all these macros the following applies:
//!
//! If content braces `{}` starts with `'` (e.g. `"{'command}hello"`) than the
//! content is interpreted by this crate, otherwised it is interpreted by the
//! [`format`] macro.
//!
//! The content can contain one or more commands that will expand directly to a
//! string literal.
//!
//! For most of the color commands the folowing is true:
//! - The commands have short aliases (e.g. `w` is alias for `white`)
//! - Some of the commands can be reset, the resetting command has the same
//!   name but it is prepended with `_` (e.g. use `italic` to set font style to
//!   italic and than use `_italic` to unset the italic font style)
//! - Some commands take arguments and some of the arguments are optional. The
//!   arguments are passed to the command by typing them directly after the
//!   command and separate them by `,` (e.g. `move_to0,0` moves the cursor to
//!   0, 0. This is also the default value so you can just use `move_to,`).
//!
//! ### RGB color commands
//! The colors can be set either by the [color commands](#color-commands) or
//! by hex color:
//!
//! The hex color starts with `#` and can contain either 1, 2, 3 or 6 hex
//! digits. They are interpreted as follows:
//! - **6 digits:** normal 6 digit RGB color (e.g `#FF0000` is pure red)
//! - **3 digits:** 3 digit RGB color, each digit is repeated twice (e.g.
//!   `#ABC` has the same result as `#AABBCC`)
//! - **2 digits:** the two digits are repeated 3 times to form one of 256
//!   shades of gray (e.g. `#AB` has the same result as `#ABABAB`)
//! - **1 digit:** the digit is repeated 6 times to form one of 16 shades of
//!   gray (e.g. `#A` has the same result as `#AAAAAA`)
//!
//! If you want to set the foregorund color, you just type the hex (e.g.
//! `#FF0000` will set the foreground color to pure red). In order to set the
//! background color, you can append `_` to the color (e.g. `#FF0000_` will set
//! the background color to pure red).
//!
//! ### Ascii commands
//! - `bell`: console bell (create sound)
//! - `backspace`: move left by one
//! - `htab`, `tab`: horizontal tabulator
//! - `move_down_scrl`, `mds`: move down by one line scrolling if needed
//! - `newline`, `nl`: move to the start of the next line
//! - `vtab`: vertical tabulator
//! - `carriage_return` | `cr`: move to the start of the current line
//!
//! ### Commands for moving the cursor
//! - `move_to`, `mt`: moves the cursor to the given position, has two
//!   arguments, default values are `0`.
//! - `move_up`, `mu`: moves the cursor up by the given amount, has one
//!   argument, default value is `1`
//! - `move_down`, `md`: moves the cursor down by the given amount, has one
//!   argument, default value is `1`
//! - `move_right`, `mr`: moves the cursor right by the given amount, has one
//!   argument, default value is `1`
//! - `move_left`, `ml`: moves the cursor left by the given amount, has one
//!   argument, default value is `1`
//! - `set_down`, `sd`: moves the cursor to the start of line n lines down, has
//!   one argument, default value is `1`
//! - `set_up`, `su`: moves the cursor to the start of line n lines up, has one
//!   argument, default value is `1`
//! - `move_to_column`, `mc`: moves the cursor to the given x coordinate, has
//!   one argument, default value is `0`
//! + `move_up_scrl`, `mus`: moves the cursor up by one line, scrolling if
//!    needed
//! + `save_cur`, `save`, `s`: saves the current cursor position (single slot,
//!   not stack)
//! + `load_cur`, `load`, `l`: loads the last saved cursor position
//!
//! ### Erase commands
//! - `erase_to_end`, `e_`: erases from the cursor to the end of the screen
//! - `erase_from_start`, `_e`: erases from the start of the screen to the cursor
//! - `erase_screen`, `_e_`: erases the whole screen
//! - `erase_all`, `e`: erases the whole screen and the scroll buffer
//! - `erase_ln_end`, `el_`: erases from the cursor to the end of the line
//! - `erase_ln_start`, `_el`: erases from the start of the line to the cursor
//! - `erase_line`, `erase_ln`, `_el_`, `el`: erases the current line
//!
//! ### Font style and color command
//! + `reset`, `_`: resets all colors and styles
//!
//! ### Font style commands
//! - `bold`: sets style to bold
//! - `faint`, `f`: sets style to faint
//! - `italic`, `i`: sets style to italic
//! - `underline`, `u`: sets style to underline
//! - `blinking`, `blink`: sets style to blinking
//! - `inverse`: sets style to inverse (swap background and foreground)
//! - `invisible`, `invis`: sets the style to invisible (foreground and
//!   background are same)
//! - `striketrough`, `strike`: sets the style to striketrough
//! - `double_underline`, `dunderline`, `dun`: sets the style to double
//!   underline
//! + `_bold`: resets bold and faint
//! + `_italic`, `_i`: resets italic
//! + `_underline`, `_u`: resets underline and double underline
//! + `_blinking`, `_blink`: resets blinking
//! + `_inverse`: resets inverse
//! + `_invisible`, `_invis`: resets invisible
//! + `_striketrough`, `_strike`: resets striketrough
//!
//! ### Color commands
//! - `black_fg`, `black`, `bl`: sets the foreground to black
//! - `white_fg`, `white`, `w`: sets the foreground to white
//! - `gray_fg`, `gray`, `gr`: sets the foreground to green
//! - `bright_gray_fg`, `bgray`, `bgr`: sets the foreground to bright gray
//! + `red_fg`, `red`, `r`: sets the foreground to red
//! + `green_fg`, `green`, `g`: sets the foreground to green
//! + `yellow_fg`, `yellow`, `y`: sets the foreground to yellow
//! + `magenta_fg`, `magenta`, `m`: sets the foreground to magenta
//! + `cyan_fg`, `cyan`, `c`: sets the foreground to cyan
//! - `dark_red_fg`, `dred`, `dr`: sets the foreground to dark red
//! - `dark_green_fg`, `dgreen`, `dg`: sets the foreground to dark green
//! - `dark_yellow_fg`, `dyellow`, `dy`: sets the foreground to dark yellow
//! - `dark_magenta_fg`, `dmagenta`, `dm`: sets the foreground to dark magenta
//! - `dark_cyan_fg`, `dcyan`, `dc`: sets the foreground to dark cyan
//! + `_fg`: resets the foreground color
//! - `black_bg`, `blackb`, `blb`: sets the background to black
//! - `white_bg`, `whiteb`, `wb`: sets the background to white
//! - `gray_bg`, `grayb`, `grb`: sets the background to green
//! - `bright_gray_bg`, `bgrayb`, `bgrb`: sets the background to bright gray
//! + `red_bg`, `redb`, `rb`: sets the background to red
//! + `green_bg`, `greenb`, `gb`: sets the background to green
//! + `yellow_bg`, `yellowb`, `yb`: sets the background to yellow
//! + `magenta_bg`, `magentab`, `mb`: sets the background to magenta
//! + `cyan_bg`, `cyanb`, `cb`: sets the background to cyan
//! - `dark_red_bg`, `dredb`, `drb`: sets the background to dark red
//! - `dark_green_bg`, `dgreenb`, `dgb`: sets the background to dark green
//! - `dark_yellow_bg`, `dyellowb`, `dyb`: sets the background to dark yellow
//! - `dark_magenta_bg`, `dmagentab`, `dmb`: sets the background to dark
//!   magenta
//! - `dark_cyan_bg`, `dcyanb`, `dcb`: sets the background to dark cyan
//! + `_bg`: resets the background
//! - `fg`: sets the foreground color to one of the 256 colors, has one
//!   argument
//! - `bg`: sets the background color to one of the 256 colors, has one
//!   argument
//!
//! ### Other
//! - `line_wrap`, `wrap`: enable line wrapping
//! - `_line_wrap`, `_wrap`: disable line wrapping
//! + `hide_cursor`, `nocur`: hide the cursor
//! + `show_cursor`, `_nocur`: show the cursor
//! + `save_screen`, `sscr`: saves the screen view (single slot, not stack)
//! + `load_screen`, `lscr`: restores the last saved screen view
//! + `alt_buf`, `abuf`: enable alternative buffer
//! + `_alt_buf`, `_abuf`: disable alternative buffer
//!
//! ### Compound
//! - `clear`, `cls`: erases the screen and the buffer and moves the cursor to the
//!   topleft position (equivalent to `e mt,`)
//!
//! ## The uncoloring macros
//! There are also macros that will skip the terminal commands. These can be
//! useful when you need to conditionaly print with colors or without colors.
//!
//! The macros have the same names but they have `n` before the `c` to signify
//! *no color*: [`formatnc`], [`printnc`], [`printncln`], [`eprintnc`] and
//! [`eprintncln`].
//!
//! ## The conditionally coloring macros
//! Theese are same as the normal coloring macros except they take additional
//! first argument that tells whether the output should be colored or not.
//!
//! They have the same names as the uncoloring macros but they have `m` instead
//! of the `n` to signify *maybe color*:[`formatmc`], [`printmc`],
//! [`printmcln`], [`eprintmc`] and [`eprintmcln`].
//!
//! ## Examples
//! ### With macro
//! ```rust
//! use termal::*;
//!
//! // you can use a special macro to inline the color codes, this will write
//! // italic text with yellow foreground and reset at the end.
//! printcln!("{'yellow italic}hello{'reset}");
//!
//! // the macro also supports standard formatting
//! printcln!("{'yellow italic}{}{'reset}", "hello");
//!
//! // you can also use short versions of the codes
//! printcln!("{'y i}{}{'_}", "hello");
//!
//! // you can also use true colors with their hex codes
//! printcln!("{'#dd0 i}{}{'_}", "hello");
//! ```
//!
//! ### Without macro
//! ```rust
//! // Move cursor to position column 5 on line 7 and write 'hello' in italic
//! // yellow
//!
//! use termal::codes::*;
//! use termal::*;
//!
//! println!("{}{YELLOW_FG}{ITALIC}hello{RESET}", move_to!(5, 7));
//! ```
//!
//! ## Other macros
//!
//! The macros such as `move_to!` can accept either literals or dynamic values.
//! Its main feature is that if you supply literals, it expands to a string
//! literal with the ansi code.
//! If you however supply dynamic values it expands to a `format!` macro:
//! ```rust
//! use termal::*;
//!
//! let a = move_to!(5, 7);
//! // expands to:
//! let a = "\x1b[5;7H";
//!
//! let b = move_to!(2 + 3, 7);
//! // expands to:
//! let b = format!("\x1b[{};{}H", 2 + 3, 7);
//! ```
//!
//! If you know the values for the arguments you can also use the `*c` macros:
//! ```rust
//! use termal::formatc;
//!
//! // the spaces, or the lack of them is important
//! let a = formatc!("{'move_to5,7}");
//! ```
//!
//! ### Gradients
//! Youn can create gradients with the function `termal::gradient`:
//! ```rust
//! use termal::*;
//!
//! // This will create foreground gradient from the rgb color `(250, 50, 170)`
//! // to the rgb color `(180, 50, 240)`
//! printcln!("{}{'_}",gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)));
//! ```

pub use termal_core::*;
pub use termal_proc as proc;

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
        $crate::proc::colorize!($l)
    };
    ($l:literal, $($e:expr),+) => {
        $crate::proc::colorize!($l, $($e),+)
    };
}

/// Works as [`println!`], skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// printncln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! printncln {
    ($l:literal) => {
        println!("{}", $crate::proc::uncolor!($l));
    };
    ($l:literal, $($e:expr),+) => {
        println!("{}", $crate::proc::uncolor!($l, $($e),+));
    };
}

/// Works as [`print!`], skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// printnc!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! printnc {
    ($l:literal) => {
        print!("{}", $crate::proc::uncolor!($l));
    };
    ($l:literal, $($e:expr),+) => {
        print!("{}", $crate::proc::uncolor!($l, $($e),+));
    };
}

/// Works as [`eprintln!`], skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// eprintncln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! eprintncln {
    ($l:literal) => {
        eprintln!("{}", $crate::proc::uncolor!($l));
    };
    ($l:literal, $($e:expr),+) => {
        eprintln!("{}", $crate::proc::uncolor!($l, $($e),+));
    };
}

/// Works as [`eprint!`], skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// printnc!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! eprintnc {
    ($l:literal) => {
        eprint!("{}", $crate::proc::uncolor!($l));
    };
    ($l:literal, $($e:expr),+) => {
        eprint!("{}", $crate::proc::uncolor!($l, $($e),+));
    };
}

/// Works as [`format!`], skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Generate 'hello' (not in yellow, the terminal commands are skipped):
/// printcln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! formatnc {
    ($l:literal) => {
        $crate::proc::uncolor!($l)
    };
    ($l:literal, $($e:expr),+) => {
        $crate::proc::uncolor!($l, $($e),+)
    };
}

/// Works as [`println!`], conditionally skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// printncln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! printmcln {
    ($cond:expr, $l:literal) => {
        if $cond {
            println!("{}", $crate::proc::colorize!($l));
        } else {
            println!("{}", $crate::proc::uncolor!($l));
        }
    };
    ($cond:expr, $l:literal, $($e:expr),+) => {
        if $cond {
            println!("{}", $crate::proc::colorize!($l, $($e),+));
        } else {
            println!("{}", $crate::proc::uncolor!($l, $($e),+));
        }
    };
}

/// Works as [`print!`], conditionally skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// printnc!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! printmc {
    ($cond:expr, $l:literal) => {
        if $cond {
            print!("{}", $crate::proc::colorize!($l));
        } else {
            print!("{}", $crate::proc::uncolor!($l));
        }
    };
    ($cond:expr, $l:literal, $($e:expr),+) => {
        if $cond {
            print!("{}", $crate::proc::colorize!($l, $($e),+));
        } else {
            print!("{}", $crate::proc::uncolor!($l, $($e),+));
        }
    };
}

/// Works as [`eprintln!`], conditionally skips terminal commands in
/// `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// eprintncln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! eprintmcln {
    ($cond:expr, $l:literal) => {
        if $cond {
            eprintln!("{}", $crate::proc::colorize!($l));
        } else {
            eprintln!("{}", $crate::proc::uncolor!($l));
        }
    };
    ($cond:expr, $l:literal, $($e:expr),+) => {
        if $cond {
            eprintln!("{}", $crate::proc::colorize!($l, $($e),+));
        } else {
            eprintln!("{}", $crate::proc::uncolor!($l, $($e),+));
        }
    };
}

/// Works as [`eprint!`], conditionally skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Print 'hello' (not in yellow, the terminal commands are skipped):
/// printnc!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! eprintmc {
    ($cond:expr, $l:literal) => {
        if $cond {
            eprint!("{}", $crate::proc::colorize!($l));
        } else {
            eprint!("{}", $crate::proc::uncolor!($l));
        }
    };
    ($cond:expr, $l:literal, $($e:expr),+) => {
        if $cond {
            eprint!("{}", $crate::proc::colorize!($l, $($e),+));
        } else {
            eprint!("{}", $crate::proc::uncolor!($l, $($e),+));
        }
    };
}

/// Works as [`format!`], conditionally skips terminal commands in `"{'...}"`.
///
/// # Examples
/// ```
/// use termal::*;
/// // Generate 'hello' (not in yellow, the terminal commands are skipped):
/// printcln!("{'yellow}hello{'reset}");
/// ```
#[macro_export]
macro_rules! formatmc {
    ($cond:expr, $l:literal) => {
        if $cond {
            $crate::proc::colorize!($l)
        } else {
            $crate::proc::uncolor!($l)
        }
    };
    ($cond:expr, $l:literal, $($e:expr),+) => {
        if $cond {
            $crate::proc::colorize!($l, $($e),+)
        } else {
            $crate::proc::uncolor!($l, $($e),+)
        }
    };
}

#[cfg(test)]
mod tests {
    use std::io::{stdout, Write};

    use super::*;

    #[test]
    fn test_gradient() {
        print!("Expect 'BonnyAD9' as pink to magenta gradient: ");
        printcln!(
            "{}{'_}",
            gradient("BonnyAD9", (250, 50, 170), (180, 50, 240))
        );
        _ = stdout().flush();
    }

    #[test]
    fn test_printcln_println() {
        let s = "Hello";
        let num = 4;
        print!("Expect 'Hello 4' in yellow: ");
        printcln!("{'y}{s} {num}{'_}");
        _ = stdout().flush();
    }

    #[test]
    fn test_formatnc() {
        let s = "Hello";
        let num = 4;
        let r = formatnc!("{'y}{s} {num}{'_}");
        assert_eq!(r, "Hello 4");
    }

    #[test]
    fn test_m() {
        let s = "Hello";
        let num = 4;
        assert_eq!(
            formatmc!(true, "{'y}{s} {num}{'_}"),
            formatc!("{'y}{s} {num}{'_}")
        );
        assert_eq!(
            formatmc!(false, "{'y}{s} {num}{'_}"),
            formatnc!("{'y}{s} {num}{'_}")
        );
    }

    #[test]
    fn try_test() {
        raw::events::AmbigousEvent::from_code(b"\x1b[?1;2c");
    }
}
