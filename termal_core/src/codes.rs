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

/// The escape character.
pub const ESC: char = '\x1b';
/// Control Sequence Introducer: Start of CSI sequence.
///
/// Equivalent to `ESC [`.
pub const CSI: &str = "\x1b[";
/// Device Control String: Start of DCS sequence.
///
/// Equivalent to `ESC P`.
pub const DCS: &str = "\x1bP";
/// Operating System Command: Start of OSC sequence.
///
/// Equivalent to `ESC ]`
pub const OSC: &str = "\x1b]";
/// String terminator. Terminates for example DCS.
///
/// Equivalent to `ESC \`
pub const ST: &str = "\x1b\\";
/// Single shift three.
///
/// Equivalent to `ESC O`.
pub const SS3: &str = "\x1bO";

/// Creates control escape sequence, the first literal is the end of the
/// sequence, the other arguments are the values in the sequence.
///
/// `csi!(Pi, (Pa),*)` is quivalent to `CSI (Pa);* Pi`.
#[macro_export]
macro_rules! csi {
    ($i:literal $(,$a:expr)* $(,)?) => {
        $crate::seq!("\x1b[", $i $(, $a)*)
    };
}

/// Creates control escape sequence for graphic mode.
///
/// `graphic!((Pa),*)` is quivalent to `CSI (Pa);* m`.
#[macro_export]
macro_rules! graphic {
    ($($a:expr),* $(,)?) => {
        $crate::csi!('m' $(, $a)*)
    };
}

/// Creates operating system command sequence. The arguments are the values in
/// the sequence.
///
/// `osc!((Pa),*)` is quivalent to `OSC (Pa);* ST`.
#[macro_export]
macro_rules! osc {
    ($($a:expr),+) => {
        $crate::seq!("\x1b]", "\x1b\\", $($a),+)
    };
}

/// Enables the given private terminal mode.
///
/// `enable!(Pa)` is quivalent to `CSI ? Pa h`.
#[macro_export]
macro_rules! enable {
    ($a:expr) => {
        $crate::seq!("\x1b[?", 'h', $a)
    };
}

/// Disables the given private terminal mode.
///
/// `enable!(Pa)` is quivalent to `CSI ? Pa l`.
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
/// Equivalent to `CSI Py ; Px H`
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

Equivalent to `CSI Pn A`.

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

Equivalent to `CSI Pn B`.

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

Equivalent to `CSI Pn C`.

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

Equivalent to `CSI Pn D`.

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

Equivalent to `CSI Pn L`.

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

Equivalent to `CSI Pn M`.

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

Equivalent to `CSI Pn @`.

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

Equivalent to `CSI Pn P`.

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

Equivalent to `CSI Pn ' }`.

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
        ? "Delete n columns, moving them from the right.

Equivalent to `CSI Pn ' ~`.

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

Equivalent to `CSI Pn E`.

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

Equivalent to `CSI Pn F`.

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

Equivalent to `CSI Pn b`.

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

Equivalent to `CSI Pn G`.

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
/// Equivalent to `CSI H`
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
pub const MOVE_HOME: &str = csi!('H');

