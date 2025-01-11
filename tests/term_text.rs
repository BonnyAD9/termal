use std::borrow::Cow;

use termal::{formatc, term_text::{TermText, TermTextSpan}};

#[test]
fn test_term_text() {
    let s = formatc!("Text{'r}íček{'_}");
    let text = TermText::new(&s);

    assert_eq!(text.as_str(), s);
    assert_eq!(text.as_cow(), &Cow::Borrowed(&s));

    assert_eq!(text.byte_cnt(), s.len());
    assert_eq!(text.char_cnt(), s.chars().count());
    assert_eq!(text.control_bytes_cnt(), 9);
    assert_eq!(text.control_char_cnt(), 9);
    assert_eq!(text.display_bytes_cnt(), 10);
    assert_eq!(text.display_char_cnt(), 8);

    fn sf(txt: &TermText, f: impl Fn(&TermTextSpan) -> bool) -> String {
        txt.spans().filter(f).flat_map(|s| s.text().chars()).collect::<String>()
    }

    assert_eq!(sf(&text, |_| true), s);
    assert_eq!(sf(&text, |_| false), "");
    assert_eq!(sf(&text, |c| c.is_control()), formatc!("{'r}{'_}"));
    assert_eq!(sf(&text, |c| !c.is_control()), "Textíček");
}
