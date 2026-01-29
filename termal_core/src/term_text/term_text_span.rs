use unicode_width::UnicodeWidthStr;

use crate::codes;

/// Span of single plain text or single control sequence. Note that all
/// whitespace except space `' '` is treated as control sequence.
pub struct TermTextSpan<'a> {
    text: &'a str,
    columns: usize,
    chars: usize,
    control: bool,
}

impl<'a> TermTextSpan<'a> {
    /// Get the text of the span.
    pub fn text(&self) -> &'a str {
        self.text
    }

    /// Get the number of chars in the span. This is cached.
    pub fn chars(&self) -> usize {
        self.chars
    }

    /// Get the number of columns the text will take up on screen.
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Check if the span contains either control sequnece or plain text.
    pub fn is_control(&self) -> bool {
        self.control
    }

    /// Create new span from the start of the given string.
    ///
    /// # Returns
    /// The span and rest of the string.
    pub fn create(text: &str) -> (TermTextSpan<'_>, &str) {
        let found = text
            .char_indices()
            .chain([(usize::MAX, '\0')])
            .enumerate()
            .find(|(_, (_, c))| c.is_ascii_control());
        let Some((idx, (ind, chr))) = found else {
            unreachable!();
        };

        // The whole text is not control sequence
        if ind == usize::MAX && chr == '\0' {
            return (
                TermTextSpan {
                    text,
                    columns: text.width(),
                    chars: idx,
                    control: false,
                },
                "",
            );
        }

        // The first part is not control sequence
        if ind != 0 {
            return Self::split_from(text, ind, idx, false);
        }

        // Single char control sequence
        if chr != codes::ESC {
            return Self::split_from(text, 1, 1, true);
        }

        // Only the escape char. This is invalid escape sequence but it is a
        // control sequence.
        let Some(chr) = text.chars().nth(1) else {
            return Self::split_from(text, 1, 1, true);
        };

        match chr as u32 {
            // DCS | OSC | PM | APC
            0x50 | 0x5d | 0x5e | 0x5f => Self::end_with_str(text, "\x1b\x5c"),
            // CSI
            0x5b => Self::end_with_pat(text, 2, |c| {
                (0x40..0x7f).contains(&(c as u32))
            }),
            // SS2 | SS3
            0x4e | 0x4f => Self::split_from(text, 3, 3, true),
            // Two char C1 escape sequence
            0x40..=0x5f => Self::split_from(text, 2, 2, true),
            // Invalid escape sequence
            _ => Self::split_from(text, 1, 1, true),
        }
    }

    fn split_from(
        text: &str,
        ind: usize,
        chars: usize,
        control: bool,
    ) -> (TermTextSpan<'_>, &str) {
        let columns = if control { 0 } else { text[..ind].width() };
        (
            TermTextSpan {
                text: &text[..ind],
                columns,
                chars,
                control,
            },
            &text[ind..],
        )
    }

    fn end_with_str<'b>(
        text: &'b str,
        pat: &str,
    ) -> (TermTextSpan<'b>, &'b str) {
        if let Some(p) = text.find(pat) {
            let ind = p + pat.len();
            Self::split_from(text, ind, text[..ind].chars().count(), true)
        } else {
            (
                TermTextSpan {
                    text,
                    columns: 0,
                    chars: text.chars().count(),
                    control: true,
                },
                "",
            )
        }
    }

    fn end_with_pat(
        text: &str,
        skip: usize,
        f: impl Fn(char) -> bool,
    ) -> (TermTextSpan<'_>, &str) {
        let end = text[skip..]
            .char_indices()
            .chain([(usize::MAX, '0')])
            .enumerate()
            .find(|(_, (_, c))| f(*c));
        let Some((idx, (ind, c))) = end else {
            unreachable!();
        };

        if ind == usize::MAX && c == '0' {
            // sequence is missing the final character
            (
                TermTextSpan {
                    text,
                    columns: 0,
                    chars: idx + skip,
                    control: true,
                },
                "",
            )
        } else {
            Self::split_from(
                text,
                ind + c.len_utf8() + skip,
                idx + 1 + skip,
                true,
            )
        }
    }
}
