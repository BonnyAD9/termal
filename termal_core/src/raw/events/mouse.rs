use super::Modifiers;

bitflags::bitflags! {
    #[doc = "Key modifiers. Some of them are usualy not sent to terminals."]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub(crate) struct State: u32 {
        /// Primary button (usually left)
        const PRIMARY = 0x0;
        /// Middle button.
        const MIDDLE = 0x1;
        /// Secondary button (usually right)
        const SECONDARY = 0x2;
        /// Event is release.
        const RELEASE = 0x3;
        const SHIFT = 0x4;
        const ALT = 0x8;
        const CONTROL = 0x10;
        /// Modifiers mask.
        const MODIFIERS = 0x1C;
        /// Move event.
        const MOVE = 0x20;
        /// Scroll bit.
        const ANY_SCROLL = 0x40;
        /// Scroll up.
        const SCROLL_UP = 0x40;
        /// Scroll down.
        const SCROLL_DOWN = 0x41;
        /// Scroll mask.
        const SCROLL = 0x41;
        /// Other button bit.
        const OTHER_BTN = 0x80;
        /// Usually the back button on mouse.
        const BUTTON4 = 0x80;
        /// Usually the forward button on mouse.
        const BUTTON5 = 0x81;
        const BUTTON6 = 0x82;
        const BUTTON7 = 0x83;
        /// Button mask.
        const BUTTON = 0xC3;
    }
}

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    None,
    Left,
    Middle,
    Right,
    /// Usually the back button on mouse.
    Button4,
    /// Usually the forward button on mouse.
    Button5,
    Button6,
    Button7,
    Other(u32),
}

/// Mouse events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Down,
    Up,
    ScrollUp,
    ScrollDown,
    Move,
}

/// Mouse event.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mouse {
    /// Button which interacted.
    pub button: Button,
    /// Event of that button.
    pub event: Event,
    /// Keyboard modifiers pressed while the button was down.
    pub modifiers: Modifiers,
    /// X coordinate of mouse (may be either in chars on pixels depending on
    /// mouse mode)
    pub x: usize,
    /// Y coordinate of mouse (may be either in chars on pixels depending on
    /// mouse mode)
    pub y: usize,
}

impl Mouse {
    /// Create new mouse event from mouse event data.
    pub fn from_data(
        state: u32,
        x: usize,
        y: usize,
        down: Option<bool>,
    ) -> Self {
        let state = State::from_bits_retain(state);
        let button = if state.contains(State::ANY_SCROLL) {
            Button::Middle
        } else {
            state.into()
        };
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
            State::MIDDLE | State::SCROLL_DOWN | State::SCROLL_UP => {
                Self::Middle
            }
            State::SECONDARY => Self::Right,
            State::BUTTON4 => Self::Button4,
            State::BUTTON5 => Self::Button5,
            State::BUTTON6 => Self::Button6,
            State::BUTTON7 => Self::Button7,
            v => Self::Other(v.bits()),
        }
    }
}

impl From<State> for Modifiers {
    fn from(value: State) -> Self {
        Modifiers::from_bits_retain((value & State::MODIFIERS).bits() >> 2)
    }
}
