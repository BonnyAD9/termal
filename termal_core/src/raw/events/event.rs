use crate::raw::events::csi::Csi;

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

#[derive(Debug, Clone, Copy, Hash)]
pub struct Key {
    pub key_char: Option<char>,
    pub code: KeyCode,
    pub modifiers: Modifiers,
}

bitflags::bitflags!{
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Modifiers: u32 {
        const NONE = 0x0;
        const SHIFT = 0x1;
        const ALT = 0x2;
        const CONTROL = 0x4;
        const META = 0x8;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Up,
    Down,
    Right,
    Left,
    Space,
    Tab,
    Enter,
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    Delete,
    Divide,
    Insert,
    End,
    Home,
    PgUp,
    PgDown,
    Backspace,
    Esc,
    Char(char),
}

impl AmbigousEvent {
    pub fn from_char_code(code: char) -> Self {
        Self::char_key(code)
    }

    pub fn from_code(code: &[u8]) -> Self {
        if let Ok(s) = std::str::from_utf8(code) {
            Self::from_code_str(s)
        } else {
            AmbigousEvent {
                event: AnyEvent::Unknown(code.into()),
                other: vec![],
            }
        }

    }

    fn from_code_str(code: &str) -> Self {
        if code.is_empty() {
            return Self {
                event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                other: vec![],
            };
        }

        if code.len() == 1 {
            let chr = code.chars().next().unwrap();
            return Self::char_key(chr);
        }

        if code.len() == 2 && code.starts_with('\x1b') {
            let chr = code.chars().last().unwrap();
            let mut res = Self::from_char_code(chr);
            if let AnyEvent::Known(Event::KeyPress(ref mut k)) = res.event {
                k.modifiers |= Modifiers::ALT;
            }

            for k in res.other.iter_mut() {
                let Event::KeyPress(k) = k;
                k.modifiers |= Modifiers::ALT;
            }

            return res;
        }

        let Some(cscode) = code.strip_prefix("\x1b[") else {
            if let Some(cscode) = code.strip_prefix("\x1bO") {
                let csi = Csi::parse(cscode);
                if !matches!(csi.postfix.as_str(), "P" | "Q" | "R" | "S") {
                    return Self {
                        event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                        other: vec![],
                    };
                }


                let pchr = csi.postfix.chars().next().unwrap();
                return match csi.args.as_slice() {
                    [] | [ 1 ] => {
                        if let Some(k) = KeyCode::from_xterm_id(pchr) {
                            Self {
                                event: AnyEvent::Known(Event::KeyPress(Key {
                                    key_char: None,
                                    code: k,
                                    modifiers: Modifiers::NONE,
                                })),
                                other: vec![],
                            }
                        } else {
                            Self {
                                event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                                other: vec![],
                            }
                        }
                    }
                    [ 1, m ] => {
                        if let Some(k) = KeyCode::from_xterm_id(pchr) {
                            Self {
                                event: AnyEvent::Known(Event::KeyPress(Key {
                                    key_char: None,
                                    code: k,
                                    modifiers: Modifiers::from_id(*m),
                                })),
                                other: vec![],
                            }
                        } else {
                            Self {
                                event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                                other: vec![],
                            }
                        }
                    }
                    _ => Self {
                        event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                        other: vec![],
                    },
                }
            }
            return Self {
                event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                other: vec![],
            };
        };

        let csi = Csi::parse(cscode);
        if !csi.prefix.is_empty() {
            return Self {
                event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                other: vec![],
            };
        }

        if csi.postfix == "~" {
            return match csi.args.as_slice() {
                [ k ] => {
                    if let Some(k) = KeyCode::from_vt_id(*k) {
                        Self {
                            event: AnyEvent::Known(Event::KeyPress(Key {
                                key_char: None,
                                code: k,
                                modifiers: Modifiers::NONE,
                            })),
                            other: vec![],
                        }
                    } else {
                        Self {
                            event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                            other: vec![],
                        }
                    }
                }
                [ k, m ] => {
                    if let Some(k) = KeyCode::from_vt_id(*k) {
                        Self {
                            event: AnyEvent::Known(Event::KeyPress(Key {
                                key_char: None,
                                code: k,
                                modifiers: Modifiers::from_id(*m),
                            })),
                            other: vec![],
                        }
                    } else {
                        Self {
                            event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                            other: vec![],
                        }
                    }
                }
                _ => Self {
                    event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                    other: vec![],
                },
            }
        }

        if csi.postfix.len() != 1 {
            return Self {
                event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                other: vec![],
            };
        }

        let pchr = csi.postfix.chars().next().unwrap();

        match csi.args.as_slice() {
            [] | [ 1 ] => {
                if let Some(k) = KeyCode::from_xterm_id(pchr) {
                    Self {
                        event: AnyEvent::Known(Event::KeyPress(Key {
                            key_char: None,
                            code: k,
                            modifiers: Modifiers::NONE,
                        })),
                        other: vec![],
                    }
                } else {
                    Self {
                        event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                        other: vec![],
                    }
                }
            }
            [ 1, m ] => {
                if let Some(k) = KeyCode::from_xterm_id(pchr) {
                    Self {
                        event: AnyEvent::Known(Event::KeyPress(Key {
                            key_char: None,
                            code: k,
                            modifiers: Modifiers::from_id(*m),
                        })),
                        other: vec![],
                    }
                } else {
                    Self {
                        event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                        other: vec![],
                    }
                }
            }
            _ => Self {
                event: AnyEvent::Unknown(code.as_bytes().to_owned()),
                other: vec![],
            },
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

        if chr >= '\0' && chr <= '\x19' && chr != '\x09' && chr != '\x0d' {
            key.modifiers |= Modifiers::CONTROL;
        }

        if chr.is_ascii_control() && chr != '\t' {
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

impl KeyCode {
    pub fn from_char(chr: char) -> Self {
        match chr.to_ascii_lowercase() {
            ' ' | '\0' => Self::Space,
            '\t' => Self::Tab,
            '\r' => Self::Enter,
            '/' => Self::Divide,
            '\x7f' => Self::Backspace,
            '\x1b' => Self::Esc,
            'a' | '\x01' => Self::Char('a'),
            'b' | '\x02' => Self::Char('b'),
            'c' | '\x03' => Self::Char('c'),
            'd' | '\x04' => Self::Char('d'),
            'e' | '\x05' => Self::Char('e'),
            'f' | '\x06' => Self::Char('f'),
            'g' | '\x07' => Self::Char('g'),
            'h' | '\x08' => Self::Char('h'),
            'i' => Self::Char('i'),
            'j' | '\x0a' => Self::Char('j'),
            'k' | '\x0b' => Self::Char('k'),
            'l' | '\x0c' => Self::Char('l'),
            'm' => Self::Char('m'),
            'n' | '\x0e' => Self::Char('n'),
            'o' | '\x0f' => Self::Char('o'),
            'p' | '\x10' => Self::Char('p'),
            'q' | '\x11' => Self::Char('q'),
            'r' | '\x12' => Self::Char('r'),
            's' | '\x13' => Self::Char('s'),
            't' | '\x14' => Self::Char('t'),
            'u' | '\x15' => Self::Char('u'),
            'v' | '\x16' => Self::Char('v'),
            'w' | '\x17' => Self::Char('w'),
            'x' | '\x18' => Self::Char('x'),
            'y' | '\x19' => Self::Char('y'),
            'z' | '\x1A' => Self::Char('z'),
            c => Self::Char(c),
        }
    }

    pub fn from_vt_id(id: u32) -> Option<Self> {
        match id {
            1 => Some(Self::Home),
            2 => Some(Self::Insert),
            3 => Some(Self::Delete),
            4 => Some(Self::End),
            5 => Some(Self::PgUp),
            6 => Some(Self::PgDown),
            7 => Some(Self::Home),
            8 => Some(Self::End),
            10 => Some(Self::F0),
            11 => Some(Self::F1),
            12 => Some(Self::F2),
            13 => Some(Self::F3),
            14 => Some(Self::F4),
            15 => Some(Self::F5),
            17 => Some(Self::F6),
            18 => Some(Self::F7),
            19 => Some(Self::F8),
            20 => Some(Self::F9),
            21 => Some(Self::F10),
            23 => Some(Self::F11),
            24 => Some(Self::F12),
            25 => Some(Self::F13),
            26 => Some(Self::F14),
            28 => Some(Self::F15),
            29 => Some(Self::F16),
            31 => Some(Self::F17),
            32 => Some(Self::F18),
            33 => Some(Self::F19),
            34 => Some(Self::F20),
            _ => None,
        }
    }

    pub fn from_xterm_id(id: char) -> Option<Self> {
        match id {
            'A' => Some(Self::Up),
            'B' => Some(Self::Down),
            'C' => Some(Self::Right),
            'D' => Some(Self::Left),
            'F' => Some(Self::End),
            'G' => Some(Self::from_char('5')),
            'H' => Some(Self::Home),
            'P' => Some(Self::F1),
            'Q' => Some(Self::F2),
            'R' => Some(Self::F3),
            'S' => Some(Self::F4),
            _ => None,
        }
    }
}

impl Modifiers {
    pub fn from_id(id: u32) -> Self {
        Modifiers::from_bits_retain(id - 1)
    }
}
