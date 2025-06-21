use std::time::Duration;

use common::BufProvider;
use termal::{
    Error,
    raw::{Terminal, events::AmbiguousEvent},
};

mod common;

#[test]
fn test_basic() {
    let mut t = Terminal::new(BufProvider::new(&[b"ab", b"cd"]));
    assert!(!t.has_buffered_input());
    assert!(t.has_input());
    assert_eq!(t.read_byte().unwrap(), b'a');
    assert!(t.has_buffered_input());
    assert!(t.has_input());
    assert_eq!(t.read_byte().unwrap(), b'b');
    assert!(!t.has_buffered_input());
    assert!(t.has_input());
    assert_eq!(t.read_byte().unwrap(), b'c');
    assert!(t.has_buffered_input());
    assert!(t.has_input());
    assert_eq!(t.read_byte().unwrap(), b'd');
    assert!(!t.has_buffered_input());
    assert!(t.has_input());
    assert!(matches!(t.read_byte(), Err(Error::StdInEof)));
    assert!(!t.has_buffered_input());
    assert!(t.has_input()); // Eof is input
}

#[test]
fn test_read_raw() {
    let mut t = Terminal::new(BufProvider::new(&[b"ab", b"cd"]));
    let mut res = [0; 10];
    assert_eq!(t.read_raw(&mut res).unwrap(), 4);
    assert_eq!(&res[..4], b"abcd");
    assert!(matches!(t.read_raw(&mut res), Err(Error::StdInEof)));
}

#[test]
fn test_read_timeout() {
    let mut t = Terminal::new(BufProvider::new(&[b"ab", b"cd"]));
    let mut res = [0; 10];
    assert_eq!(t.read_raw_timeout(&mut res, Duration::ZERO).unwrap(), 4);
    assert_eq!(&res[..4], b"abcd");
    assert!(matches!(
        t.read_raw_timeout(&mut res, Duration::ZERO),
        Err(Error::StdInEof)
    ));
}

#[test]
fn test_read_single_timeout() {
    let mut t = Terminal::new(BufProvider::new(&[b"ab", b"cd"]));
    let mut res = [0; 10];
    assert_eq!(
        t.read_raw_single_timeout(&mut res, Duration::ZERO).unwrap(),
        4
    );
    assert_eq!(&res[..4], b"abcd");
    assert!(matches!(
        t.read_raw_single_timeout(&mut res, Duration::ZERO),
        Err(Error::StdInEof)
    ));
}

#[test]
fn test_read_line() {
    let mut t = Terminal::new(BufProvider::eof_panic(0, &[b"hello there\r"]));
    assert_eq!(t.read_line().unwrap(), "hello there");
    let mut t = Terminal::new(BufProvider::eof_panic(1, &[b"hello there"]));
    assert_eq!(t.read_line().unwrap(), "hello there");
    let mut t = Terminal::new(BufProvider::eof_panic(2, &[b""]));
    assert_eq!(t.read_line().unwrap(), "");
    assert_eq!(t.read_line().unwrap(), "");
}

#[test]
fn test_edit_line() {
    let mut t = Terminal::new(BufProvider::eof_panic(0, &[b"\x1b[Hhello \r"]));
    assert_eq!(t.edit_line("there").unwrap(), "hello there");
}

#[test]
fn test_events() {
    let mut t = Terminal::new(BufProvider::new(&[
        b"h\x1b",
        b"\x1b\x1b\x1bc\xc5\xa1\x1b[1;5H\r\x1b[200~\x1b\rh\x1b[201~",
        b"h\x1b[>>H\x1b[M\x20\x28\x2F\x1b]52;;aGVsbG8gdGhlcmU=\x1b\\l",
    ]));
    assert_eq!(t.read_ambiguous().unwrap(), AmbiguousEvent::from_code(b"h"));
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b")
    );
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b\x1b")
    );
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1bc")
    );
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code("š".as_bytes())
    );
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b[1;5H")
    );
    assert_eq!(t.read_ambiguous().unwrap(), AmbiguousEvent::from_code(b"\r"));
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b[200~")
    );
    assert_eq!(t.read_ambiguous().unwrap(), AmbiguousEvent::verbatim('\x1b'));
    assert_eq!(t.read_ambiguous().unwrap(), AmbiguousEvent::verbatim('\n'));
    assert_eq!(t.read_ambiguous().unwrap(), AmbiguousEvent::verbatim('h'));
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b[201~")
    );
    assert_eq!(t.read_ambiguous().unwrap(), AmbiguousEvent::from_code(b"h"));
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b[>>H")
    );
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b[M\x20\x28\x2F")
    );
    assert_eq!(
        t.read_ambiguous().unwrap(),
        AmbiguousEvent::from_code(b"\x1b]52;;aGVsbG8gdGhlcmU=\x1b\\")
    );
    assert_eq!(t.read_ambiguous().unwrap(), AmbiguousEvent::from_code(b"l"));
    assert!(matches!(t.read_ambiguous(), Err(Error::StdInEof)));
}
