pub enum Event {
    KeyPress(Key),
}

pub struct Key {
    key_char: Option<char>,
    code: KeyCode,
    modifiers: Modifiers,
}

#[derive(Debug, Eq, PartialEq)]
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
    Other(char),
}

bitflags::bitflags!{
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Modifiers: u32 {
        const SHIFT = 0x1;
        const ALT = 0x2;
        const CONTROL = 0x4;
        const META = 0x8;
    }
}
