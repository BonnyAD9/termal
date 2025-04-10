//! Module with ansi escape codes.
//!
//! Most of them are taken from:
//! <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797>
//!
//! There are several types of codes here:
//! - **Sequences:** string/char used to introduce escape sequence, most of the
//!   other codes use them
//! - **General ascii codes:** single char sequences some of them have escape
//!   codes in rust string/char literals (such as '\n')
//! - **Macro codes:** these escape codes have one or more parameters. Here
//!   they are in form of a macro that takes the parameters. If the macro is
//!   invoked with literals, it expands to `&'static str`. If the arguments
//!   are not literals it expands to a call to the `format!` macro. Because
//!   these codes may expand either to `&'static str` or `String` you can use
//!   the [`GetString::get_string`] method to get `String`, or you can use
//!   `AsRef<str>::as_ref` method to get `&str`, or you can use
//!   `Into<Cow<'static, str>>::into` to get the possibly owned string.
//! - **String codes:** these codes are just strings that can be just printed
//!   to terminal to do what they say they do. This is the majority of the
//!   codes.

use std::fmt::Display;

use base64::Engine;
use place_macro::place;

/// Creates the given sequence, this is used internally.
#[macro_export]
macro_rules! seq {
    ($sq:literal, $i:literal) => {
        concat!($sq, $i)
    };
    ($sq:literal, $i:literal, $f:literal, $($a:literal),*) => {
        concat!($sq, $f $(, ';', $a)*, $i)
    };
    ($sq:literal, $i:literal, $f:expr $(,$a:expr)*) => {
        $crate::seq!($sq, $i, $f, $(";{}"; $a),*)
    };
    ($sq:literal, $i:literal, $f:expr, $($l:literal; $e:expr),*) => {
        format!(concat!($sq, "{}" $(,$l)*, "{}"), $f $(,$e)*, $i)
    }
}

// Sequences:

/// The escape character
pub const ESC: char = '\x1b';
/// Control Sequence Introducer: Start of CSI sequence
pub const CSI: &str = "\x1b[";
/// Device Control String: Start of DCS sequence
pub const DCS: &str = "\x1bP";
/// Operating System Command: Start of OSC sequence
pub const OSC: &str = "\x1b]";
/// String terminator. Terminates for example DCS.
pub const ST: &str = "\x1b\\";
/// Single shift three
pub const SS3: &str = "\x1bO";

/// Creates control escape sequence, the first literal is the end of the
/// sequence, the other arguments are the values in the sequence
#[macro_export]
macro_rules! csi {
    ($i:literal $(,$a:expr)* $(,)?) => {
        $crate::seq!("\x1b[", $i $(, $a)*)
    };
}

/// Creates control escape sequence for graphic mode.
#[macro_export]
macro_rules! graphic {
    ($($a:expr),* $(,)?) => {
        $crate::csi!('m' $(, $a)*)
    };
}

/// Creates operating system command sequence. The arguments are the values in
/// the sequence.
#[macro_export]
macro_rules! osc {
    ($($a:expr),+) => {
        $crate::seq!("\x1b]", "\x1b\\", $($a),+)
    };
}

/// Enables the given private terminal mode.
#[macro_export]
macro_rules! enable {
    ($a:expr) => {
        $crate::seq!("\x1b[?", 'h', $a)
    };
}

/// Disables the given private terminal mode.
#[macro_export]
macro_rules! disable {
    ($a:expr) => {
        $crate::seq!("\x1b[?", 'l', $a)
    };
}

// General ASCII codes

/// Produces terminal bell (audio or visual).
pub const BELL: char = '\x07';
/// Moves the cursor left by one positoin.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = String::new();
///
/// buf += "Some test";
/// buf.push(codes::BACKSPACE);
/// buf.push(codes::BACKSPACE);
/// buf += "x";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/backspace.png)
pub const BACKSPACE: char = '\x08';
/// Horizontal tabulator, moves cursor to the next stop. Same as `\t`
///
/// # Example
/// ```no_run
/// println!("1\t: number");
/// println!("hello\t: greeting");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/htab.png)
pub const HTAB: char = '\t';
/// Moves the cursor to the start of the next line. Same as `\n`.
///
/// Nothe that in raw terminal, this will move cursor down and not to the start
/// of the line.
///
/// # Example
/// ```no_run
/// use termal_core::{raw::enable_raw_mode, reset_terminal};
///
/// println!("normal:");
/// println!("one\ntwo");
///
/// println!("raw:");
/// enable_raw_mode()?;
/// println!("one\ntwo\r");
///
/// reset_terminal();
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/newline.png)
pub const NEWLINE: char = '\n';
/// Vertical tabulator, moves the cursor to the next vertical stop.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = String::new();
///
/// buf += "hello";
/// buf.push(codes::VTAB);
/// buf += "there";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/vtab.png)
pub const VTAB: char = '\x0b';
/// Indicates new page, usualy has no use in terminal.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = String::new();
///
/// buf += "hello";
/// buf.push(codes::FORMFEED);
/// buf += "there";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/vtab.png)
pub const FORMFEED: char = '\x0c';
/// Moves cursor to the start of the line. Same as `\r`.
///
/// # Example
/// ```no_run
/// println!("hello me\rgreet");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/carriage_return.png)
pub const CARRIAGE_RETURN: char = '\r';
/// Does nothing.
pub const DELETE: char = '\x7f';

// Cursor controls

// For the macros is true that:
// If you use literals it returns `&str`,
// if you use expressions, it returns [`String`]. You can use the
// `.get_string()` method from the trait [`GetString`] to get [`String`] in
// both cases