/// Moves cursor one line up, 'scrolling' if needed.
///
/// Equivalent to `ESC M`
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
/// Equivalent to `ESC 7`
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
/// Equivalent to `ESC 8`
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
/// Equivalent to `CSI J`
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
/// Equivalent to `CSI 1 J`
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
/// Equivalent to `CSI 2 J`
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
/// Equivalent to `CSI 3 J`
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
/// Equivalent to `CSI K`
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
/// Equivalent to `CSI 1 K`
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
/// Equivalent to `CSI 2 K`
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
/// Equivalent to `CSI 2 J CSI 3 J`
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
/// Equivalent to `CSI 2 J CSI 3 J CSI H`
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
/// Equivalent to `CSI 0 m`
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
/// Equivalent to `CSI 1 m`
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
/// Equivalent to `CSI 2 m`
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
/// Equivalent to `CSI 3 m`
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
/// Equivalent to `CSI 4 m`
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
/// Equivalent to `CSI 5 m`
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
/// Equivalent to `CSI 7 m`
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
/// Equivalent to `CSI 8 m`
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
/// Equivalent to `CSI 9 m`
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
/// Equivalent to `CSI 2 1 m`
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
/// Equivalent to `CSI 5 3 m`
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
/// Equivalent to `CSI 2 2 m`
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
/// Equivalent to `CSI 2 3 m`
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
/// Equivalent to `CSI 2 4 m`
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
/// Equivalent to `CSI 2 5 m`
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
/// Equivalent to `CSI 2 7 m`
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
/// Equivalent to `CSI 2 8 m`
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
/// Equivalent to `CSI 2 9 m`
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
/// Equivalent to `CSI 5 5 m`
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
/// Equivalent to `CSI 3 0 m`
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
/// Equivalent to `CSI 9 7 m`
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
/// Equivalent to `CSI 9 0 m`
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
/// Equivalent to `CSI 3 7 m`
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
/// Equivalent to `CSI 9 1 m`
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
/// Equivalent to `CSI 9 2 m`
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
/// Equivalent to `CSI 9 3 m`
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
/// Equivalent to `CSI 9 4 m`
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
/// Equivalent to `CSI 9 5 m`
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
/// Equivalent to `CSI 9 6 m`
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
/// Equivalent to `CSI 3 1 m`
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
/// Equivalent to `CSI 3 2 m`
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
/// Equivalent to `CSI 3 3 m`
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
/// Equivalent to `CSI 3 4 m`
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
/// Equivalent to `CSI 3 5 m`
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
/// Equivalent to `CSI 3 6 m`
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
/// Equivalent to `CSI 3 9 m`
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
/// Equivalent to `CSI 4 0 m`
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
/// Equivalent to `CSI 1 0 7 m`
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
/// Equivalent to `CSI 1 0 0 m`
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
/// Equivalent to `CSI 4 7 m`
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
/// Equivalent to `CSI 1 0 1 m`
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
/// Equivalent to `CSI 1 0 2 m`
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
/// Equivalent to `CSI 1 0 3 m`
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
/// Equivalent to `CSI 1 0 4 m`
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
/// Equivalent to `CSI 1 0 5 m`
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
/// Equivalent to `CSI 1 0 6 m`
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
/// Equivalent to `CSI 4 1 m`
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
/// Equivalent to `CSI 4 2 m`
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
/// Equivalent to `CSI 4 3 m`
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
/// Equivalent to `CSI 4 4 m`
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
/// Equivalent to `CSI 4 5 m`
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
/// Equivalent to `CSI 4 6 m`
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
/// Equivalent to `CSI 4 9 m`
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

Equivalent to `CSI 3 8 ; 5 ; Pc m`.

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

Equivalent to `CSI 4 8 ; 5 ; Pc m`.

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

Equivalent to `CSI 5 8 ; 5 ; Pc m`.

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

Equivalent to `CSI 3 8 ; 2 ; Pr ; Pg ; Pb m`.

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

Equivalent to `CSI 4 8 ; 2 ; Pr ; Pg ; Pb m`.

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

Equivalent to `CSI 5 8 ; 2 ; Pr ; Pg ; Pb m`.

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
/// Equivalent to `CSI 5 9 m`.
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
/// Equivalent to `ESC # 3`.
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
/// When this mode is reset (with [`RESET_CHAR_SIZE`]), clipped characters will
/// reapear. Wrapped lines will not unwrap.
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
///
/// Equivalent to `ESC # 4`.
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
/// When this mode is reset (with [`RESET_CHAR_SIZE`]), clipped characters will
/// reapear. Wrapped lines will not unwrap.
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
/// buf += codes::DOUBLE_CHAR_HEIGHT_UP;
/// buf += "\nbelow";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/double_char_height_up.png)
pub const DOUBLE_CHAR_HEIGHT_UP: &str = "\x1b#4";
/// Makes this line character twice as wide (but not twice as tall).
///
/// Equivalent to `ESC # 6`.
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
/// When this mode is reset (with [`RESET_CHAR_SIZE`]), clipped characters will
/// reapear. Wrapped lines will not unwrap.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "normal\n";
/// buf += "double";
/// buf += codes::DOUBLE_CHAR_WIDTH;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/double_char_width.png)
pub const DOUBLE_CHAR_WIDTH: &str = "\x1b#6";
/// Resets this line character size.
///
/// Equivalent to `ESC # 5`.
///
/// This is used to reset [`DOUBLE_CHAR_HEIGHT_DOWN`],
/// [`DOUBLE_CHAR_HEIGHT_UP`] and [`DOUBLE_CHAR_WIDTH`].
///
/// Characters clipped when the mode was set will reapear.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "\nbig1";
/// buf += codes::DOUBLE_CHAR_HEIGHT_UP;
/// buf += "\n\nbig2";
/// buf += codes::DOUBLE_CHAR_HEIGHT_UP;
/// buf += "\nwide1";
/// buf += codes::DOUBLE_CHAR_WIDTH;
/// buf += "\nwide2";
/// buf += codes::DOUBLE_CHAR_WIDTH;
///
/// buf += codes::move_up!(1);
/// buf += codes::RESET_CHAR_SIZE;
/// buf += codes::move_up!(3);
/// buf += codes::RESET_CHAR_SIZE;
///
/// buf += codes::move_down!(4);
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/reset_char_size.png)
pub const RESET_CHAR_SIZE: &str = "\x1b#5";

