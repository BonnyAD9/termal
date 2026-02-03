use crate::Rgb;

use super::TermAttr;

/// Status event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Terminal attributes.
    ///
    /// Sent after request [`crate::codes::REQUEST_DEVICE_ATTRIBUTES`].
    Attributes(TermAttr),
    /// Status OK
    ///
    /// Sent after request [`crate::codes::REQUEST_STATUS_REPORT`].
    Ok,
    /// Cursor position report.
    ///
    /// Sent after request [`crate::codes::REQUEST_CURSOR_POSITION`] or
    /// [`crate::codes::REQUEST_CURSOR_POSITION2`].
    CursorPosition { x: usize, y: usize },
    /// Terminal name report.
    ///
    /// Sent after request [`crate::codes::REQUEST_TERMINAL_NAME`].
    TerminalName(String),
    /// Size of text area in pixels report.
    ///
    /// Sent after request [`crate::codes::REQUEST_TEXT_AREA_SIZE_PX`].
    TextAreaSizePx { w: usize, h: usize },
    /// Size of text area in characters report.
    ///
    /// Sent after request [`crate::codes::REQUEST_TEXT_AREA_SIZE`].
    TextAreaSize { w: usize, h: usize },
    /// Size of character in pixels.
    ///
    /// Sent after request [`crate::codes::REQUEST_CHAR_SIZE`].
    CharSize { w: usize, h: usize },
    /// Number of supported sixel colors.
    ///
    /// Sent after request [`crate::codes::REQUEST_SIXEL_COLORS`].
    SixelColors(usize),
    /// Maximum size of sixel image.
    SixelSize { w: usize, h: usize },
    /// Color of the given color code.
    ///
    /// Sent after request [`crate::codes::request_color_code!`].
    ColorCodeColor { code: u8, color: Rgb<u16> },
    /// Default foreground color.
    ///
    /// Sent after request [`crate::codes::REQUEST_DEFAULT_FG_COLOR`].
    DefaultFgColor(Rgb<u16>),
    /// Default background color.
    ///
    /// Sent after request [`crate::codes::REQUEST_DEFAULT_BG_COLOR`].;
    DefaultBgColor(Rgb<u16>),
    /// Cursor color.
    ///
    /// Sent after request [`crate::codes::REQUEST_CURSOR_COLOR`].
    CursorColor(Rgb<u16>),
    /// Data from selection.
    ///
    /// Sent after request [`crate::codes::REQUEST_SELECTION`] or
    /// [`crate::codes::request_selection`].
    SelectionData(Vec<u8>),
}
