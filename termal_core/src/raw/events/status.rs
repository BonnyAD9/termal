use super::TermAttr;

#[derive(Debug, Clone)]
pub enum Status {
    Attributes(TermAttr),
    Ok,
    CursorPosition { x: usize, y: usize },
    TerminalName(String),
    TextAreaSizePx { w: usize, h: usize },
    TextAreaSize { w: usize, h: usize },
    CharSize { w: usize, h: usize },
    SixelColors(usize),
    SixelSize { w: usize, h: usize },
}