macro_rules! code_macro {
    ($code:ident $(
        $name:ident
        $(, $($nam:ident)? $($lit:literal)?)+ ;
        $($i:literal)? $(?$doc:literal)?),+ $(,)?
    ) => {
        place! {$(
            $(#[doc = $doc])?
            #[macro_export]
            macro_rules! $name {
                (__start__($($(__s__ $nam:expr,)?)+) __s__ (,)?) => {
                    __s__ crate::$code!($($i,)? $($(__s__ $nam)? $($lit)?),+)
                }
            }
            pub use $name;
        )+}
    };
    ($code:ident != $ex:literal => $(
        $name:ident,
        $nam:ident;
        $($i:literal)? $(?$doc:literal)?),+ $(,)?
    ) => {
        place! {$(
            $(#[doc = $doc])?
            #[macro_export]
            macro_rules! $name {
                (__start__(__s__ $nam:literal,)) => {{
                    if __s__ $nam == $ex {
                        "".into()
                    } else {
                        __s__ crate::$code!($($i,)? __s__ $nam)
                    }
                }};
                (__start__(__s__ $nam:expr,)) => {{
                    let v = __s__ $nam;
                    if v == $ex {
                        "".into()
                    } else {
                        __s__ crate::$code!($($i,)? v)
                    }
                }}
            }
            pub use $name;
        )+}
    };
}

/// Moves cursor to the given position. Position of the top left conrner is
/// (1, 1).
///
/// If used with literals, produces `&'static str`, otherwise produces
/// [`String`].
///
/// # Example
/// ```no_run
/// use termal_core::{raw::term_size, codes};
///
/// let mut buf = String::new();
/// buf += codes::ERASE_ALL;
///
/// let txt = "centered";
/// let size = term_size()?;
/// let x = (size.char_width - txt.len() + 1) / 2;
/// let y = size.char_height / 2;
/// // If one of arguments is not literal, produces string.
/// let center: String = codes::move_to!(x, y);
/// buf += &center;
/// buf += txt;
///
/// // With literals, it constructs static slice.
/// let home: &'static str = codes::move_to!(1, 1);
/// buf += home;
/// buf += "top left";
///
/// // Move to the second to last line from bottom.
/// buf += &codes::move_to!(0, size.char_height - 1);
///
/// println!("{}", buf);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/move_to.png)
#[macro_export]
macro_rules! move_to {
    ($x:expr, $y:expr) => {
        $crate::csi!('H', $y, $x)
    };
}

pub use move_to;

use crate::Rgb;

code_macro!(csi != 0 =>
    move_up, n; 'A'
        ? "Moves cursor up by N positions.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```ignore
assert_eq!(formatc!(\"{'mu5}\"), codes::move_up!(5));
assert_eq!(formatc!(\"{'md5}\"), codes::move_down!(5));
assert_eq!(formatc!(\"{'mu}\"), codes::move_up!(1));
assert_eq!(formatc!(\"{'md}\"), codes::move_down!(1));

printcln!(\"{'clear}\\n\\nhello{'mu2}up{'md}down{'md}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/move_up_down.png)
        ",

    move_down, n; 'B'
        ? "Moves cursor down by N positions.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```ignore
assert_eq!(formatc!(\"{'mu5}\"), codes::move_up!(5));
assert_eq!(formatc!(\"{'md5}\"), codes::move_down!(5));
assert_eq!(formatc!(\"{'mu}\"), codes::move_up!(1));
assert_eq!(formatc!(\"{'md}\"), codes::move_down!(1));

printcln!(\"{'clear}\\n\\nhello{'mu2}up{'md}down{'md}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/move_up_down.png)
        ",

    move_right, n; 'C'
        ? "Moves cursor right by N positions.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```ignore
assert_eq!(formatc!(\"{'mr5}\"), codes::move_right!(5));
assert_eq!(formatc!(\"{'ml5}\"), codes::move_left!(5));

printcln!(\"{'clear}{'mr7}there{'ml11}hello\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/move_right_left.png)
        ",

    move_left, n; 'D'
        ? "Moves cursor left by N positions.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```ignore
assert_eq!(formatc!(\"{'mr5}\"), codes::move_right!(5));
assert_eq!(formatc!(\"{'ml5}\"), codes::move_left!(5));

printcln!(\"{'clear}{'mr7}there{'ml11}hello\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/move_right_left.png)
        ",

    insert_lines, n; 'L'
        ? "Insert n lines at the cursor moving them down.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```ignore
let mut buf = formatc!(\"{'clear}\");

buf += \"line 1\\n\";
buf += \"line 2\\n\";
buf += codes::move_up!(1);
buf += codes::insert_lines!(2);
buf += \"inserted 1\\n\";
buf += \"inserted 2\\n\";

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/insert_lines.png)
        ",

    delete_lines, n; 'M'
        ? "Delete n lines at the cursor, moving the remaining from bottom.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```ignore
let mut buf = formatc!(\"{'clear}\");

buf += \"line 1\\n\";
buf += \"line 2\\n\";
buf += \"line 3\\n\";
buf += \"line 4\";
buf += codes::move_up!(2);
buf += codes::delete_lines!(2);

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/delete_lines.png)
        ",

    insert_chars, n; '@'
        ? "Insert n characters, moving them to the right.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```ignore
let mut buf = formatc!(\"{'clear}\");

buf += \"say there\";
buf += codes::move_left!(5);
buf += codes::insert_chars!(6);
buf += \"hello\";

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/insert_chars.png)
        ",

    delete_chars, n; 'P'
        ? "Delete n characters, moving the chars from right.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

buf += \"say hello there\";
buf += codes::move_left!(11);
buf += codes::delete_chars!(6);

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/delete_chars.png)
        ",

    insert_columns, n; "'}"
        ? "Insert n columns, moving them to the right.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

buf += \"say line 1\\n\";
buf += \"say line 2\\n\";
buf += \"say line 3\";
buf += codes::move_left!(6);
buf += codes::insert_columns!(9);
buf += \"hello to \";

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/insert_columns.png)
        ",

    delete_columns, n; "'~"
        ? "Delete n columns, moving them from the right

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

buf += \"say hello to line 1\\n\";
buf += \"say greeting line 2\\n\";
buf += \"say no words line 3\";
buf += codes::move_left!(15);
buf += codes::delete_columns!(9);

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/delete_columns.png)
        ",

    set_down, n; 'E'
        ? "Moves cursor to the start of line N lines down.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

buf += \"line one\";
buf += codes::set_down!(2);
buf += \"line two\";

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/set_down.png)
        ",

    set_up, n; 'F'
        ? "Moves cursor to the start of line N lines up

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

buf += \"\\n\\n\";
buf += \"line one\";
buf += codes::set_up!(2);
buf += \"line two\";
buf += \"\\n\\n\";

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/set_up.png)
        ",

    repeat_char, n; 'b'
        ? "Repeat the previous char n times.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

buf += \"lo\";
buf += codes::repeat_char!(69);
buf += \"ng word\";

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/repeat_char.png)
        "
);

code_macro!(csi
    column, n; 'G'
        ? "Moves cursor to the given column.

If used with literal, produces `&'static str`, otherwise produces [`String`].

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

buf += \"hello\";
buf += codes::column!(20);
buf += \"there\";

println!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/column.png)
        ",
);

/// Moves cursor to the top left of the screen.
///
/// Has the same effect as `move_to!(1, 1)`, but it is a little shorter code.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "\n\nhello below";
/// buf += codes::MOVE_HOME;
/// buf += "home sweet home\n\n";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/move_home.png)
pub const MOVE_HOME: &str = "\x1b[H";

/// Moves cursor one line up, 'scrolling' if needed.
///
/// **THIS MAY NOT DO WHAT YOU EXPECT** read precise description below:
///
/// Moves cursor one line up. If the cursor is already on top of the screen,
/// insert one line at the top of the screen. The line at the bottom of the
/// screen is discarded.
///
/// ```no_run
/// use std::io::Write;
/// use termal_core::{codes, raw::Terminal};
///
/// println!("{}", codes::CLEAR);
///
/// for i in 0..100 {
///     print!("\n{i}");
/// }
///
/// // Move to the second line on screen.
/// let mut buf = codes::MOVE_HOME.to_string();
/// buf += codes::move_down!(1);
/// // Move up, scrolling is not necesary so it is just move up
/// buf += codes::UP_SCRL;
/// // Move up, cursor is already on top of the screen, so empty line is
/// // inserted. Line at the bottom of the screen is discarded.
/// buf += codes::UP_SCRL;
///
/// print!("{buf}");
///
/// _ = Terminal::stdio().flush();
///
/// // Wait for enter. Screenshot is taken before enter is pressed.
/// _ = Terminal::stdio().read();
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/up_scrl.png)
pub const UP_SCRL: &str = "\x1bM";
/// Saves the cursor position (this is single save slot, not stack). Position
/// can be later restored by [`CUR_LOAD`].
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "start";
/// buf += codes::CUR_SAVE;
/// buf += "\ncontinue here";
/// buf += codes::CUR_LOAD;
/// buf += " and end here\n";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/cur_save_load.png)
pub const CUR_SAVE: &str = "\x1b7";
/// Restores the cursor position to the last saved position (this is single
/// save slot, not stack). The position can be saved by [`CUR_SAVE`].
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "start";
/// buf += codes::CUR_SAVE;
/// buf += "\ncontinue here";
/// buf += codes::CUR_LOAD;
/// buf += " and end here\n";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/cur_save_load.png)
pub const CUR_LOAD: &str = "\x1b8";

// Erase codes

/// Erases from the cursor to the end of the screen.
///
/// Note that [`ERASE_TO_END`] and [`ERASE_FROM_START`] are not opposite. Both
/// will also erase character at the cursor position.
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase to the end of the screen.
/// buf += codes::ERASE_TO_END;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_to_end.png)
pub const ERASE_TO_END: &str = csi!('J');
/// Erases from the start of the screen to the cursor.
///
/// Note that [`ERASE_FROM_START`] and [`ERASE_TO_END`] are not opposite. Both
/// will also erase character at the cursor position.
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase to the end of the screen.
/// buf += codes::ERASE_FROM_START;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_from_start.png)
pub const ERASE_FROM_START: &str = csi!('J', 1);
/// Erases the entire screen.
///
/// Doesn't erase the scrollback buffer. If you want to do both, use
/// [`ERASE_ALL`], if you want to erase just the scrollback buffer, use
/// [`ERASE_BUFFER`].
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase the whole screen.
/// buf += codes::ERASE_SCREEN;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_screen.png)
pub const ERASE_SCREEN: &str = csi!('J', 2);
/// Erase the scrollback buffer.
///
/// Doesn't erase the screen, only what is not visible on the screen because it
/// was scrolled. If you wan't to also erase the screen use [`ERASE_ALL`], if
/// you only want to erase the screen use [`ERASE_SCREEN`].
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase the scrollback buffer.
/// buf += codes::ERASE_BUFFER;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// Note that the scrollbar is full - there is nowhere to scroll - even though
/// there was the prompt and cargo compilation log before the program ran.
///
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_buffer.png)
pub const ERASE_BUFFER: &str = csi!('J', 3);
/// Erases from cursor to the end of the line.
///
/// Note that [`ERASE_TO_LN_END`] and [`ERASE_FROM_LN_START`] are not opposite.
/// Both will also erase character at the cursor position.
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase from the cursor to the end of the line.
/// buf += codes::ERASE_TO_LN_END;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_to_ln_end.png)
pub const ERASE_TO_LN_END: &str = csi!('K');
/// Erases from the start of the line to the cursor.
///
/// Note that [`ERASE_FROM_LN_START`] and [`ERASE_TO_LN_END`] are not opposite.
/// Both will also erase character at the cursor position.
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase from start of the line to the cursor.
/// buf += codes::ERASE_FROM_LN_START;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_from_ln_start.png)
pub const ERASE_FROM_LN_START: &str = csi!('K', 1);
/// Erases the entire line.
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase the entire line.
/// buf += codes::ERASE_LINE;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_line.png)
pub const ERASE_LINE: &str = csi!('K', 2);
/// Erases the whole screen and the scrollback buffer.
///
/// It is the same as combination of [`ERASE_SCREEN`] and [`ERASE_BUFFER`].
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase the whole screen and scrollback buffer.
/// buf += codes::ERASE_LINE;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// Note that the scrollbar is full - there is nowhere to scroll - even though
/// there was the prompt and cargo compilation log before the program ran.
///
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/erase_all.png)
pub const ERASE_ALL: &str = "\x1b[2J\x1b[3J";
/// Erases the whole screen and the scrollback buffer and moves cursor to the
/// top left.
///
/// It is the same as combination of [`ERASE_SCREEN`], [`ERASE_BUFFER`] and
/// [`MOVE_HOME`].
///
/// # Example
/// ```no_run
/// use termal_core::{codes, error::Error, raw::{
///     TermSize, Terminal, term_size
/// }};
///
/// // Fill the terminal with `#` and move to the center.
/// let TermSize { char_width: w, char_height: h, .. } = term_size()?;
/// let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
/// buf += &codes::move_to!(w / 2, h / 2);
///
/// // Erase the whole screen and scrollback buffer.
/// buf += codes::CLEAR;
///
/// // Print to the output and wait for enter. Screenshot is taken before enter
/// // is pressed.
/// Terminal::stdio().flushed(buf)?;
/// Terminal::stdio().read()?;
///
/// Ok::<_, Error>(())
/// ```
///
/// ## Result in terminal
/// Note that the scrollbar is full - there is nowhere to scroll - even though
/// there was the prompt and cargo compilation log before the program ran.
///
/// Also note that the cursor is in the top left corner and not in the center.
///
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/clear.png)
pub const CLEAR: &str = "\x1b[2J\x1b[3J\x1b[H";

// Text modes

/// Resets all the text modes (colors and styles).
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// // Produce some crazy style for the text.
/// buf += codes::BOLD;
/// buf += codes::ITALIC;
/// buf += codes::OVERLINE;
/// buf += codes::DOUBLE_UNDERLINE;
/// buf += codes::STRIKETROUGH;
/// buf += codes::BLUE_FG;
/// buf += codes::YELLOW_BG;
/// buf += codes::underline256!(1);
///
/// // Text with crazy style
/// buf += "crazy style";
/// // Reset the text style
/// buf += codes::RESET;
/// // Write text with normal color
/// buf += " normal style";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/reset.png)
pub const RESET: &str = graphic!(0);

/// Set bold text mode (on some terminals may be just brighter color).
///
/// This mode can be reset with [`RESET_BOLD`] or [`RESET`]. Note that
/// [`RESET_BOLD`] will also reset [`FAINT`] and [`RESET`] will reset all text
/// modes.
///
/// In some terminals, [`BOLD`] and [`FAINT`] are exclusive (e.g. konsole), in
/// others they can be combined (e.g. vscode terminal).
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BOLD;
/// buf += "bold text";
///
/// buf += codes::RESET_BOLD;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/bold.png)
pub const BOLD: &str = graphic!(1);
/// Set dim/faint text mode.
///
/// Doesn't affect the background color.
///
/// This mode can be reset with [`RESET_BOLD`] or [`RESET`]. Note that
/// [`RESET_BOLD`] will also reset [`BOLD`] and [`RESET`] will reset all text
/// modes.
///
/// In some terminals, this triggers new set of colors (third color variant)
/// and it doesn't work for 256 or true RGB colors (e.g. konsole). In other
/// terminals, the dim color is calculated from the current color and so it
/// also works for 256 and true RGB colors (e.g. vscode terminal).
///
/// In some terminals, [`FAINT`] and [`BOLD`] are exclusive (e.g. konsole), in
/// others they can be combined (e.g. vscode terminal).
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// let cols = [
///     "", // default text color
///     codes::GRAY_FG,
///     codes::WHITE_FG,
///     codes::RED_FG,
///     codes::GREEN_FG,
///     codes::YELLOW_FG,
///     codes::BLUE_FG,
///     codes::MAGENTA_FG,
///     codes::CYAN_FG,
/// ];
///
/// for c in cols {
///     buf += c;
///     buf += codes::FAINT;
///     buf += "faint text";
///     buf += codes::RESET_BOLD;
///     buf += " normal text\n";
/// }
///
/// buf.pop(); // remove the last newline
/// buf += codes::RESET_FG;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/faint.png)
pub const FAINT: &str = graphic!(2);
/// Set italic mode.
///
/// This mode can be reset with [`RESET_ITALIC`] or [`RESET`]. Note that
/// [`RESET`] will reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::ITALIC;
/// buf += "italic text";
///
/// buf += codes::RESET_ITALIC;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/italic.png)
pub const ITALIC: &str = graphic!(3);
/// Set underline mode.
///
/// This mode can be reset with [`RESET_UNDERLINE`] or [`RESET`]. Note that
/// [`RESET_UNDERLINE`] will also reset [`DOUBLE_UNDERLINE`] and [`RESET`] will
/// reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::UNDERLINE;
/// buf += "underline text";
///
/// buf += codes::RESET_UNDERLINE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/underline.png)
pub const UNDERLINE: &str = graphic!(4);
/// Set blinking mode.
///
/// Doesn't affect background color (only foreground).
///
/// This mode can be reset with [`RESET_BLINKING`] or [`RESET`]. Note that
/// [`RESET`] will reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BLINKING;
/// buf += "blinking text";
///
/// buf += codes::RESET_BLINKING;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/blinking.gif)
pub const BLINKING: &str = graphic!(5);
/// Set inverse mode (inverse foreground and background).
///
/// This mode can be reset with [`RESET_INVERSE`] or [`RESET`]. Note that
/// [`RESET`] will reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::INVERSE;
/// buf += "inverse text";
///
/// buf += codes::RESET_INVERSE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/inverse.png)
pub const INVERSE: &str = graphic!(7);
/// Set invisible mode.
///
/// This mode can be reset with [`RESET_INVISIBLE`] or [`RESET`]. Note that
/// [`RESET`] will reset all text modes.
///
/// Some terminals just set the foreground color to the background color. This
/// means that the text may actually be visible if the background color is not
/// solid. Other terminals will just not show the text (e.g. konsole).
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::INVISIBLE;
/// buf += "invisible text";
///
/// buf += codes::RESET_INVISIBLE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/invisible.png)
pub const INVISIBLE: &str = graphic!(8);
/// Set striketrough mode.
///
/// This mode can be reset with [`RESET_STRIKETROUGH`] or [`RESET`]. Note that
/// [`RESET`] will reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::STRIKETROUGH;
/// buf += "striketrough text";
///
/// buf += codes::RESET_STRIKETROUGH;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/striketrough.png)
pub const STRIKETROUGH: &str = graphic!(9);
/// Set double underline mode.
///
/// This mode can be reset with [`RESET_UNDERLINE`] or [`RESET`]. Note that
/// [`RESET_UNDERLINE`] will also reset [`UNDERLINE`] and [`RESET`] will reset
/// all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::DOUBLE_UNDERLINE;
/// buf += "double underline text";
///
/// buf += codes::RESET_UNDERLINE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/double_underline.png)
pub const DOUBLE_UNDERLINE: &str = graphic!(21);
/// Set ouverline mode.
///
/// This mode can be reset with [`RESET_OVERLINE`] or [`RESET`]. Note that
/// [`RESET`] will reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::OVERLINE;
/// buf += "overline text";
///
/// buf += codes::RESET_OVERLINE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/overline.png)
pub const OVERLINE: &str = graphic!(53);

/// Reset [`BOLD`] and [`FAINT`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BOLD;
/// buf += "bold text";
///
/// buf += codes::RESET_BOLD;
/// buf += " normal text\n";
///
/// buf += codes::FAINT;
/// buf += "faint text";
///
/// buf += codes::RESET_BOLD;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/reset_bold.png)
pub const RESET_BOLD: &str = graphic!(22);
/// Reset [`ITALIC`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::ITALIC;
/// buf += "italic text";
///
/// buf += codes::RESET_ITALIC;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/italic.png)
pub const RESET_ITALIC: &str = graphic!(23);
/// Reset [`UNDERLINE`] and [`DOUBLE_UNDERLINE`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::UNDERLINE;
/// buf += "underline text";
///
/// buf += codes::RESET_UNDERLINE;
/// buf += " normal text\n";
///
/// buf += codes::DOUBLE_UNDERLINE;
/// buf += "double underline";
///
/// buf += codes::RESET_UNDERLINE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/reset_underline.png)
pub const RESET_UNDERLINE: &str = graphic!(24);
/// Reset [`BLINKING`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BLINKING;
/// buf += "blinking text";
///
/// buf += codes::RESET_BLINKING;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/blinking.gif)
pub const RESET_BLINKING: &str = graphic!(25);
/// Reset [`INVERSE`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::INVERSE;
/// buf += "inverse text";
///
/// buf += codes::RESET_INVERSE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/inverse.png)
pub const RESET_INVERSE: &str = graphic!(27);
/// Reset [`INVISIBLE`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::INVISIBLE;
/// buf += "invisible text";
///
/// buf += codes::RESET_INVISIBLE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/invisible.png)
pub const RESET_INVISIBLE: &str = graphic!(28);
/// Reset [`STRIKETROUGH`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::STRIKETROUGH;
/// buf += "striketrough text";
///
/// buf += codes::RESET_STRIKETROUGH;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/striketrough.png)
pub const RESET_STRIKETROUGH: &str = graphic!(29);
/// Reset [`OVERLINE`] mode.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::OVERLINE;
/// buf += "overline text";
///
/// buf += codes::RESET_OVERLINE;
/// buf += " normal text";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/overline.png)
pub const RESET_OVERLINE: &str = graphic!(55);

/// Set the foreground color to black (dark black).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::BLACK_FG;
/// buf += " black";
/// buf += codes::WHITE_FG;
/// buf += " white\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::BLACK_FG;
/// buf += " black";
/// buf += codes::WHITE_FG;
/// buf += " white";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/black_fg.png)
pub const BLACK_FG: &str = graphic!(30);
/// Set the foreground color to white (bright white).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::BLACK_FG;
/// buf += " black";
/// buf += codes::WHITE_FG;
/// buf += " white\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::BLACK_FG;
/// buf += " black";
/// buf += codes::WHITE_FG;
/// buf += " white";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/black_fg.png)
pub const WHITE_FG: &str = graphic!(97);
/// Set the foreground color to gray (bright black).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::GRAY_FG;
/// buf += " gray";
/// buf += codes::GRAY_BRIGHT_FG;
/// buf += " bright\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::GRAY_FG;
/// buf += " gray";
/// buf += codes::GRAY_BRIGHT_FG;
/// buf += " bright";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/gray_fg.png)
pub const GRAY_FG: &str = graphic!(90);
/// Set to foreground color to bright gray (dark white).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::GRAY_FG;
/// buf += " gray";
/// buf += codes::GRAY_BRIGHT_FG;
/// buf += " bright\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::GRAY_FG;
/// buf += " gray";
/// buf += codes::GRAY_BRIGHT_FG;
/// buf += " bright";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/gray_fg.png)
pub const GRAY_BRIGHT_FG: &str = graphic!(37);

/// Set the foreground color to red (bright red).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::RED_FG;
/// buf += " red";
/// buf += codes::RED_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::RED_FG;
/// buf += " red";
/// buf += codes::RED_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/red_fg.png)
pub const RED_FG: &str = graphic!(91);
/// Set the foreground color to green (bright green).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::GREEN_FG;
/// buf += " green";
/// buf += codes::GREEN_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::GREEN_FG;
/// buf += " green";
/// buf += codes::GREEN_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/green_fg.png)
pub const GREEN_FG: &str = graphic!(92);
/// Set the foreground color to yellow (bright yellow).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::YELLOW_FG;
/// buf += " yellow";
/// buf += codes::YELLOW_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::YELLOW_FG;
/// buf += " yellow";
/// buf += codes::YELLOW_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/yellow_fg.png)
pub const YELLOW_FG: &str = graphic!(93);
/// Set the foreground color to blue (bright blue).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::BLUE_FG;
/// buf += " blue";
/// buf += codes::BLUE_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::BLUE_FG;
/// buf += " blue";
/// buf += codes::BLUE_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/blue_fg.png)
pub const BLUE_FG: &str = graphic!(94);
/// Set the foreground color to magenta (bright magenta).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::MAGENTA_FG;
/// buf += " magenta";
/// buf += codes::MAGENTA_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::MAGENTA_FG;
/// buf += " magenta";
/// buf += codes::MAGENTA_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/magenta_fg.png)
pub const MAGENTA_FG: &str = graphic!(95);
/// Set the foreground color to cyan (bright cyan).
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::CYAN_FG;
/// buf += " cyan";
/// buf += codes::CYAN_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::CYAN_FG;
/// buf += " cyan";
/// buf += codes::CYAN_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/cyan_fg.png)
pub const CYAN_FG: &str = graphic!(96);

/// Set the foreground color to dark red.
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::RED_FG;
/// buf += " red";
/// buf += codes::RED_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::RED_FG;
/// buf += " red";
/// buf += codes::RED_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/red_fg.png)
pub const RED_DARK_FG: &str = graphic!(31);
/// Set the foreground color to dark green.
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::GREEN_FG;
/// buf += " green";
/// buf += codes::GREEN_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::GREEN_FG;
/// buf += " green";
/// buf += codes::GREEN_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/green_fg.png)
pub const GREEN_DARK_FG: &str = graphic!(32);
/// Set the foreground color to dark yellow.
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::YELLOW_FG;
/// buf += " yellow";
/// buf += codes::YELLOW_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::YELLOW_FG;
/// buf += " yellow";
/// buf += codes::YELLOW_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/yellow_fg.png)
pub const YELLOW_DARK_FG: &str = graphic!(33);
/// Set the foreground color to dark blue.
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::BLUE_FG;
/// buf += " blue";
/// buf += codes::BLUE_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::BLUE_FG;
/// buf += " blue";
/// buf += codes::BLUE_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/blue_fg.png)
pub const BLUE_DARK_FG: &str = graphic!(34);
/// Set the foreground color to dark magenta.
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::MAGENTA_FG;
/// buf += " magenta";
/// buf += codes::MAGENTA_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::MAGENTA_FG;
/// buf += " magenta";
/// buf += codes::MAGENTA_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/magenta_fg.png)
pub const MAGENTA_DARK_FG: &str = graphic!(35);
/// Set the foreground color to dark cyan.
///
/// Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal";
/// buf += codes::CYAN_FG;
/// buf += " cyan";
/// buf += codes::CYAN_DARK_FG;
/// buf += " dark\n";
/// buf += codes::RESET_FG;
///
/// buf += codes::FAINT;
/// buf += "faint ";
/// buf += codes::CYAN_FG;
/// buf += " cyan";
/// buf += codes::CYAN_DARK_FG;
/// buf += " dark";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/cyan_fg.png)
pub const CYAN_DARK_FG: &str = graphic!(36);

/// Reset the foreground color to the default foreground color.
///
/// This doesn't affect [`FAINT`] mode.
///
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::GRAY_BG;
/// buf += codes::YELLOW_FG;
/// buf += "fg and bg";
/// buf += codes::RESET_FG;
/// buf += " bg only";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/reset_fg.png)
pub const RESET_FG: &str = graphic!(39);

/// Set the background color to black (dark black).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BLACK_BG;
/// buf += "black";
/// buf += codes::WHITE_BG;
/// buf += " white";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/black_bg.png)
pub const BLACK_BG: &str = graphic!(40);
/// Set the background color to white (bright white).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BLACK_BG;
/// buf += "black";
/// buf += codes::WHITE_BG;
/// buf += " white";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/black_bg.png)
pub const WHITE_BG: &str = graphic!(107);
/// Set the background color to gray (bright black).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::GRAY_BG;
/// buf += "gray";
/// buf += codes::GRAY_BRIGHT_BG;
/// buf += " bright";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/gray_bg.png)
pub const GRAY_BG: &str = graphic!(100);
/// Set to background color to bright gray (dark white).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::GRAY_BG;
/// buf += "gray";
/// buf += codes::GRAY_BRIGHT_BG;
/// buf += " bright";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/gray_bg.png)
pub const GRAY_BRIGHT_BG: &str = graphic!(47);

/// Set the background color to red (bright red).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::RED_BG;
/// buf += "red";
/// buf += codes::RED_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/red_bg.png)
pub const RED_BG: &str = graphic!(101);
/// Set the background color to green (bright green).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::GREEN_BG;
/// buf += "green";
/// buf += codes::GREEN_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/green_bg.png)
pub const GREEN_BG: &str = graphic!(102);
/// Set the background color to yellow (bright yellow).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::YELLOW_BG;
/// buf += "yellow";
/// buf += codes::YELLOW_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/yellow_bg.png)
pub const YELLOW_BG: &str = graphic!(103);
/// Set the background color to blue (bright blue).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BLUE_BG;
/// buf += "blue";
/// buf += codes::BLUE_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/blue_bg.png)
pub const BLUE_BG: &str = graphic!(104);
/// Set the background color to magenta (bright magenta).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::MAGENTA_BG;
/// buf += "magenta";
/// buf += codes::MAGENTA_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/magenta_bg.png)
pub const MAGENTA_BG: &str = graphic!(105);
/// Set the background color to cyan (bright cyan).
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::CYAN_BG;
/// buf += "cyan";
/// buf += codes::CYAN_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/cyan_bg.png)
pub const CYAN_BG: &str = graphic!(106);

/// Set the background color to dark red.
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::RED_BG;
/// buf += "red";
/// buf += codes::RED_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/red_bg.png)
pub const RED_DARK_BG: &str = graphic!(41);
/// Set the background color to dark green.
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::GREEN_BG;
/// buf += "green";
/// buf += codes::GREEN_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/green_bg.png)
pub const GREEN_DARK_BG: &str = graphic!(42);
/// Set the background color to dark yellow.
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::YELLOW_BG;
/// buf += "yellow";
/// buf += codes::YELLOW_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/yellow_bg.png)
pub const YELLOW_DARK_BG: &str = graphic!(43);
/// Set the background color to dark blue.
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::BLUE_BG;
/// buf += "blue";
/// buf += codes::BLUE_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/blue_bg.png)
pub const BLUE_DARK_BG: &str = graphic!(44);
/// Set the background color to dark magenta.
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::MAGENTA_BG;
/// buf += "magenta";
/// buf += codes::MAGENTA_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/magenta_bg.png)
pub const MAGENTA_DARK_BG: &str = graphic!(45);
/// Set the background color to dark cyan.
///
/// Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
/// [`RESET`] will also reset all text modes.
///
/// Printing newline with background set might fill the whole line to the end
/// with the background color. This is why I recommend to always reset the
/// background color before printing newline.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::CYAN_BG;
/// buf += "cyan";
/// buf += codes::CYAN_DARK_BG;
/// buf += " dark";
/// buf += codes::RESET_BG;
/// buf += " normal";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/cyan_bg.png)
pub const CYAN_DARK_BG: &str = graphic!(46);

/// Reset the background color.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::GRAY_BG;
/// buf += codes::YELLOW_FG;
/// buf += "fg and bg";
/// buf += codes::RESET_BG;
/// buf += " fg only";
/// buf += codes::RESET;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/reset_bg.png)
pub const RESET_BG: &str = graphic!(49);

code_macro! { graphic
    fg256, 38, 5, c;
        ? "
Creates a foreground color, color is value in range 0..256.

Colors in range `0..16` corespond to the named colors in order black, red,
green, yellow, blue, magenta, cyan and yellow. `0..8` are the dark variants and
`8..16` are the bright variants.

Colors in range `16..232` (216 color variants) are usually colors of the form
16 + RGB in base 6. So for example if you want full green, that is `050` in
base 6, in base 10 that is `30` and than we add 16. So the final number for
full green is `46`.

Colors in range `232..256` are usually 24 shades of gray from dark to bright
not including full black and full white. (full black is 16 and full white is
231).

If the argument is literal, this expands to [`&'static str`]. Otherwise this
expands to [`String`].

Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
[`RESET`] will also reset all text modes.

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

for y in 0..16 {
    for x in 0..16 {
        let c = y * 16 + x;

        buf += &codes::fg256!(c);
        buf += &format!(\"{c:03} \");
    }
    buf.push('\\n');
}

buf += codes::RESET_FG;

print!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/fg256.png)
        ",

    bg256, 48, 5, c;
        ? "Creates a background color, color is value in range 0..256.

Colors in range `0..16` corespond to the named colors in order black, red,
green, yellow, blue, magenta, cyan and yellow. `0..8` are the dark variants and
`8..16` are the bright variants.

Colors in range `16..232` (216 color variants) are usually colors of the form
16 + RGB in base 6. So for example if you want full green, that is `050` in
base 6, in base 10 that is `30` and than we add 16. So the final number for
full green is `46`.

Colors in range `232..256` are usually 24 shades of gray from dark to bright
not including full black and full white. (full black is 16 and full white is
231).

If the argument is literal, this expands to [`&'static str`]. Otherwise this
expands to [`String`].

Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
[`RESET`] will also reset all text modes.

Printing newline with background set might fill the whole line to the end with
the background color. This is why I recommend to always reset the background
color before printing newline.

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();

for y in 0..16 {
    for x in 0..16 {
        let c = y * 16 + x;

        buf += &codes::bg256!(c);
        buf += &format!(\"{c:03} \");
    }
    buf += codes::RESET_BG;
    buf.push('\\n');
}


print!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/bg256.png)
        ",

    underline256, 58, 5, c;
        ? "Set underline color as 256 color.

Works for both [`UNDERLINE`] and [`DOUBLE_UNDERLINE`].

Colors in range `0..16` corespond to the named colors in order black, red,
green, yellow, blue, magenta, cyan and yellow. `0..8` are the dark variants and
`8..16` are the bright variants.

Colors in range `16..232` (216 color variants) are usually colors of the form
16 + RGB in base 6. So for example if you want full green, that is `050` in
base 6, in base 10 that is `30` and than we add 16. So the final number for
full green is `46`.

Colors in range `232..256` are usually 24 shades of gray from dark to bright
not including full black and full white. (full black is 16 and full white is
231).

If the argument is literal, this expands to [`&'static str`]. Otherwise this
expands to [`String`].

Underline color can be reset with [`RESET_UNDERLINE_COLOR`] or [`RESET`]. Note
that [`RESET`] will also reset all text modes (uncluding [`UNDERLINE`] and
[`DOUBLE_UNDERLINE`]).

# Example
```no_run
use termal_core::codes;

let mut buf = codes::CLEAR.to_string();
const ULS: &[&str] = &[codes::UNDERLINE, codes::DOUBLE_UNDERLINE];

for y in 0..16 {
    buf += ULS[y % ULS.len()];
    for x in 0..16 {
        let c = y * 16 + x;

        buf += &codes::underline256!(c);
        buf += &format!(\"{c:03} \");
    }
    buf += codes::RESET_UNDERLINE;
    buf.push('\\n');
}

print!(\"{buf}\");
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/underline256.png)
        ",

    fg, 38, 2, r, g, b;
        ? "Creates a true rgb foreground color. R, G and B must be values in
range 0..256.

If the argument is literal, this expands to [`&'static str`]. Otherwise this
expands to [`String`].

Foreground color can be reset with [`RESET_FG`] or [`RESET`]. Note that
[`RESET`] will also reset all text modes.

# Example
```no_run
use termal_core::{codes, raw::term_size, error::Error};

let mut buf = codes::CLEAR.to_string();
let size = term_size()?;
let w = size.char_width;
let h = size.char_height - 1;
let l = (w * h).isqrt();

for y in 0..h {
    for x in 0..w {
        let r = y * 256 / h;
        let g = x * 256 / w;
        let b = 255 - (x * y).isqrt() * 256 / l;

        buf += &codes::fg!(r, g, b);
        buf.push('H');
    }
    buf.push('\\n');
}

print!(\"{buf}\");

Ok::<(), Error>(())
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/fg.png)
",

    bg, 48, 2, r, g, b;
        ? "Creates a true rgb background color. R, G and B must be values in
range 0..256.

If the argument is literal, this expands to [`&'static str`]. Otherwise this
expands to [`String`].

Background color can be reset with [`RESET_BG`] or [`RESET`]. Note that
[`RESET`] will also reset all text modes.

Printing newline with background set might fill the whole line to the end with
the background color. This is why I recommend to always reset the background
color before printing newline.

# Example
```no_run
use termal_core::{codes, raw::term_size, error::Error};

let mut buf = codes::CLEAR.to_string();
let size = term_size()?;
let w = size.char_width;
let h = size.char_height - 1;
let l = (w * h).isqrt();

for y in 0..h {
    for x in 0..w {
        let r = y * 256 / h;
        let g = x * 256 / w;
        let b = 255 - (x * y).isqrt() * 256 / l;

        buf += &codes::bg!(r, g, b);
        buf.push('H');
    }
    buf += codes::RESET_BG;
    buf.push('\\n');
}

print!(\"{buf}\");

Ok::<(), Error>(())
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/bg.png)
    ",

    underline_rgb, 58, 2, r, g, b;
        ? "Set underline color as rgb. R, G, and B muse be values in range
0..256.

If the argument is literal, this expands to [`&'static str`]. Otherwise this
expands to [`String`].

Underline color can be reset with [`RESET_UNDERLINE_COLOR`] or [`RESET`]. Note
that [`RESET`] will also reset all text modes (uncluding [`UNDERLINE`] and
[`DOUBLE_UNDERLINE`]).

# Example
```no_run
use termal_core::{codes, raw::term_size, error::Error};

const ULS: &[&str] = &[codes::UNDERLINE, codes::DOUBLE_UNDERLINE];

let mut buf = codes::CLEAR.to_string();
let size = term_size()?;
let w = size.char_width;
let h = size.char_height - 1;
let l = (w * h).isqrt();

for y in 0..h {
    for x in 0..w {
        let r = y * 256 / h;
        let g = x * 256 / w;
        let b = 255 - (x * y).isqrt() * 256 / l;

        buf += ULS[y % ULS.len()];
        buf += &codes::underline_rgb!(r, g, b);
        buf.push('H');
    }
    buf += codes::RESET_UNDERLINE;
    buf.push('\\n');
}

buf += codes::RESET_UNDERLINE_COLOR;
print!(\"{buf}\");

Ok::<(), Error>(())
```

## Result in terminal
![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/underline_rgb.png)
    ",
}

/// Reset the underline color.
///
/// Underline color may be set by [`underline256`] or [`underline_rgb`].
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::UNDERLINE;
/// buf += codes::underline256!(2);
/// buf += "colored";
/// buf += codes::RESET_UNDERLINE_COLOR;
/// buf += " default";
/// buf += codes::RESET_UNDERLINE;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/reset_underline_color.png)
pub const RESET_UNDERLINE_COLOR: &str = graphic!(59);

// Line modes
/// Makes this line characters twice as large overlapping with the line below.
///
/// Using this code will affect the current line. It can be printed before or
/// after the line contents of the line are printed.
///
/// If the line already has characters, characters that don't fit on the line
/// will clip ouside of the buffer. If this mode is already enabled and
/// characters would clip outside of the buffer, they will move to the next
/// line as usual.
///
/// This line mode can be reset by using [`RESET_CHAR_SIZE`] on the same line,
/// or by using [`ERASE_SCREEN`]. Note that [`ERASE_SCREEN`] will also erase
/// the whole screen buffer.
///
/// Note that the overlapping part of the characters is often clipped when the
/// line that it overlaps changes. They will be usually redrawn when the
/// console window resizes or moved.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "above\n";
/// buf += "double";
/// buf += codes::DOUBLE_CHAR_HEIGHT_DOWN;
/// buf += "\nbelow";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/double_char_height_down.png)
pub const DOUBLE_CHAR_HEIGHT_DOWN: &str = "\x1b#3";
/// Makes this line characters twice as large overlapping with the line above.
pub const DOUBLE_CHAR_HEIGHT_UP: &str = "\x1b#4";
/// Makes this line character twice as wide (but not twice as tall).
pub const DOUBLE_CHAR_WIDTH: &str = "\x1b#6";
/// Resets this line character size.
pub const RESET_CHAR_SIZE: &str = "\x1b#5";

// Screen modes

/// Enables line wrapping
pub const ENABLE_LINE_WRAP: &str = "\x1b[=7h";
/// Disables line wrapping
pub const DISABLE_LINE_WRAP: &str = "\x1b[=7l";

/// Enables reverse color for the whole terminal display.
pub const ENABLE_REVERSE_COLOR: &str = enable!(5);
/// Disables reverse color for the whole terminal display. (This actually often
/// doesn't work)
pub const DISABLE_REVERSE_COLOR: &str = disable!(5);

// Private modes

/// Makes the cursor invisible
pub const HIDE_CURSOR: &str = disable!(25);
/// Makes the cursor visible
pub const SHOW_CURSOR: &str = enable!(25);
/// Saves the visible part of the screen buffer
pub const SAVE_SCREEN: &str = disable!(47);
/// Loads the last saved screen
pub const LOAD_SCREEN: &str = enable!(47);
/// Enables alternative buffer
pub const ENABLE_ALTERNATIVE_BUFFER: &str = enable!(1049);
/// Disables the laternative buffer
pub const DISABLE_ALTERNATIVE_BUFFER: &str = disable!(1049);

// Other
/// Full terminal reset. Clear the screen, buffer, reset all modes, ...
pub const FULL_RESET: &str = "\x1bc";

/// Request the device attributes.
pub const REQUEST_DEVICE_ATTRIBUTES: &str = csi!('c');
/// Request the device status.
pub const REQUEST_STATUS_REPORT: &str = csi!('n', 5);
/// Request the current cursor position. In some terminals, the report may be
/// ambigous with F3 key press with modifiers.
pub const REQUEST_CURSOR_POSITION: &str = csi!('n', 6);
/// Request the current cursor position. Difference from
/// [`REQUEST_CURSOR_POSITION`] is that the response is not ambigous, but it is
/// not supported by some terminals that support [`REQUEST_CURSOR_POSITION`].
pub const REQUEST_CURSOR_POSITION2: &str = "\x1b[?6n";
/// Requests the terminal name and version.
pub const REQUEST_TERMINAL_NAME: &str = "\x1b[>0q";
/// Request the text area size of terminal in pixels.
pub const REQUEST_TEXT_AREA_SIZE_PX: &str = csi!('t', 14);
/// Request size of single character on creen in pixels.
pub const REQUEST_CHAR_SIZE: &str = csi!('t', 16);
/// Request size of the text area in characters.
pub const REQUEST_TEXT_AREA_SIZE: &str = csi!('t', 18);
/// Request the number of sixel color registers.
pub const REQUEST_SIXEL_COLORS: &str = "\x1b[?1;1;1S";

/// Enables mouse tracking for X and Y coordinate on press.
pub const ENABLE_MOUSE_XY_TRACKING: &str = enable!(9);
/// Disables mouse tracking for X and Y coordinate on press.
pub const DISABLE_MOUSE_XY_TRACKING: &str = disable!(9);
/// Enables mouse tracking for X and Y coordinate on press and release.
pub const ENABLE_MOUSE_XY_PR_TRACKING: &str = enable!(1000);
/// Disables mouse tracking for X and Y coordinate on press and release.
pub const DISABLE_MOUSE_XY_PR_TRACKING: &str = disable!(1000);
/// Enables mouse tracking for X and Y coordinate on press, release and drag.
pub const ENABLE_MOUSE_XY_DRAG_TRACKING: &str = enable!(1002);
/// Disables mouse tracking for X and Y coordinate on press, release and drag.
pub const DISABLE_MOUSE_XY_DRAG_TRACKING: &str = disable!(1002);
/// Enables mouse tracking for X and Y coordinate on press, release, drag and
/// move.
pub const ENABLE_MOUSE_XY_ALL_TRACKING: &str = enable!(1003);
/// Disables mouse tracking for X and Y coordinate on press, release, drag and
/// move.
pub const DISABLE_MOUSE_XY_ALL_TRACKING: &str = disable!(1003);
/// Enables sending event on focus gain.
pub const ENABLE_FOCUS_EVENT: &str = enable!(1004);
/// Disables sending event on focus gain.
pub const DISABLE_FOCUS_EVENT: &str = disable!(1004);
/// Enables extension to send mouse inputs in format extended to utf8 two byte
/// characters.
pub const ENABLE_MOUSE_XY_UTF8_EXT: &str = enable!(1005);
/// Disables extension to send mouse inputs in format extended to utf8 two byte
/// characters.
pub const DISABLE_MOUSE_XY_UTF8_EXT: &str = disable!(1005);
/// Enables extension to send mouse inputs in different format as position in
/// characters.
pub const ENABLE_MOUSE_XY_EXT: &str = enable!(1006);
/// Disables extension to send mouse inputs in different format as position in
/// characters.
pub const DISABLE_MOUSE_XY_EXT: &str = disable!(1006);
/// Enables URXVT mouse extension. Not recommended, rather use
/// [`ENABLE_MOUSE_XY_EXT`].
pub const ENABLE_MOUSE_XY_URXVT_EXT: &str = enable!(1015);
/// Disables URXVT mouse extension.
pub const DISABLE_MOUSE_XY_URXVT_EXT: &str = disable!(1015);
/// Enables extension to send mouse inputs in different format as position in
/// pixels.
pub const ENABLE_MOUSE_XY_PIX_EXT: &str = enable!(1016);
/// Disables extension to send mouse inputs in different format as position in
/// pixels.
pub const DISABLE_MOUSE_XY_PIX_EXT: &str = disable!(1016);

code_macro! { csi
    scroll_region, t, b; 'r'
        ? "Set the scroll region in the terminal. Also moves the cursor to the
           top left."
}

/// Reset the scroll region
pub const RESET_SCROLL_REGION: &str = scroll_region!(0, 0);
/// Don't limit the printing area.
pub const DONT_LIMIT_PRINT_TO_SCROLL_REGION: &str = enable!(19);
/// Limit printing area only to scroll region.
pub const LIMIT_PRINT_TO_SCROLL_REGION: &str = disable!(19);

/// Enables bracketed paste mode. In this mode, pasted text is treated
/// verbatim.
pub const ENABLE_BRACKETED_PASTE_MODE: &str = enable!(2004);
pub const DISABLE_BRACKETED_PASTE_MODE: &str = disable!(2004);

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum CursorStyle {
    /// Set cursor to block.
    /// - `true` -> blink
    /// - `false` -> don't blink
    /// - [`None`] -> blink (default)
    Block(Option<bool>),
    /// Set cursor to underline.
    /// - `true` -> blink
    /// - `false` -> don't blink
    Underline(bool),
    /// Set cursor vertical bar.
    /// - `true` -> blink
    /// - `false` -> don't blink
    Bar(bool),
}

pub fn set_cursor(style: CursorStyle) -> &'static str {
    match style {
        CursorStyle::Block(Some(true)) => csi!(" q", 0),
        CursorStyle::Block(None) => csi!(" q", 1),
        CursorStyle::Block(Some(false)) => csi!(" q", 2),
        CursorStyle::Underline(true) => csi!(" q", 3),
        CursorStyle::Underline(false) => csi!(" q", 4),
        CursorStyle::Bar(true) => csi!(" q", 5),
        CursorStyle::Bar(false) => csi!(" q", 6),
    }
}

// OSC sequences

code_macro! {osc
    request_color_code, 4, code, "?";
        ? "Requests the current color assigned to the given color code.",

    reset_color_code, 104, code;
        ? "Resets the color definition for the given color code.",
}

/// Defines color for the given color code.
pub fn define_color_code<T>(code: u8, color: impl Into<Rgb<T>>) -> String
where
    Rgb<T>: Display,
{
    osc!(4, code, color.into())
}

/// Sets the default foreground color
pub fn set_default_fg_color<T>(color: impl Into<Rgb<T>>) -> String
where
    Rgb<T>: Display,
{
    osc!(10, color.into())
}

/// Sets the default foreground color
pub fn set_default_bg_color<T>(color: impl Into<Rgb<T>>) -> String
where
    Rgb<T>: Display,
{
    osc!(11, color.into())
}

/// Sets the color of the cursor.
pub fn set_cursor_color<T>(color: impl Into<Rgb<T>>) -> String
where
    Rgb<T>: Display,
{
    osc!(12, color.into())
}

/// Resets all the color codes to their default colors.
pub const RESET_ALL_COLOR_CODES: &str = osc!(104);
/// Resets the default foreground color.
pub const RESET_DEFAULT_FG_COLOR: &str = osc!(110);
/// Resets the default background color.
pub const RESET_DEFAULT_BG_COLOR: &str = osc!(111);
/// Resets the cursor color.
pub const RESET_CURSOR_COLOR: &str = osc!(112);

/// Requests the default foreground color.
pub const REQUEST_DEFAULT_FG_COLOR: &str = osc!(10, '?');
/// Requests the default background color.
pub const REQUEST_DEFAULT_BG_COLOR: &str = osc!(11, '?');
/// Requests the cursor color.
pub const REQUEST_CURSOR_COLOR: &str = osc!(12, '?');

/// Requests the copy/paste selection data.
pub const REQUEST_SELECTION: &str = osc!(52, "", '?');

/// Specifies the selection buffer.
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Selection {
    Clipboard,
    Primary,
    Secondary,
    // Either [`Primary`] or [`Clipboard`] (what is the configured default)
    Select,
    Cut0,
    Cut1,
    Cut2,
    Cut3,
    Cut4,
    Cut5,
    Cut6,
    Cut7,
}

impl Selection {
    fn get_char(&self) -> char {
        match self {
            Selection::Clipboard => 'c',
            Selection::Primary => 'p',
            Selection::Secondary => 'q',
            Selection::Select => 's',
            Selection::Cut0 => '0',
            Selection::Cut1 => '1',
            Selection::Cut2 => '2',
            Selection::Cut3 => '3',
            Selection::Cut4 => '4',
            Selection::Cut5 => '5',
            Selection::Cut6 => '6',
            Selection::Cut7 => '7',
        }
    }
}

fn prepare_selection(sel: impl IntoIterator<Item = Selection>) -> String {
    let mut res = "\x1b]52;".to_string();
    for b in sel {
        res.push(b.get_char());
    }
    res.push(';');
    res
}

/// Requests selection for the first available of the given selection buffers.
/// If empty requests the default buffer selection.
pub fn request_selection(sel: impl IntoIterator<Item = Selection>) -> String {
    prepare_selection(sel) + "?\x1b\\"
}

/// Sets the given selection buffers. If empty sets the default selection
/// buffers.
pub fn set_selection(
    sel: impl IntoIterator<Item = Selection>,
    data: impl AsRef<[u8]>,
) -> String {
    let mut res = prepare_selection(sel);
    base64::prelude::BASE64_STANDARD.encode_string(data, &mut res);
    res + "\x1b\\"
}

// TODO: Kitty extensions

// Internal

/// Input code for bracketed paste start. Used internally.
pub const BRACKETED_PASTE_START: &str = "\x1b[200~";
/// Input code for bracketed paste end. Used internally.
pub const BRACKETED_PASTE_END: &str = "\x1b[201~";

/// Trait for getting string from &str and String
pub trait GetString {
    /// If [`self`] is `&str` uses `.to_owned()`, if [`self`] is [`String`] returns
    /// [`self`]
    fn get_string(self) -> String;
}

impl GetString for &str {
    fn get_string(self) -> String {
        self.to_owned()
    }
}

impl GetString for String {
    fn get_string(self) -> String {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    fn type_id_of<T: 'static>(_: T) -> TypeId {
        TypeId::of::<T>()
    }

    #[test]
    fn test_macros() {
        assert_eq!(csi!('a', 1, 2, 3, 4, 5), "\x1b[1;2;3;4;5a");
        assert_eq!(csi!('a', 1 + 0, 2, 3, 4, 5), "\x1b[1;2;3;4;5a");
        assert_eq!(type_id_of(csi!('a', 1, 2, 3, 4, 5)), TypeId::of::<&str>());
        assert_eq!(
            type_id_of(csi!('a', 1 + 0, 2, 3, 4, 5)),
            TypeId::of::<String>()
        );
    }
}
