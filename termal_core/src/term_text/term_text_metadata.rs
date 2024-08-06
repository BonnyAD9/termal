use super::TermTextSpans;

/// Information about text with control sequences.
#[derive(Debug, Copy, Clone, Default)]
pub struct TermTextMetadata {
    /// Total number of chars.
    pub chars: usize,
    /// Number of chars from control sequences.
    pub control_chars: usize,
    /// Number of bytes from control sequences.
    pub control_bytes: usize,
}

impl TermTextMetadata {
    /// Get the metadata from the given text.
    pub fn from_text(text: &str) -> Self {
        let mut res = TermTextMetadata::default();
        res.add_length(text);
        res
    }

    /// Add to the metadata with metadata of the given text.
    pub fn add_length(&mut self, text: &str) {
        for span in TermTextSpans::new(text) {
            self.chars += span.chars();
            if span.is_control() {
                self.control_chars += span.chars();
                self.control_bytes += span.text().len();
            }
        }
    }

    /// Get the number of display characters.
    pub fn display_chars(&self) -> usize {
        self.chars - self.control_chars
    }
}
