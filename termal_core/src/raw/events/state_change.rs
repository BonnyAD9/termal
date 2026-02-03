/// Change the state of the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateChange {
    /// Paste has started. Treat the input verbatim.
    ///
    /// Can be received only if enabled with
    /// [`crate::codes::ENABLE_BRACKETED_PASTE_MODE`].
    BracketedPasteStart,
    /// Paste has ended. Stop treating the input verbatim.
    ///
    /// Can be received only if enabled with
    /// [`crate::codes::ENABLE_BRACKETED_PASTE_MODE`].
    BracketedPasteEnd,
}
