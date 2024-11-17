use super::TermAttr;

#[derive(Debug, Clone)]
pub enum Status {
    Attributes(TermAttr),
    Ok,
    CursorPosition { x: usize, y: usize },
    TerminalName(String),
}
