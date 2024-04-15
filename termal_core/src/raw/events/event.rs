pub struct AmbigousEvent {
    pub event: AnyEvent,
    pub other: Vec<Vec<Event>>,
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
    Multiply,
    Add,
    Comma,
    Minus,
    Delete,
    Divide,
    Insert,
    End,
    Home,
    PgUp,
    PgDown,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Backspace,
    Esc,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Exclamation,
    DoubleQuote,
    Pound,
    Dollar,
    Percent,
    Quote,
    OpenParent,
    CloseParent,
    Dot,
    Colon,
    Semicolon,
    Less,
    More,
    Question,
    At,
    OpenBracket,
    Backslash,
    CloseBracket,
    Hat,
    Underscore,
    Backtick,
    OpenBrace,
    Pipe,
    CloseBrace,
    Tilde,
    Other(char),
}

impl AmbigousEvent {
    pub fn from_char_code(code: char) -> Self {
        Self::char_key(code)
    }

    pub fn from_code(code: &[u8]) -> Self {
        AmbigousEvent {
            event: AnyEvent::Unknown(code.into()),
            other: vec![],
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
            '\0' => amb.push(vec![]),
            '\x08' => amb.push(vec![Event::KeyPress(Key {
                key_char: None,
                code: KeyCode::Backspace,
                modifiers: Modifiers::CONTROL,
            })]),
            '\x09' => amb.push(vec![Event::KeyPress(Key {
                key_char: None,
                code: KeyCode::I,
                modifiers: Modifiers::CONTROL,
            })]),
            '\x0d' => amb.push(vec![Event::KeyPress(Key {
                key_char: None,
                code: KeyCode::M,
                modifiers: Modifiers::CONTROL,
            })]),
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
        match chr {
            ' ' | '\0' => KeyCode::Space,
            '\t' => KeyCode::Tab,
            '\r' => KeyCode::Enter,
            '*' => KeyCode::Multiply,
            '+' => KeyCode::Add,
            ',' => KeyCode::Comma,
            '-' => KeyCode::Minus,
            '/' => KeyCode::Divide,
            '0' => KeyCode::Num0,
            '1' => KeyCode::Num1,
            '2' => KeyCode::Num2,
            '3' => KeyCode::Num3,
            '4' => KeyCode::Num4,
            '5' => KeyCode::Num5,
            '6' => KeyCode::Num6,
            '7' => KeyCode::Num7,
            '8' => KeyCode::Num8,
            '9' => KeyCode::Num9,
            '\x7f' => KeyCode::Backspace,
            '\x1b' => KeyCode::Esc,
            'A' | 'a' | '\x01' => KeyCode::A,
            'B' | 'b' | '\x02' => KeyCode::B,
            'C' | 'c' | '\x03' => KeyCode::C,
            'D' | 'd' | '\x04' => KeyCode::D,
            'E' | 'e' | '\x05' => KeyCode::E,
            'F' | 'f' | '\x06' => KeyCode::F,
            'G' | 'g' | '\x07' => KeyCode::G,
            'H' | 'h' | '\x08' => KeyCode::H,
            'I' | 'i' => KeyCode::I,
            'J' | 'j' | '\x0a' => KeyCode::J,
            'K' | 'k' | '\x0b' => KeyCode::K,
            'L' | 'l' | '\x0c' => KeyCode::L,
            'M' | 'm' => KeyCode::M,
            'N' | 'n' | '\x0e' => KeyCode::N,
            'O' | 'o' | '\x0f' => KeyCode::O,
            'P' | 'p' | '\x10' => KeyCode::P,
            'Q' | 'q' | '\x11' => KeyCode::Q,
            'R' | 'r' | '\x12' => KeyCode::R,
            'S' | 's' | '\x13' => KeyCode::S,
            'T' | 't' | '\x14' => KeyCode::T,
            'U' | 'u' | '\x15' => KeyCode::U,
            'V' | 'v' | '\x16' => KeyCode::V,
            'W' | 'w' | '\x17' => KeyCode::W,
            'X' | 'x' | '\x18' => KeyCode::X,
            'Y' | 'y' | '\x19' => KeyCode::Y,
            'Z' | 'z' | '\x1A' => KeyCode::Z,
            '!' => KeyCode::Exclamation,
            '"' => KeyCode::DoubleQuote,
            '#' => KeyCode::Pound,
            '$' => KeyCode::Dollar,
            '%' => KeyCode::Percent,
            '\'' => KeyCode::Quote,
            '(' => KeyCode::OpenParent,
            ')' => KeyCode::CloseParent,
            '.' => KeyCode::Dot,
            ':' => KeyCode::Colon,
            ';' => KeyCode::Semicolon,
            '<' => KeyCode::Less,
            '>' => KeyCode::More,
            '?' => KeyCode::Question,
            '@' => KeyCode::At,
            '[' => KeyCode::OpenBracket,
            '\\' => KeyCode::Backslash,
            ']' => KeyCode::CloseBracket,
            '^' => KeyCode::Hat,
            '_' => KeyCode::Underscore,
            '`' => KeyCode::Backtick,
            '{' => KeyCode::OpenBrace,
            '|' => KeyCode::Pipe,
            '}' => KeyCode::CloseBrace,
            '~' => KeyCode::Tilde,
            c => KeyCode::Other(c),
        }
    }
}
