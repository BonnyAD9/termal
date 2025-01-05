/// Configuration for terminal reader.
#[derive(Debug, Clone, Default)]
pub struct ReadConf {
    /// What thext should be edited. Empty by default.
    pub edit: Vec<char>,
    /// Position of cursor within the edited text. End by default.
    pub edit_pos: Option<usize>,
}