// Screen modes

/// Enables line wrapping.
///
/// Equivalent to `CSI ? 7 h`.
///
/// Line wrapping is usually enabled by default. It can be disabled with
/// [`DISABLE_LINE_WRAP`].
///
/// This doesn't affect line wrapping behaviour on terminal resize.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::DISABLE_LINE_WRAP;
/// buf +=
///     "this is some long text that doesn't fit on the line without \
///     wrapping\n";
/// buf += codes::ENABLE_LINE_WRAP;
/// buf += "this is some long text that doesn't fit on the line with wrapping";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/disable_line_wrap.png)
pub const ENABLE_LINE_WRAP: &str = enable!(7);
/// Disables line wrapping.
///
/// Equivalent to `CSI ? 7 l`.
///
/// Line wrapping is usually enabled by default. It can be enabled with
/// [`ENABLE_LINE_WRAP`].
///
/// This doesn't affect line wrapping behaviour on terminal resize.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += codes::DISABLE_LINE_WRAP;
/// buf +=
///     "this is some long text that doesn't fit on the line without \
///     wrapping\n";
/// buf += codes::ENABLE_LINE_WRAP;
/// buf += "this is some long text that doesn't fit on the line with wrapping";
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/disable_line_wrap.png)
pub const DISABLE_LINE_WRAP: &str = disable!(7);

/// Enables reverse color for the whole terminal display.
///
/// Equivalent to `CSI ? 5 h`.
///
/// This mode is usually disabled by default and can be disabled by
/// [`DISABLE_REVERSE_COLOR`].
///
/// # Example
/// ```no_run
/// use std::io::Write;
/// use termal_core::{codes, raw::Terminal};
///
/// print!("{}", codes::ENABLE_REVERSE_COLOR);
///
/// _ = Terminal::stdio().flush();
/// _ = Terminal::stdio().read(); // wait for enter
///
/// print!("{}", codes::DISABLE_REVERSE_COLOR);
/// ```
///
/// # Result in terminal
/// The screenshot is taken before enter is pressed.
///
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/enable_reverse_color.png)
pub const ENABLE_REVERSE_COLOR: &str = enable!(5);
/// Disables reverse color for the whole terminal display.
///
/// Equivalent to `CSI ? 5 l`.
///
/// This mode is usually disabled by default and may be enabled by
/// [`ENABLE_REVERSE_COLOR`].
///
/// # Example
/// ```no_run
/// use std::io::Write;
/// use termal_core::{codes, raw::Terminal};
///
/// print!("{}", codes::ENABLE_REVERSE_COLOR);
///
/// _ = Terminal::stdio().flush();
/// _ = Terminal::stdio().read(); // wait for enter
///
/// print!("{}", codes::DISABLE_REVERSE_COLOR);
/// ```
///
/// # Result in terminal
/// The screenshot is taken after enter is pressed.
///
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/disable_reverse_color.png)
pub const DISABLE_REVERSE_COLOR: &str = disable!(5);

// Private modes

