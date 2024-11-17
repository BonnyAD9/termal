use super::TermAttr;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Attributes(TermAttr),
    Ok,
    CursorPosition { x: usize, y: usize },
}
