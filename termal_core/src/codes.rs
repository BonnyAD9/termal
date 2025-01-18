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

use place_macro::place;

/// Creates the given sequence, this is used internally, you should use
/// the macro [`csi`]
#[macro_export]
macro_rules! seq {
    ($sq:literal, $i:literal, $f:literal, $($a:literal),*) => {
        concat!($sq, $f $(, ';', $a)*, $i)
    };
    ($sq:literal, $i:literal, $f:expr $(,$a:expr)*) => {
        $crate::seq!($sq, $i, $f, $(";{}"; $a),*)
    };
    ($sq:literal, $i:literal, $f:expr, $($l:literal; $e:expr),*) => {
        format!(concat!($sq, "{}" $(,$l)*, $i), $f $(,$e)*)
    }
}

// Sequences:

/// The escape character
pub const ESC: char = '\x1b';
/// Control Sequence Introducer: Start of CSI sequence
pub const CSI: &str = "\x1b[";
/// Device Control String: Start of DCS sequence
pub const DCS: &str = "\x1bP";
/// Operating System Command: Start of OCS sequence
pub const OCS: &str = "\x1b]";
/// String terminator. Terminates for example DCS.
pub const ST: &str = "\x1b\\";
/// Single shift three
pub const SS3: &str = "\x1bO";

/// Creates escape sequence, the first literal is the end of the sequence,
/// the other arguments are the values in the sequence
#[macro_export]
macro_rules! csi {
    ($i:literal, $($a:expr),+) => {
        $crate::seq!("\x1b[", $i, $($a),+)
    };
}

// General ASCII codes

/// Produces terminal bell
pub const BELL: char = '\x07';
/// Moves the cursor left by one positoin
pub const BACKSPACE: char = '\x08';
/// Horizontal tabulator, moves cursor to the next stop
pub const HTAB: char = '\t';
/// Moves the cursor to the start of the next line
pub const NEWLINE: char = '\n';
/// Vertical tabulator, moves the cursor to the next vertical stop
pub const VTAB: char = '\x0b';
/// Indicates new page, usualy has no use in terminal
pub const FORMFEED: char = '\x0c';
/// Moves cursor to the start of the line
pub const CARRIAGE_RETURN: char = '\r';
/// Does nothing
pub const DELETE: char = '\x7f';

// Cursor controls

// For the macros is true that:
// If you use literals it returns `&str`,
// if you use expressions, it returns [`String`]. You can use the
// `.get_string()` method from the trait [`GetString`] to get [`String`] in
// both cases

