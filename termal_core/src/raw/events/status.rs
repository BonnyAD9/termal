use crate::Rgb;

use super::TermAttr;

/// Status event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Terminal attributes.
    Attributes(TermAttr),
    /// Status OK
    Ok,
    /// Cursor position report.
    CursorPosition { x: usize, y: usize },
    /// Terminal name report.
    TerminalName(String),
    /// Size of text area in pixels report.
    TextAreaSizePx { w: usize, h: usize },
    /// Size of text area in characters report.
    TextAreaSize { w: usize, h: usize },
    /// Size of character in pixels.
    CharSize { w: usize, h: usize },
    /// Number of supported sixel colors.
    SixelColors(usize),
    /// Maximum size of sixel image.
    SixelSize { w: usize, h: usize },
    /// Color of the given color code.
    ColorCodeColor { code: u8, color: Rgb<u16> },
    /// Default foreground color.
    DefaultFgColor(Rgb<u16>),
    /// Default background color.
    DefaultBgColor(Rgb<u16>),
    /// Cursor color.
    CursorColor(Rgb<u16>),
    /// Data from selection.
    SelectionData(Vec<u8>),
}
