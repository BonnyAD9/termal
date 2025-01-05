use std::{borrow::Cow, cell::Cell, fmt::Display};

mod term_text_metadata;
mod term_text_span;
mod term_text_spans;

pub use self::{term_text_metadata::*, term_text_span::*, term_text_spans::*};

/// String with control escape sequences.
///
/// Can be used to extract/strip the control sequences or to get some
/// information about how the text will be displayed. All the information is
/// cached as much as possible.
#[derive(Debug, Clone)]
pub struct TermText<'a> {
    text: Cow<'a, str>,
    metadata: Cell<Option<TermTextMetadata>>,
}

impl<'a> TermText<'a> {
    /// Creates new [`TermText`].
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            metadata: Cell::default(),
        }
    }

    /// Creates new [`TermText`] with the given metadata.
    ///
    /// # Safety
    /// This is marked unsafe because the chached metadata has to be valid in
    /// safe rust.
    pub unsafe fn from_metadata(
        text: impl Into<Cow<'a, str>>,
        metadata: TermTextMetadata,
    ) -> Self {
        Self {
            text: text.into(),
            metadata: Cell::new(Some(metadata)),
        }
    }

    /// Create new [`TermText`] and immidietely cache the metadata.
    pub fn chached(text: impl Into<Cow<'a, str>>) -> Self {
        let res = Self::new(text);
        res.get_metadata();
        res
    }

    /// Create reference to this term text.
    pub fn reference<'b>(&'b self) -> TermText<'b> {
        TermText {
            text: self.as_str().into(),
            metadata: Cell::new(self.metadata.get()),
        }
    }

    /// Get the stored string.
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Get the metadata, but only if it is available.
    pub fn try_get_metadata(&self) -> Option<TermTextMetadata> {
        self.metadata.get()
    }

    /// Get the metadata. If it is not availeble it will be calculated.
    #[inline]
    pub fn get_metadata(&self) -> TermTextMetadata {
        if let Some(md) = self.metadata.get() {
            md
        } else {
            let md = TermTextMetadata::from_text(&self.text);
            self.metadata.set(Some(md));
            md
        }
    }

    /// Get the total number of bytes.
    pub fn byte_cnt(&self) -> usize {
        self.text.len()
    }

    /// Get the total number of chars. If it is not cached it will be
    /// calculated.
    pub fn char_cnt(&self) -> usize {
        self.get_metadata().chars
    }

    /// Get the number of display characters. If it is not cached it will be
    /// calculated.
    pub fn display_char_cnt(&self) -> usize {
        self.get_metadata().display_chars()
    }

    /// Get the number of display bytes. If it is not cached it will be
    /// calculated.
    pub fn display_bytes_cnt(&self) -> usize {
        let meta = self.get_metadata();
        self.byte_cnt() - meta.control_bytes
    }

    /// Get the number of control characters. If it is not cached it will be
    /// calculated.
    pub fn control_char_cnt(&self) -> usize {
        self.get_metadata().control_chars
    }

    /// Get the number of control bytes. If it is not cached, it will be
    /// calculated.
    pub fn control_bytes_cnt(&self) -> usize {
        self.get_metadata().control_bytes
    }

    /// Get iterator over the spans of the control string. Single span contains
    /// either plain text or single control sequence.
    pub fn spans(&self) -> TermTextSpans<'_> {
        TermTextSpans::new(&self.text)
    }

    /// Strips the string of control sequences
    #[inline]
    pub fn strip_control(&self) -> String {
        let mut res = String::new();
        if let Some(meta) = self.metadata.get() {
            res.reserve_exact(self.text.len() - meta.control_bytes);
            for span in self.spans().filter(|s| !s.is_control()) {
                res.push_str(span.text());
            }
        } else {
            let mut meta = TermTextMetadata::default();

            for span in self.spans().filter(|s| !s.is_control()) {
                res.push_str(span.text());
                meta.chars += span.chars();
                if span.is_control() {
                    meta.control_chars += span.chars();
                    meta.control_bytes += span.text().len();
                }
            }
        }

        res
    }

    /// Converts the text to string. This will also cache the metadata if it is
    /// not already cached. To avoid caching use `.as_str().to_string()`
    pub fn to_string_cache(&self) -> String {
        if self.metadata.get().is_some() {
            self.to_string()
        } else {
            let mut res = String::with_capacity(self.text.len());

            let mut meta = TermTextMetadata::default();
            for span in self.spans() {
                meta.chars += span.chars();
                res.push_str(span.text());
                if span.is_control() {
                    meta.control_bytes += span.text().len();
                    meta.control_chars += span.chars();
                }
            }

            res
        }
    }

    /// Get the unerlying [`Cow`]
    pub fn as_cow(&self) -> &Cow<'a, str> {
        &self.text
    }

    /// Get owned version of the term text.
    pub fn to_owned(self) -> TermText<'static> {
        match self.text {
            Cow::Owned(s) => TermText {
                text: Cow::Owned(s),
                metadata: self.metadata,
            },
            Cow::Borrowed(_) => {
                let text = self.to_string();
                TermText {
                    text: Cow::Owned(text),
                    metadata: self.metadata,
                }
            }
        }
    }
}

impl AsRef<str> for TermText<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for TermText<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for TermText<'_> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<'a> From<&'a str> for TermText<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl<'a> From<TermText<'a>> for String {
    fn from(value: TermText<'a>) -> Self {
        value.text.into()
    }
}

impl<'a> From<TermText<'a>> for Cow<'a, str> {
    fn from(value: TermText<'a>) -> Self {
        value.text
    }
}

impl Default for TermText<'_> {
    fn default() -> Self {
        Self {
            text: Default::default(),
            metadata: Cell::new(Some(Default::default())),
        }
    }
}
