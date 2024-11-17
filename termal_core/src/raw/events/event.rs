use crate::{codes, raw::events::csi::Csi};

use super::{mouse::Mouse, Key, KeyCode, Modifiers, Status, TermAttr};

/// Possibly ambiguous terminal event.
///
/// Some terminal events are amiguous. This will contain all sensible
/// possibilities.
#[derive(Clone, Debug)]
pub struct AmbigousEvent {
    /// The main (most propable) event.
    pub event: AnyEvent,
    /// Other amiguous events.
    pub other: Vec<Event>,
}

/// Either known or unknown event.
#[derive(Clone, Debug)]
pub enum AnyEvent {
    /// Known parsed event.
    Known(Event),
    /// Unknown unparsed event.
    Unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum Event {
    /// Key was pressed.
    KeyPress(Key),
    /// Mouse event
    Mouse(Mouse),
    /// Received terminal attributes.
    Status(Status),
    /// The terminal has gained focus.
    Focus,
    /// The terminal has lost focus.
    FocusLost,
}

impl AmbigousEvent {
    /// Create unknown event from the given data.
    pub fn unknown<B>(data: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        AmbigousEvent {
            event: AnyEvent::Unknown(data.into()),
            other: vec![],
        }
    }

    /// Create unambiguous event.
    pub fn event(evt: Event) -> Self {
        Self {
            event: AnyEvent::Known(evt),
            other: vec![],
        }
    }

    /// Create unambiguous key event.
    pub fn key(key: Key) -> Self {
        Self::event(Event::KeyPress(key))
    }

    pub fn mouse(mouse: Mouse) -> Self {
        Self::event(Event::Mouse(mouse))
    }

    pub fn status(status: Status) -> Self {
        Self::event(Event::Status(status))
    }

    /// Parse single char event.
    pub fn from_char_code(code: char) -> Self {
        Self::char_key(code)
    }

    /// Parse the code into event.
    pub fn from_code(code: &[u8]) -> Self {
        if (6..=9).contains(&code.len()) && code.starts_with(b"\x1b[M") {
            return Self::mouse_code(code);
        }

        std::str::from_utf8(code)
            .ok()
            .and_then(Self::from_code_str)
            .unwrap_or_else(|| Self::unknown(code))
    }

    fn mouse_code(code: &[u8]) -> Self {
        if code.len() == 6 && code.starts_with(b"\x1b[M") {
            return AmbigousEvent::mouse(Mouse::from_data(
                code[3] as u32 - 32,
                code[4] as usize - 32,
                code[5] as usize - 32,
                None,
            ));
        }
        let utf_code = std::str::from_utf8(&code[3..])
            .map(|s| s.chars().collect::<Box<[char]>>());
        let Ok([s, x, y]) = utf_code.as_deref() else {
            return Self::unknown(code);
        };
        AmbigousEvent::mouse(Mouse::from_data(
            *s as u32 - 32,
            *x as usize - 32,
            *y as usize - 32,
            None,
        ))
    }

    fn from_code_str(code: &str) -> Option<Self> {
        if code.is_empty() {
            return None;
        }

        let cnt = (code.len() <= 8).then(|| code.chars().count());

        // shouldn't really happen
        if cnt == Some(1) {
            return code.chars().next().map(Self::char_key);
        }

        // ALT code
        if cnt == Some(2) && code.starts_with('\x1b') {
            let chr = code.chars().last().unwrap();
            let mut res = Self::from_char_code(chr);
            if let AnyEvent::Known(Event::KeyPress(ref mut k)) = res.event {
                k.modifiers |= Modifiers::ALT;
                k.key_char = None;
            }

            for k in res.other.iter_mut() {
                let Event::KeyPress(k) = k else {
                    return None;
                };
                k.modifiers |= Modifiers::ALT;
                k.key_char = None;
            }

            return Some(res);
        }

        // check if it is CSI
        if let Some(code) = code.strip_prefix(codes::CSI) {
            Self::csi(code)
        } else if let Some(code) = code.strip_prefix(codes::DCS) {
            Self::dcs(code)
        } else {
            code.strip_prefix("\x1bO").and_then(|cscode| {
                let csi = Csi::parse(cscode);
                matches!(csi.postfix.as_str(), "P" | "Q" | "R" | "S")
                    .then(|| Self::csi_xterm(csi))
                    .flatten()
            })
        }
    }