/// Makes the cursor invisible.
///
/// Equivalent to `CSI ? 2 5 l`.
///
/// Cursor is usually visible by default. It can be made visible with
/// [`SHOW_CURSOR`].
///
/// # Example
/// ```no_run
/// use std::io::Write;
/// use termal_core::{codes, raw::Terminal};
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "cursor is here > <";
/// buf += codes::HIDE_CURSOR;
/// buf += codes::move_left!(2);
///
/// print!("{buf}");
///
/// _ = Terminal::stdio().flush();
/// _ = Terminal::stdio().read(); // Wait for first enter
///
/// buf = codes::SHOW_CURSOR.to_string();
/// buf += codes::move_right!(16);
/// buf += codes::move_up!(1);
///
/// print!("{buf}");
///
/// _ = Terminal::stdio().flush();
/// _ = Terminal::stdio().read(); // Wait for second enter
/// ```
///
/// # Result in terminal
/// The screenshot is taken before enter is pressed.
///
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/hide_cursor.png)
pub const HIDE_CURSOR: &str = disable!(25);
/// Makes the cursor visible.
///
/// Equivalent to `CSI ? 2 5 h`.
///
/// Cursor is usually visible by default. It can be made invisible with
/// [`HIDE_CURSOR`].
///
/// # Example
/// ```no_run
/// use std::io::Write;
/// use termal_core::{codes, raw::Terminal};
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "cursor is here > <";
/// buf += codes::HIDE_CURSOR;
/// buf += codes::move_left!(2);
///
/// print!("{buf}");
///
/// _ = Terminal::stdio().flush();
/// _ = Terminal::stdio().read(); // Wait for first enter
///
/// buf = codes::SHOW_CURSOR.to_string();
/// buf += codes::move_right!(16);
/// buf += codes::move_up!(1);
///
/// print!("{buf}");
///
/// _ = Terminal::stdio().flush();
/// _ = Terminal::stdio().read(); // Wait for second enter
/// ```
///
/// # Result in terminal
/// The screenshot is taken after first enter press.
///
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/show_curosr.png)
pub const SHOW_CURSOR: &str = enable!(25);
/// Saves the visible part of the screen buffer and the cursor position.
///
/// Equivalent to `CSI ? 4 7 l`.
///
/// The screen and cursor position may be restored to the saved state with
/// [`LOAD_SCREEN`].
///
/// The lines are not preserved; loaded text will not unwrap after resize.
///
/// If you have tui app and you want to preserve the terminal state, rather
/// use [`ENABLE_ALTERNATIVE_BUFFER`] and [`DISABLE_ALTERNATIVE_BUFFER`].
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "This text will be saved and restored";
/// buf += codes::SAVE_SCREEN;
///
/// buf += codes::CLEAR;
/// buf += "You will not see this text because it will be overwritten with \
///     the saved screen";
///
/// buf += codes::LOAD_SCREEN;
///
/// println!("{buf}");
/// ```
///
/// # Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/save_screen.png)
pub const SAVE_SCREEN: &str = disable!(47);
/// Loads the last saved screen and the cursor position.
///
/// Equivalent to `CSI ? 4 7 h`.
///
/// The screen and cursor position are saved with [`SAVE_SCREEN`].
///
/// The lines are not preserved; loaded text will not unwrap after resize.
///
/// If you have tui app and you want to preserve the terminal state, rather
/// use [`ENABLE_ALTERNATIVE_BUFFER`] and [`DISABLE_ALTERNATIVE_BUFFER`].
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "This text will be saved and restored";
/// buf += codes::SAVE_SCREEN;
///
/// buf += codes::CLEAR;
/// buf += "You will not see this text because it will be overwritten with \
///     the saved screen";
///
/// buf += codes::LOAD_SCREEN;
///
/// println!("{buf}");
/// ```
///
/// # Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/save_screen.png)
pub const LOAD_SCREEN: &str = enable!(47);
/// Enables alternative buffer.
///
/// Equivalent to `CSI ? 1 0 4 9 h`.
///
/// Some terminal functionalities are sometimes allowed only in alternative
/// buffer.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "This text is in the default buffer";
/// buf += codes::ENABLE_ALTERNATIVE_BUFFER;
///
/// buf += codes::CLEAR;
/// buf += "In this alternative buffer I can do whatever I want without \
///     affecting the default buffer.";
///
/// buf += codes::DISABLE_ALTERNATIVE_BUFFER;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/enable_alternative_buffer.png)
pub const ENABLE_ALTERNATIVE_BUFFER: &str = enable!(1049);
/// Disables the laternative buffer and loads the previous contents of the
/// default buffer.
///
/// Equivalent to `CSI ? 1 0 4 9 l`.
///
/// Some terminal functionalities are sometimes allowed only in alternative
/// buffer.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::CLEAR.to_string();
///
/// buf += "This text is in the default buffer";
/// buf += codes::ENABLE_ALTERNATIVE_BUFFER;
///
/// buf += codes::CLEAR;
/// buf += "In this alternative buffer I can do whatever I want without \
///     affecting the default buffer.";
///
/// buf += codes::DISABLE_ALTERNATIVE_BUFFER;
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/enable_alternative_buffer.png)
pub const DISABLE_ALTERNATIVE_BUFFER: &str = disable!(1049);

// Other
/// Full terminal reset. Clear the screen, buffer, reset all modes, ...
///
/// Equivalent to `ESC c`.
///
/// # Example
/// ```no_run
/// use termal_core::codes;
///
/// let mut buf = codes::HIDE_CURSOR.to_string();
/// buf += codes::ENABLE_REVERSE_COLOR;
/// buf += "printing some text";
///
/// buf += codes::FULL_RESET;
///
/// print!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/full_reset.png)
pub const FULL_RESET: &str = "\x1bc";

