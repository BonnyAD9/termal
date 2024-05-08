use crate::raw::events::csi::Csi;

use super::{Key, KeyCode, Modifiers};

pub struct AmbigousEvent {
    pub event: AnyEvent,
    pub other: Vec<Event>,
}

pub enum AnyEvent {
    Known(Event),
    Unknown(Vec<u8>),
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Event {
    KeyPress(Key),
}

impl AmbigousEvent {
    pub fn unknown<B>(data: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        AmbigousEvent {
            event: AnyEvent::Unknown(data.into()),
            other: vec![],
        }
    }

    pub fn event(evt: Event) -> Self {
        AmbigousEvent {
            event: AnyEvent::Known(evt),
            other: vec![],
        }
    }

    pub fn key(key: Key) -> Self {
        Self {
            event: AnyEvent::Known(Event::KeyPress(key)),
            other: vec![],
        }
    }

    pub fn from_char_code(code: char) -> Self {
        Self::char_key(code)
    }

    pub fn from_code(code: &[u8]) -> Self {
        std::str::from_utf8(code)
            .ok()
            .and_then(Self::from_code_str)
            .unwrap_or_else(|| Self::unknown(code))
    }

    fn from_code_str(code: &str) -> Option<Self> {
        if code.is_empty() {
            return None;
        }

        // shouldn't really happen
        if code.len() == 1 {
            return code.chars().next().map(Self::char_key);
        }

        // ALT code
        if code.len() == 2 && code.starts_with('\x1b') {
            let chr = code.chars().last().unwrap();
            let mut res = Self::from_char_code(chr);
            if let AnyEvent::Known(Event::KeyPress(ref mut k)) = res.event {
                k.modifiers |= Modifiers::ALT;
                k.key_char = None;
            }

            for k in res.other.iter_mut() {
                let Event::KeyPress(k) = k;
                k.modifiers |= Modifiers::ALT;
                k.key_char = None;
            }

            return Some(res);
        }

        // check if it is CSI
        let Some(cscode) = code.strip_prefix("\x1b[") else {
            // This is bug in some terminal for F1 - F4 keys
            return code.strip_prefix("\x1bO").and_then(|cscode| {
                let csi = Csi::parse(cscode);
                matches!(csi.postfix.as_str(), "P" | "Q" | "R" | "S")
                    .then(|| Self::csi_xterm(csi))
                    .flatten()
            });
        };

        let csi = Csi::parse(cscode);
        if !csi.prefix.is_empty() {
            return None;
        }

        if csi.postfix == "~" {
            return Self::csi_vt(csi);
        }

        (csi.postfix.len() == 1)
            .then(|| Self::csi_xterm(csi))
            .flatten()
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
