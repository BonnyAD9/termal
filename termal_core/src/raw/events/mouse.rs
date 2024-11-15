bitflags::bitflags! {
    #[doc = "Key modifiers. Some of them are usualy not sent to terminals."]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub(crate) struct MouseState: u32 {
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
        const ACTION = 0x43;
    }
}
