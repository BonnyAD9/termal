/// Key press event.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Key {
    /// Char that should be displayed with this key press.
    pub key_char: Option<char>,
    /// The pressed key.
    pub code: KeyCode,
    /// Modifiers that were pressed with the key.
    pub modifiers: Modifiers,
}

bitflags::bitflags! {
    #[doc = "Key modifiers. Some of them are usualy not sent to terminals."]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Modifiers: u32 {
        #[doc = "No modifiers."]
        const NONE = 0x0;
        #[doc = "The shift key."]
        const SHIFT = 0x1;
        #[doc = "The alt key."]
        const ALT = 0x2;
        #[doc = "The control key."]
        const CONTROL = 0x4;
        #[doc = "The meta (windows) key."]
        const META = 0x8;
    }
}

/// Key codes.
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
    Insert,
    End,
    Home,
    PgUp,
    PgDown,
    Backspace,
    Esc,
    /// Any other key coresponding directly to a character.
    Char(char),
}

impl Key {
    /// Create new key from its components.
    pub fn new(code: KeyCode, modifiers: Modifiers, chr: char) -> Self {
        Self {
            code,
            modifiers,
            key_char: Some(chr),
        }
    }

    /// Checks if the key code and modifiers are same.
    pub fn same_key(&self, other: &Key) -> bool {
        self.code == other.code && self.modifiers == other.modifiers
    }

    /// Create new key without key char.
    pub fn mcode(code: KeyCode, modifiers: Modifiers) -> Self {
        Self {
            code,
            modifiers,
            key_char: None,
        }
    }

    /// Create new key without key char and no modifiers.
    pub fn code(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: Modifiers::NONE,
            key_char: None,
        }
    }
}

impl KeyCode {
    /// Create key code from its representative character.
    pub fn from_char(chr: char) -> Self {
        match chr.to_ascii_lowercase() {
            ' ' | '\0' => Self::Space,
            '\t' => Self::Tab,
            '\r' => Self::Enter,
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

    /// Craete key code from VT id.
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

    /// Get key code from xterm id.
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
    /// Get modifiers from their ID.
    pub fn from_id(id: u32) -> Self {
        Modifiers::from_bits_retain(id - 1)
    }
}
