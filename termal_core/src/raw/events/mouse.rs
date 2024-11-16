use super::Modifiers;

bitflags::bitflags! {
    #[doc = "Key modifiers. Some of them are usualy not sent to terminals."]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub(crate) struct State: u32 {
        const PRIMARY = 0x0;
        const MIDDLE = 0x1;
        const SECONDARY = 0x2;
        const RELEASE = 0x3;
        const BUTTON = 0x3;
        const SHIFT = 0x4;
        const ALT = 0x8;
        const CONTROL = 0x10;
        const MODIFIERS = 0x1C;
        const MOVE = 0x20;
        const SCROLL_UP = 0x40;
        const SCROLL_DOWN = 0x41;
        const SCROLL = 0x41;
        const ACTION = 0x43;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    None,
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Down,
    Up,
    ScrollUp,
    ScrollDown,
    Move,
}

#[derive(Copy, Clone, Debug)]
pub struct Mouse {
    pub button: Button,
    pub event: Event,
    pub modifiers: Modifiers,
    pub x: usize,
    pub y: usize,
}

impl Mouse {
    pub fn from_data(
        state: u32,
        x: usize,
        y: usize,
        down: Option<bool>,
    ) -> Self {
        let state = State::from_bits_retain(state);
        let button = state.into();
        let event = if state.contains(State::MOVE) {
            Event::Move
        } else if (state & State::SCROLL) == State::SCROLL_UP {
            Event::ScrollUp
        } else if (state & State::SCROLL) == State::SCROLL_DOWN {
            Event::ScrollDown
        } else if down.unwrap_or(button != Button::None) {
            Event::Down
        } else {
            Event::Up
        };
        let modifiers = state.into();

        Self {
            button,
            event,
            modifiers,
            x,
            y,
        }
    }
}

impl From<State> for Button {
    fn from(value: State) -> Self {
        match value & State::BUTTON {
            State::RELEASE => Self::None,
            State::PRIMARY => Self::Left,
            State::MIDDLE => Self::Middle,
            State::SECONDARY => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl From<State> for Modifiers {
    fn from(value: State) -> Self {
        Modifiers::from_bits_retain((value & State::MODIFIERS).bits() >> 2)
    }
}