macro_rules! csi_macro {
    ($(
        $name:ident
        $(, $($nam:ident)? $($lit:literal)?)+ ;
        $i:literal $(?$doc:literal)?),+ $(,)?
    ) => {
        place! {$(
            $(#[doc = __repnl__($doc, " ")])?
            #[macro_export]
            macro_rules! $name {
                (__start__($($(__s__ $nam:expr,)?)+) __s__ (,)?) => {
                    __s__ crate::csi!($i, $($(__s__ $nam)? $($lit)?),+)
                }
            }
            pub use $name;
        )+}
    };
    (!= $ex:literal => $(
        $name:ident,
        $nam:ident;
        $i:literal $(?$doc:literal)?),+ $(,)?
    ) => {
        place! {$(
            $(#[doc = __repnl__($doc, " ")])?
            #[macro_export]
            macro_rules! $name {
                (__start__(__s__ $nam:expr,)) => {{
                    let v = __s__ $nam;
                    if v == $ex {
                        "".into()
                    } else {
                        __s__ crate::csi!($i, v)
                    }
                }}
            }
            pub use $name;
        )+}
    };
}

/// Moves cursor to the given position.
#[macro_export]
macro_rules! move_to {
    ($x:expr, $y:expr) => {
        $crate::csi!('H', $y, $x)
    };
}

pub use move_to;

csi_macro!( != 0 =>
    move_up, n; 'A' ? "Moves cursor up by N positions",
    move_down, n; 'B' ? "Moves cursor down by N positions",
    move_right, n; 'C' ? "Moves cursor right by N positions",
    move_left, n; 'D' ? "Moves cursor left by N positions",
    insert_lines, n; 'L' ? "Insert n lines at the cursor moving them down.",
    delete_lines, n; 'M'
        ? "Delete n lines at the cursor, moving the remaining from bottom.",
    insert_chars, n; '@' ? "Insert n characters, moving them to the right.",
    delete_chars, n; 'P' ? "Delete n characters, moving the chars from right.",
    insert_columns, n; "'}}" ? "Insert n columns, moving them to the right.",
    delete_columns, n; "'~" ? "Delete n columns, moving them from the right",
    set_down, n; 'E' ? "Moves cursor to the start of line N lines down",
    set_up, n; 'F' ? "Moves cursor to the start of line N lines up",
);

csi_macro!(
    column, n; 'G' ? "Moves cursor to the given column",
);

/// Moves cursor one line up, scrolling if needed
pub const UP_SCRL: &str = "\x1bM";
/// Saves the cursor position (this is single save slot, not stack)
pub const CUR_SAVE: &str = "\x1b7";
/// Restores the cursor position to the last saved position (this is single
/// save slot, not stack)
pub const CUR_LOAD: &str = "\x1b8";

// Erase codes

/// Erases from the cursor to the end of the screen
pub const ERASE_TO_END: &str = "\x1b[J";
/// Erases from the start of the screen to the cursor
pub const ERASE_FROM_START: &str = "\x1b[1J";
/// Erases the entire screen
pub const ERASE_SCREEN: &str = "\x1b[2J";
/// Erases the whole screen and the scrollback buffer
pub const ERASE_ALL: &str = "\x1b[3J";
/// Erases from cursor to the end of the line
pub const ERASE_TO_LN_END: &str = "\x1b[K";
/// Erases from the start of the line to the cursor
pub const ERASE_FROM_LN_START: &str = "\x1b[1K";
/// Erases the entire line
pub const ERASE_LINE: &str = "\x1b[2K";

// Text modes

/// Resets all the text modes (colors and styles)
pub const RESET: &str = "\x1b[0m";

/// Set bold text mode (on some terminals may be just brighter color)
pub const BOLD: &str = "\x1b[1m";
/// Set dim/faint text mode
pub const FAINT: &str = "\x1b[2m";
/// Set italic mode
pub const ITALIC: &str = "\x1b[3m";
/// Set underline mode
pub const UNDERLINE: &str = "\x1b[4m";
/// Set blinking mode
pub const BLINKING: &str = "\x1b[5m";
/// Set inverse mode (inverse foreground and background)
pub const INVERSE: &str = "\x1b[7m";
/// Set invisible mode (foreground is same as background)
pub const INVISIBLE: &str = "\x1b[8m";
/// Set striketrough mode
pub const STRIKETROUGH: &str = "\x1b[9m";
/// Set double underline mode
pub const DOUBLE_UNDERLINE: &str = "\x1b[21";

/// Reset [`BOLD`] and [`FAINT`] mode
pub const RESET_BOLD: &str = "\x1b[22m";
/// Reset [`ITALIC`] mode
pub const RESET_ITALIC: &str = "\x1b[23m";
/// Reset [`UNDERLINE`] and [`DOUBLE_UNDERLINE`] mode
pub const RESET_UNDERLINE: &str = "\x1b[24m";
/// Reset [`BLINKING`] mode
pub const RESET_BLINKING: &str = "\x1b[25m";
/// Reset [`INVERSE`] mode
pub const RESET_INVERSE: &str = "\x1b[27m";
/// Reset [`INVISIBLE`] mode
pub const RESET_INVISIBLE: &str = "\x1b[28m";
/// Reset [`STRIKETROUGH`] mode
pub const RESET_STRIKETROUGH: &str = "\x1b[29m";

/// Set the foreground color to black (dark black)
pub const BLACK_FG: &str = "\x1b[30m";
/// Set the foreground color to white (bright white)
pub const WHITE_FG: &str = "\x1b[97m";
/// Set the foreground color to gray (bright black)
pub const GRAY_FG: &str = "\x1b[90m";
/// Set to foreground color to bright gray (dark white)
pub const GRAY_BRIGHT_FG: &str = "\x1b[37m";

/// Set the foreground color to red (bright red)
pub const RED_FG: &str = "\x1b[91m";
/// Set the foreground color to green (bright green)
pub const GREEN_FG: &str = "\x1b[92m";
/// Set the foreground color to yellow (bright yellow)
pub const YELLOW_FG: &str = "\x1b[93m";
/// Set the foreground color to blue (bright blue)
pub const BLUE_FG: &str = "\x1b[94m";
/// Set the foreground color to magenta (bright magenta)
pub const MAGENTA_FG: &str = "\x1b[95m";
/// Set the foreground color to cyan (bright cyan)
pub const CYAN_FG: &str = "\x1b[96m";

/// Set the foreground color to dark red
pub const RED_DARK_FG: &str = "\x1b[31m";
/// Set the foreground color to dark green
pub const GREEN_DARK_FG: &str = "\x1b[32m";
/// Set the foreground color to dark yellow
pub const YELLOW_DARK_FG: &str = "\x1b[33m";
/// Set the foreground color to dark blue
pub const BLUE_DARK_FG: &str = "\x1b[34m";
/// Set the foreground color to dark magenta
pub const MAGENTA_DARK_FG: &str = "\x1b[35m";
/// Set the foreground color to dark cyan
pub const CYAN_DARK_FG: &str = "\x1b[36m";

/// Reset the foreground color
pub const RESET_FG: &str = "\x1b[39m";

/// Set the background color to black (dark black)
pub const BLACK_BG: &str = "\x1b[40m";
/// Set the background color to white (bright white)
pub const WHITE_BG: &str = "\x1b[107m";
/// Set the background color to gray (bright black)
pub const GRAY_BG: &str = "\x1b[100m";
/// Set to background color to bright gray (dark white)
pub const GRAY_BRIGHT_BG: &str = "\x1b[47m";

/// Set the background color to red (bright red)
pub const RED_BG: &str = "\x1b[101m";
/// Set the background color to green (bright green)
pub const GREEN_BG: &str = "\x1b[102m";
/// Set the background color to yellow (bright yellow)
pub const YELLOW_BG: &str = "\x1b[103m";
/// Set the background color to blue (bright blue)
pub const BLUE_BG: &str = "\x1b[104m";
/// Set the background color to magenta (bright magenta)
pub const MAGENTA_BG: &str = "\x1b[105m";
/// Set the background color to cyan (bright cyan)
pub const CYAN_BG: &str = "\x1b[106m";

/// Set the background color to dark red
pub const RED_DARK_BG: &str = "\x1b[41m";
/// Set the background color to dark green
pub const GREEN_DARK_BG: &str = "\x1b[42m";
/// Set the background color to dark yellow
pub const YELLOW_DARK_BG: &str = "\x1b[43m";
/// Set the background color to dark blue
pub const BLUE_DARK_BG: &str = "\x1b[44m";
/// Set the background color to dark magenta
pub const MAGENTA_DARK_BG: &str = "\x1b[45m";
/// Set the background color to dark cyan
pub const CYAN_DARK_BG: &str = "\x1b[46m";

/// Reset the background color
pub const RESET_BG: &str = "\x1b[49m";

csi_macro! {
    fg256, 38, 5, c; 'm'
        ? "creates a foreground color, color is value in range 0..256",

    bg256, 48, 5, c; 'm'
        ? "creates a background color, color is value in range 0..256",

    fg, 38, 2, r, g, b; 'm'
        ? "creates a true rgb foreground color. R, G and B must be values in
           range 0..256",

    bg, 48, 2, r, g, b; 'm'
        ? "creates a true rgb background color. R, G and B must be values in
           range 0..256",
    repeat_char, n; 'b'
        ? "Repeat the previous char n times."
}

// Screen modes

/// Enables line wrapping
pub const ENABLE_LINE_WRAP: &str = "\x1b[=7h";
/// Disables line wrapping
pub const DISABLE_LINE_WRAP: &str = "\x1b[=7l";

// Private modes

/// Makes the cursor invisible
pub const HIDE_CURSOR: &str = "\x1b[?25l";
/// Makes the cursor visible
pub const SHOW_CURSOR: &str = "\x1b[?25h";
/// Saves the visible part of the screen buffer
pub const SAVE_SCREEN: &str = "\x1b[?47l";
/// Loads the last saved screen
pub const LOAD_SCREEN: &str = "\x1b[?47h";
/// Enables alternative buffer
pub const ENABLE_ALTERNATIVE_BUFFER: &str = "\x1b[?1049h";
/// Disables the laternative buffer
pub const DISABLE_ALTERNATIVE_BUFFER: &str = "\x1b[?1049l";

// Other
/// Full terminal reset. Clear the screen, buffer, reset all modes, ...
pub const FULL_RESET: &str = "\x1bc";

/// Request the device attributes.
pub const REQUEST_DEVICE_ATTRIBUTES: &str = "\x1b[c";
/// Request the device status.
pub const REQUEST_STATUS_REPORT: &str = "\x1b[5n";
/// Request the current cursor position. In some terminals, the report may be
/// ambigous with F3 key press with modifiers.
pub const REQUEST_CURSOR_POSITION: &str = "\x1b[6n";
/// Request the current cursor position. Difference from
/// [`REQUEST_CURSOR_POSITION`] is that the response is not ambigous, but it is
/// not supported by some terminals that support [`REQUEST_CURSOR_POSITION`].
pub const REQUEST_CURSOR_POSITION2: &str = "\x1b[?6n";
/// Requests the terminal name and version.
pub const REQUEST_TERMINAL_NAME: &str = "\x1b[>0q";
/// Request the text area size of terminal in pixels.
pub const REQUEST_TEXT_AREA_SIZE_PX: &str = "\x1b[14t";
/// Request size of single character on creen in pixels.
pub const REQUEST_CHAR_SIZE: &str = "\x1b[16t";
/// Request size of the text area in characters.
pub const REQUEST_TEXT_AREA_SIZE: &str = "\x1b[18t";
/// Request the number of sixel color registers.
pub const REQUEST_SIXEL_COLORS: &str = "\x1b[1;1;1S";

/// Enables mouse tracking for X and Y coordinate on press.
pub const ENABLE_MOUSE_XY_TRACKING: &str = "\x1b[?9h";
/// Disables mouse tracking for X and Y coordinate on press.
pub const DISABLE_MOUSE_XY_TRACKING: &str = "\x1b[?9l";
/// Enables mouse tracking for X and Y coordinate on press and release.
pub const ENABLE_MOUSE_XY_PR_TRACKING: &str = "\x1b[?1000h";
/// Disables mouse tracking for X and Y coordinate on press and release.
pub const DISABLE_MOUSE_XY_PR_TRACKING: &str = "\x1b[?1000l";
/// Enables mouse tracking for X and Y coordinate on press, release and drag.
pub const ENABLE_MOUSE_XY_DRAG_TRACKING: &str = "\x1b[?1002h";
/// Disables mouse tracking for X and Y coordinate on press, release and drag.
pub const DISABLE_MOUSE_XY_DRAG_TRACKING: &str = "\x1b[?1002l";
/// Enables mouse tracking for X and Y coordinate on press, release, drag and
/// move.
pub const ENABLE_MOUSE_XY_ALL_TRACKING: &str = "\x1b[?1002h";
/// Disables mouse tracking for X and Y coordinate on press, release, drag and
/// move.
pub const DISABLE_MOUSE_XY_ALL_TRACKING: &str = "\x1b[?1002l";
/// Enables sending event on focus gain.
pub const ENABLE_FOCUS_EVENT: &str = "\x1b[?1004h";
/// Disables sending event on focus gain.
pub const DISABLE_FOCUS_EVENT: &str = "\x1b[?1004l";
/// Enables extension to send mouse inputs in format extended to utf8 two byte
/// characters.
pub const ENABLE_MOUSE_XY_UTF8_EXT: &str = "\x1b[?1005h";
/// Disables extension to send mouse inputs in format extended to utf8 two byte
/// characters.
pub const DISABLE_MOUSE_XY_UTF8_EXT: &str = "\x1b[?1005l";
/// Enables extension to send mouse inputs in different format as position in
/// characters.
pub const ENABLE_MOUSE_XY_EXT: &str = "\x1b[?1006h";
/// Disables extension to send mouse inputs in different format as position in
/// characters.
pub const DISABLE_MOUSE_XY_EXT: &str = "\x1b[?1006l";
/// Enables URXVT mouse extension. Not recommended, rather use
/// [`ENABLE_MOUSE_XY_EXT`].
pub const ENABLE_MOUSE_XY_URXVT_EXT: &str = "\x1b[?1015h";
/// Disables URXVT mouse extension.
pub const DISABLE_MOUSE_XY_URXVT_EXT: &str = "\x1b[?1015l";
/// Enables extension to send mouse inputs in different format as position in
/// pixels.
pub const ENABLE_MOUSE_XY_PIX_EXT: &str = "\x1b[?1016h";
/// Disables extension to send mouse inputs in different format as position in
/// pixels.
pub const DISABLE_MOUSE_XY_PIX_EXT: &str = "\x1b[?1016l";

csi_macro! {
    scroll_region, t, b; 'r'
        ? "Set the scroll region in the terminal. Also moves the cursor to the
           top left."
}

/// Reset the scroll region
pub const RESET_SCROLL_REGION: &str = "\x1b[0;0r";
/// Don't limit the printing area.
pub const DONT_LIMIT_PRINT_TO_SCROLL_REGION: &str = "\x1b[19h";
/// Limit printing area only to scroll region.
pub const LIMIT_PRINT_TO_SCROLL_REGION: &str = "\x1b[19l";

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
        CursorStyle::Block(Some(true)) => "\x1b[0 q",
        CursorStyle::Block(None) => "\x1b[1 q",
        CursorStyle::Block(Some(false)) => "\x1b[2 q",
        CursorStyle::Underline(true) => "\x1b[3 q",
        CursorStyle::Underline(false) => "\x1b[4 q",
        CursorStyle::Bar(true) => "\x1b[5 q",
        CursorStyle::Bar(false) => "\x1b[6 q",
    }
}

/*#[macro_export]
macro_rules! resize_window {
    ($x:expr, $y:expr) => {
        $crate::csi!('t', 8, $y, $x)
    };
}*/

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
