use super::TermTextSpan;

/// Iterator over spans of string.
pub struct TermTextSpans<'a> {
    text: &'a str,
}

impl<'a> TermTextSpans<'a> {
    /// Craete new iterator over spans of string.
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Get the remaining spans as a string.
    pub fn as_str(&self) -> &'a str {
        self.text
    }
}

impl<'a> Iterator for TermTextSpans<'a> {
    type Item = TermTextSpan<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            None
        } else {
            let (span, text) = TermTextSpan::create(self.text);
            self.text = text;
            Some(span)
        }
    }
}
