use crate::term_text::TermText;

/// Configuration for terminal reader.
#[derive(Debug, Clone, Default)]
pub struct ReadConf<'a> {
    /// What thext should be edited. Empty by default.
    pub edit: Vec<char>,
    /// Position of cursor within the edited text. End by default.
    pub edit_pos: Option<usize>,
    /// Prompt for the input. Empty by default.
    pub prompt: TermText<'a>,
}
