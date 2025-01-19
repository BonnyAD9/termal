/// Change the state of the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateChange {
    /// Paste has started. Treat the input verbatim.
    BracketedPasteStart,
    /// Paste has ended. Stop treating the input verbatim.
    BracketedPasteEnd,
}