/// Request the device attributes.
///
/// Equivalent to `CSI c`.
///
/// The terminal will reply with one of the following options based on its
/// attributes:
/// - `CSI ? 1 ; 2 c`: VT100
/// - `CSI ? 1 ; 0 c`: VT101
/// - `CSI ? 4 ; 6 c`: VT132
/// - `CSI ? 6 c`: VT102
/// - `CSI ? 7 c`: VT131
/// - `CSI ? 1 2 ; Ps c`: VT125
/// - `CSI ? 6 2 ; Ps c`: VT220
/// - `CSI ? 6 3 ; Ps c`: VT320
/// - `CSI ? 6 4 ; Ps c`: VT420
/// - `CSI ? 6 5 ; Ps c`: VT510 - VT525
///
/// Where `Ps` is list of terminal features:
/// - `1`: 132-columns
/// - `2`: Printer
/// - `3`: ReGIS graphics
/// - `4`: Sixel graphics
/// - `6`: Selective erase
/// - `8`: User-defined keys
/// - `9`: National Replacement Character sets
/// - `1 5`: Technical characters
/// - `1 6`: Locator port
/// - `1 7`: Terminal state integration
/// - `1 8`: User windows
/// - `2 1`: Horizontal scrolling
/// - `2 2`: ANSI color
/// - `2 8`: Rectangular editing
/// - `2 9`: ANSI text locator
///
/// Most of these features are not supported by modern terminals because of
/// modern alternatives or because they are not used.
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::Attributes`] with instance of
/// [`crate::raw::events::TermAttr`]. So the event will match
/// `Event::Status(Status::Attributes(TermAttr { .. }))`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_DEVICE_ATTRIBUTES);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_device_attributes.png)
pub const REQUEST_DEVICE_ATTRIBUTES: &str = csi!('c');
/// Request the device status.
///
/// Equivalent to `CSI 5 n`.
///
/// Basically ping the terminal :).
///
/// The terminal will reply with `CSI 0 n`.
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::Ok`]. So the read event will match
/// `Event::Status(Status::Ok)`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_STATUS_REPORT);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_status_report.png)
pub const REQUEST_STATUS_REPORT: &str = csi!('n', 5);
/// Request the current cursor position. In some terminals, the report may be
/// ambigous with F3 key press with modifiers.
///
/// Equivalent to `CSI 6 n`.
///
/// The terminal will reply with `CSI Px ; Py R` where `Px` is the column and
/// `Py` is the row. Top left corner is `Px` = `1` and `Py` = 1.
///
/// The code `CSI 1 ; Ps R` is in some terminals used for the key press `F3`
/// with `Ps` being the modifiers. This is ambiguous with the report of cursor
/// position. In this ambiguous case, termal will choose `F3` key press as the
/// primary interpretation because it is more likely when the application
/// desn't expect to receive cursor position report. If you want to avoid this
/// ambiguity, you can use [`REQUEST_CURSOR_POSITION2`], but it is not
/// supported by some terminals that do support this code.
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::CursorPosition`]. So the read event will
/// match `Event::Status(Status::CursorPosition { x: _, y: _ })`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// print!("{}", codes::move_to!(5, 2));
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_CURSOR_POSITION);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_cursor_position.png)
pub const REQUEST_CURSOR_POSITION: &str = csi!('n', 6);
/// Request the current cursor position. Difference from
/// [`REQUEST_CURSOR_POSITION`] is that the response is not ambigous, but it is
/// not supported by some terminals that support [`REQUEST_CURSOR_POSITION`].
///
/// Equivalent to `CSI ? 6 n`.
///
/// The terminal will reply with `CSI ? Px ; Py R` where `Px` is the column and
/// `Py` is the row. Top left corner is `Px` = `1` and `Py` = 1.
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::CursorPosition`]. So the read event will
/// match `Event::Status(Status::CursorPosition { x: _, y: _ })`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// print!("{}", codes::move_to!(5, 2));
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_CURSOR_POSITION2);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_cursor_position.png)
pub const REQUEST_CURSOR_POSITION2: &str = "\x1b[?6n";
/// Requests the terminal name and version.
///
/// Equivalent to `CSI > 0 q`.
///
/// The terminal will reply with `DCS > | text ST` where `text` is the terminal
/// name and version.
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::TerminalName`]. So the read event will match
/// `Event::Status(Status::TerminalName(_))`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_TERMINAL_NAME);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_terminal_name.png)
pub const REQUEST_TERMINAL_NAME: &str = "\x1b[>0q";
/// Request the text area size of the terminal in pixels.
///
/// Equivalent to `CSI 1 4 t`.
///
/// The terminal will reply with `CSI 4 ; Ph ; Pw t` where `Ph` is the height
/// name `Pw` is the width of the terminal text area in pixels.
///
/// On unix (linux) it is better to use [`crate::raw::term_size`]. Windows
/// doesn't provide size in pixels when using [`crate::raw::term_size`].
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::TextAreaSizePx`]. So the read event will match
/// `Event::Status(Status::TextAreaSizePx { w: _, h: _ })`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_TEXT_AREA_SIZE_PX);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_text_area_size_px.png)
pub const REQUEST_TEXT_AREA_SIZE_PX: &str = csi!('t', 14);
/// Request size of single character on screen in pixels.
///
/// Equivalent to `CSI 1 6 t`.
///
/// The terminal will reply with `CSI 6 ; Ph ; Pw t` where `Ph` is the height
/// name `Pw` is the width of the terminal text area in pixels.
///
/// On unix (linux) it is better to use [`crate::raw::term_size`] and calculate
/// the character size. Windows doesn't provide size in pixels when using
/// [`crate::raw::term_size`].
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::CharSize`]. So the read event will match
/// `Event::Status(Status::CharSize { w: _, h: _ })`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_CHAR_SIZE);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_char_size.png)
pub const REQUEST_CHAR_SIZE: &str = csi!('t', 16);
/// Request size of the text area in characters.
///
/// Equivalent to `CSI 1 8 t`.
///
/// The terminal will reply with `CSI 8 ; Ph ; Pw t` where `Ph` is the height
/// name `Pw` is the width of the terminal text area in characters.
///
/// On unix (linux) and windows it is better to use [`crate::raw::term_size`].
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::TextAreaSize`]. So the read event will match
/// `Event::Status(Status::TextAreaSize { w: _, h: _ })`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_TEXT_AREA_SIZE);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_text_area_size.png)
pub const REQUEST_TEXT_AREA_SIZE: &str = csi!('t', 18);
/// Request the number of sixel color registers.
///
/// Equivalent to `CSI ? 1 ; 1 ; 1 S`.
///
/// The terminal will reply with `CSI ? 1 ; 0 ; Ps S` where `Ps` is the number
/// of sixel registers that are supported by this terminal.
///
/// Note that terminals that don't support sixels will propably not reply.
///
/// Termal can parse the response as [`crate::raw::events::Event::Status`] with
/// [`crate::raw::events::Status::SixelColors`]. So the read event will match
/// `Event::Status(Status::SixelColors(_))`.
///
/// # Example
/// ```no_run
/// use termal_core::{
///     raw::{enable_raw_mode, disable_raw_mode, Terminal}, codes
/// };
/// use std::io::Write;
///
/// enable_raw_mode()?;
///
/// print!("{}", codes::REQUEST_SIXEL_COLORS);
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// let event = term.read()?;
///
/// disable_raw_mode()?;
///
/// println!("{}{event:#?}", codes::CLEAR);
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/request_sixel_colors.png)
pub const REQUEST_SIXEL_COLORS: &str = "\x1b[?1;1;1S";