    fn csi(code: &str) -> Option<Self> {
        match code {
            "I" => return Some(Self::event(Event::Focus)),
            "O" => return Some(Self::event(Event::FocusLost)),
            "0n" => return Some(Self::status(Status::Ok)),
            _ => {}
        }

        let csi = Csi::parse(code);

        match (csi.prefix.as_str(), &csi.args[..], csi.postfix.as_str()) {
            // Ambiguous (F3 with modifiers or specific cursor position)
            ("", [1, x], "R") if *x < 16 => Some(Self {
                event: AnyEvent::Known(Event::KeyPress(Key::mcode(
                    KeyCode::F3,
                    Modifiers::from_id(*x),
                ))),
                other: vec![Event::Status(Status::CursorPosition {
                    x: *x as usize,
                    y: 1,
                })],
            }),
            // Terminal attributes
            ("?", _, "c") => {
                Some(Self::status(Status::Attributes(TermAttr::parse(csi))))
            }
            // Mouse event with the SGR extension
            ("<", [s, x, y], d @ ("M" | "m")) => Some(Self::mouse(
                Mouse::from_data(*s, *x as usize, *y as usize, Some(d == "M")),
            )),
            // Mouse event with the URXVT extension
            ("", [s, x, y], "M") => Some(Self::mouse(Mouse::from_data(
                *s - 32,
                *x as usize,
                *y as usize,
                None,
            ))),
            // Cursor position
            ("" | "?", [y, x], "R") => {
                Some(Self::status(Status::CursorPosition {
                    x: *x as usize,
                    y: *y as usize,
                }))
            }
            // Possibly VT key press
            ("", _, "~") => Self::csi_vt(csi),
            // Possibly xterm key press
            ("", _, post) if post.len() == 1 => Self::csi_xterm(csi),
            _ => None,
        }
    }

    fn dcs(code: &str) -> Option<Self> {
        let code = code.strip_suffix(codes::ST)?;

        code.strip_prefix(">|")
            .map(|name| Self::status(Status::TerminalName(name.into())))
    }

    fn csi_xterm(csi: Csi) -> Option<Self> {
        if csi.postfix.is_empty() {
            return None;
        }

        let pchr = csi.postfix.chars().next().unwrap();
        match csi.args.as_slice() {
            [] | [1] => {
                KeyCode::from_xterm_id(pchr).map(Key::code).map(Self::key)
            }
            [1, m] => KeyCode::from_xterm_id(pchr)
                .map(|k| Self::key(Key::mcode(k, Modifiers::from_id(*m)))),
            _ => None,
        }
    }

    fn csi_vt(csi: Csi) -> Option<Self> {
        match csi.args.as_slice() {
            [k] => KeyCode::from_vt_id(*k).map(Key::code).map(Self::key),
            [k, m] => KeyCode::from_vt_id(*k)
                .map(|k| Self::key(Key::mcode(k, Modifiers::from_id(*m)))),
            _ => None,
        }
    }

    fn char_key(chr: char) -> Self {
        let mut key = Key {
            key_char: Some(chr),
            code: KeyCode::from_char(chr),
            modifiers: Modifiers::NONE,
        };

        if chr.is_uppercase() {
            key.modifiers |= Modifiers::SHIFT;
        }

        if ('\0'..='\x1A').contains(&chr) && chr != '\x09' && chr != '\x0d' {
            key.modifiers |= Modifiers::CONTROL;
        }

        if chr.is_ascii_control() {
            key.key_char = None;
        }

        if chr == '\r' {
            key.key_char = Some('\n');
        }

        let event = Event::KeyPress(key);
        let mut amb = vec![];

        match chr {
            '\x08' => amb.push(Event::KeyPress(Key {
                key_char: None,
                code: KeyCode::Backspace,
                modifiers: Modifiers::CONTROL,
            })),
            '\x09' => amb.push(Event::KeyPress(Key {
                key_char: None,
                code: KeyCode::Char('i'),
                modifiers: Modifiers::CONTROL,
            })),
            '\x0d' => amb.push(Event::KeyPress(Key {
                key_char: None,
                code: KeyCode::Char('i'),
                modifiers: Modifiers::CONTROL,
            })),
            '\x17' => amb.push(Event::KeyPress(Key {
                key_char: None,
                code: KeyCode::Backspace,
                modifiers: Modifiers::CONTROL,
            })),
            _ => {}
        }

        AmbigousEvent {
            event: AnyEvent::Known(event),
            other: amb,
        }
    }
}