/// Enables mouse tracking for X and Y coordinate on press (mouse down).
///
/// Equivalent to `CSI ? 9 h`.
///
/// This is usually not supported by terminals. The code
/// [`ENABLE_MOUSE_XY_PR_TRACKING`] is more widely supported.
///
/// The terminal will reply with `CSI M Cb Cx Cy` where:
/// - `Cb` is the button pressed on the mouse
///     - ` ` (space) for primary button (left)
///     - `!` for middle button
///     - `"` for secondary button (right)
/// - `Cx` and `Cy` are character coordinates of the mouse press. They are
///   encoded as single byte character where ordinary value of the character -
///   32 is the value of the coordinate.
///
/// Note that the responses usually aren't well formed CSI escape sequences and
/// also not well formed UTF-8 strings.
///
/// Termal can parse the responses as [`crate::raw::events::Event::Mouse`] with
/// [`crate::raw::events::mouse::Mouse`]. So the read event will match:
/// ```ignore
/// Event::Mouse(mouse::Mouse {
///     button:
///         mouse::Button::Left | mouse::Button::Middle | mouse::Button::Right,
///     event: mouse::Event::Down,
///     modifiers: Modifiers::NONE,
///     x: 1..=233,
///     y: 1..=233,
/// })
/// ```
///
/// The limitation on the maximum value of `x` and `y` coordinate comes from
/// the fact that single byte character can have only value from 0 to 255 and
/// the first 32 characters are unused. (255 - 32 == 233)
///
/// # Example
/// ```no_run
/// use termal_core::{
///     codes,
///     raw::{
///         enable_raw_mode, disable_raw_mode, Terminal,
///         events::{Event, Key, KeyCode, Modifiers},
///     },
/// };
/// use std::io::Write;
///
/// print!("{}", codes::ENABLE_MOUSE_XY_TRACKING);
/// print!("{}", codes::CLEAR);
///
/// enable_raw_mode()?;
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// loop {
///     let event = term.read()?;
///     term.flushed(format!("{}{event:#?}", codes::CLEAR))?;
///     if matches!(
///         event,
///         Event::KeyPress(Key { code: KeyCode::Char('c'), modifiers, .. })
///             if modifiers.contains(Modifiers::CONTROL)
///     ) {
///         break;
///     }
/// }
///
/// print!("{}", codes::DISABLE_MOUSE_XY_TRACKING);
/// term.flush()?;
///
/// disable_raw_mode()?;
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/enable_mouse_xy_tracking.gif)
pub const ENABLE_MOUSE_XY_TRACKING: &str = enable!(9);
/// Disables mouse tracking for X and Y coordinate on press enabled by
/// [`ENABLE_MOUSE_XY_TRACKING`].
///
/// Equivalent to `CSI ? 9 l`.
///
/// See [`ENABLE_MOUSE_XY_TRACKING`] for more info about the mouse tracking.
pub const DISABLE_MOUSE_XY_TRACKING: &str = disable!(9);
/// Enables mouse tracking for X and Y coordinate on press and release and
/// track mouse scroll events. Also reports modifiers.
///
/// Equivalent to `CSI ? 1000 h`.
///
/// The terminal will reply with `CSI M Cb Cx Cy` where:
/// - `Cb` is the button pressed on the mouse, event and modifiers. Value that
///   it represents is ordinary value of the single byte character - 32. The
///   value bits from lowest to highest (`76543210`) have this meaning:
///     - Bits `7610` form number representing the mouse button:
///         - `0` primary press (left)
///         - `1` middle press
///         - `2` secondary press (right)
///         - `3` button release (may be any button that was pressed)
///         - `4` scroll up
///         - `5` scroll down
///         - `8` button 4 (back)
///         - `9` button 5 (forward)
///         - `10` button 6
///         - `11` button 7
///     - Bit `2` represents whether shift was pressed with the event.
///     - Bit `3` represents whether alt was pressed with the event.
///     - Bit `4` represents whether control was pressed with the event.
/// - `Cx` and `Cy` are character coordinates of the mouse press. They are
///   encoded as single byte character where ordinary value of the character -
///   32 is the value of the coordinate.
///
/// Note that the responses usually aren't well formed CSI escape sequences and
/// also not well formed UTF-8 strings.
///
/// The reply sequence may be modified with extensions:
/// - [`ENABLE_MOUSE_XY_UTF8_EXT`]: use utf8 encoding to increase the
///   coordinate range to `1..2015`
/// - [`ENABLE_MOUSE_XY_EXT`]: use proper CSI sequences. Coordinate range is
///   unlimited and button release contains information about which button was
///   released.
/// - [`ENABLE_MOUSE_XY_URXVT_EXT`]: use proper CSI sequences. Coordinate range
///   is unlimited. May be ambiguous with other events so it is not
///   recommended.
/// - [`ENABLE_MOUSE_XY_PIX_EXT`]: Same as [`ENABLE_MOUSE_XY_EXT`] but the
///   coordinates are in pxels instead of characters.
///
/// See the respective codes for more detailed description.
///
/// Termal can parse the responses as [`crate::raw::events::Event::Mouse`] with
/// [`crate::raw::events::mouse::Mouse`]. So the read event will match:
/// ```ignore
/// Event::Mouse(mouse::Mouse {
///     button: _,
///     event: !Event::Move, // (anything except move)
///     modifiers: Modifiers::SHIFT | Modifiers::ALT | Modifiers::CONTROL,
///     x: 1..=233,
///     y: 1..=233,
/// })
/// ```
///
/// The limitation on the maximum value of `x` and `y` coordinate comes from
/// the fact that single byte character can have only value from 0 to 255 and
/// the first 32 characters are unused. (255 - 32 == 233)
///
/// # Example
/// ```no_run
/// use termal_core::{
///     codes,
///     raw::{
///         enable_raw_mode, disable_raw_mode, Terminal,
///         events::{Event, Key, KeyCode, Modifiers},
///     },
/// };
/// use std::io::Write;
///
/// print!("{}", codes::ENABLE_MOUSE_XY_PR_TRACKING);
/// print!("{}", codes::CLEAR);
///
/// enable_raw_mode()?;
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// loop {
///     let event = term.read()?;
///     term.flushed(format!("{}{event:#?}", codes::CLEAR))?;
///     if matches!(
///         event,
///         Event::KeyPress(Key { code: KeyCode::Char('c'), modifiers, .. })
///             if modifiers.contains(Modifiers::CONTROL)
///     ) {
///         break;
///     }
/// }
///
/// print!("{}", codes::DISABLE_MOUSE_XY_PR_TRACKING);
/// term.flush()?;
///
/// disable_raw_mode()?;
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/enable_mouse_xy_pr_tracking.gif)
pub const ENABLE_MOUSE_XY_PR_TRACKING: &str = enable!(1000);
/// Disables mouse tracking for X and Y coordinate on press and release.
///
/// Equivalent to `CSI ? 1000 l`.
///
/// See [`ENABLE_MOUSE_XY_PR_TRACKING`] for more info.
pub const DISABLE_MOUSE_XY_PR_TRACKING: &str = disable!(1000);
/// Enables mouse tracking for X and Y coordinate on press, release and drag.
/// Also reacts to mouse scroll wheel.
///
/// Equivalent to `CSI ? 1002 h`.
///
/// The terminal will reply with `CSI M Cb Cx Cy` where:
/// - `Cb` is the button pressed on the mouse, event and modifiers. Value that
///   it represents is ordinary value of the single byte character - 32. The
///   value bits from lowest to highest (`76543210`) have this meaning:
///     - Bits `7610` form number representing the mouse button:
///         - `0` primary press (left)
///         - `1` middle press
///         - `2` secondary press (right)
///         - `3` button release (may be any button that was pressed)
///         - `4` scroll up
///         - `5` scroll down
///         - `8` button 4 (back)
///         - `9` button 5 (forward)
///         - `10` button 6
///         - `11` button 7
///     - Bit `2` represents whether shift was pressed with the event.
///     - Bit `3` represents whether alt was pressed with the event.
///     - Bit `4` represents whether control was pressed with the event.
///     - If bit `5` is set, the event is that mouse moved and not that key was
///       pressed.
/// - `Cx` and `Cy` are character coordinates of the mouse press. They are
///   encoded as single byte character where ordinary value of the character -
///   32 is the value of the coordinate.
///
/// Note that the responses usually aren't well formed CSI escape sequences and
/// also not well formed UTF-8 strings.
///
/// The reply sequence may be modified with extensions:
/// - [`ENABLE_MOUSE_XY_UTF8_EXT`]: use utf8 encoding to increase the
///   coordinate range to `1..2015`
/// - [`ENABLE_MOUSE_XY_EXT`]: use proper CSI sequences. Coordinate range is
///   unlimited and button release contains information about which button was
///   released.
/// - [`ENABLE_MOUSE_XY_URXVT_EXT`]: use proper CSI sequences. Coordinate range
///   is unlimited. May be ambiguous with other events so it is not
///   recommended.
/// - [`ENABLE_MOUSE_XY_PIX_EXT`]: Same as [`ENABLE_MOUSE_XY_EXT`] but the
///   coordinates are in pxels instead of characters.
///
/// See the respective codes for more detailed description.
///
/// Termal can parse the responses as [`crate::raw::events::Event::Mouse`] with
/// [`crate::raw::events::mouse::Mouse`]. So the read event will match:
/// ```ignore
/// Event::Mouse(mouse::Mouse {
///     button: _,
///     event: _,
///     modifiers: Modifiers::SHIFT | Modifiers::ALT | Modifiers::CONTROL,
///     x: 1..=233,
///     y: 1..=233,
/// })
/// ```
///
/// The limitation on the maximum value of `x` and `y` coordinate comes from
/// the fact that single byte character can have only value from 0 to 255 and
/// the first 32 characters are unused. (255 - 32 == 233)
///
/// # Example
/// ```no_run
/// use termal_core::{
///     codes,
///     raw::{
///         enable_raw_mode, disable_raw_mode, Terminal,
///         events::{Event, Key, KeyCode, Modifiers},
///     },
/// };
/// use std::io::Write;
///
/// print!("{}", codes::ENABLE_MOUSE_XY_DRAG_TRACKING);
/// print!("{}", codes::CLEAR);
///
/// enable_raw_mode()?;
///
/// let mut term = Terminal::stdio();
/// term.flush()?;
///
/// loop {
///     let event = term.read()?;
///     term.flushed(format!("{}{event:#?}", codes::CLEAR))?;
///     if matches!(
///         event,
///         Event::KeyPress(Key { code: KeyCode::Char('c'), modifiers, .. })
///             if modifiers.contains(Modifiers::CONTROL)
///     ) {
///         break;
///     }
/// }
///
/// print!("{}", codes::DISABLE_MOUSE_XY_DRAG_TRACKING);
/// term.flush()?;
///
/// disable_raw_mode()?;
///
/// # Ok::<_, termal_core::error::Error>(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/codes/enable_mouse_xy_drag_tracking.gif)
pub const ENABLE_MOUSE_XY_DRAG_TRACKING: &str = enable!(1002);
/// Disables mouse tracking for X and Y coordinate on press, release and drag.
///
/// Equivalent to `CSI ? 1002 l`.
///
/// See [`ENABLE_MOUSE_XY_DRAG_TRACKING`] for more info.
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
