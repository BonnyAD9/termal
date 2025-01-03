use bitflags::bitflags;

use super::csi::Csi;

/// Information about terminal.
#[derive(Debug, Copy, Clone)]
pub struct TermAttr {
    /// Type of the terminal. (Which terminal this terminal emulates.)
    pub typ: TermType,
    /// Features of the terminal.
    pub features: TermFeatures,
}

/// Type of simulated terminal.
#[derive(Debug, Copy, Clone)]
pub enum TermType {
    Vt100,
    Vt101,
    Vt132,
    Vt102,
    Vt131,
    Vt125,
    Vt220,
    Vt320,
    Vt420,
    Vt510,
    /// Unknown terminal type id.
    Other(Option<u32>),
}

bitflags! {
    #[doc = "Terminal features."]
    #[derive(Debug, Clone, Copy)]
    pub struct TermFeatures: u32 {
        /// No extra features.
        const NONE = 0x0;
        const COLUMNS132 = 0x1;
        const PRINTER = 0x2;
        const REGIS_GRAPHICS = 0x4;
        /// Supports sixel graphics.
        const SIXEL_GRAPHICS = 0x8;
        const SELECTIVE_ERASE = 0x10;
        const USER_DEFINED_KEYS = 0x20;
        const NATIONAL_REPLACEMENT_CHARACTER_SETS = 0x40;
        const TECHNICAL_CHARACTERS = 0x80;
        const LOCATOR_PORT = 0x100;
        const TERMINAL_STATE_INTERROGATION = 0x200;
        const USER_WINDOWS = 0x400;
        const HORIZONTAL_SCROLLING = 0x800;
        const ANSI_COLOR = 0x1000;
        const RECTANGULAR_EDITING = 0x2000;
        const ANSI_TEXT_LOCATOR = 0x4000;
    }
}

impl TermAttr {
    /// Create new [`TermAttr`] from its components.
    pub fn new(typ: TermType, features: TermFeatures) -> Self {
        Self { typ, features }
    }

    /// Create new [`TermAttr`] from csi code.
    pub(crate) fn parse(csi: Csi) -> Self {
        assert_eq!(csi.prefix, "?");
        assert_eq!(csi.postfix, "c");

        match csi.args[..] {
            [1, 2] => Self::new(TermType::Vt100, TermFeatures::NONE),
            [1, 0] => Self::new(TermType::Vt101, TermFeatures::NONE),
            [4, 6] => Self::new(TermType::Vt132, TermFeatures::NONE),
            [t] => Self::new(TermType::from_id(t), TermFeatures::NONE),
            [t, ref f @ ..] => {
                Self::new(TermType::from_id(t), TermFeatures::from_ids(f))
            }
            [] => Self::new(TermType::Other(None), TermFeatures::NONE),
        }
    }
}

impl TermType {
    /// Get terminal type from its id.
    pub fn from_id(id: u32) -> Self {
        match id {
            6 => Self::Vt102,
            7 => Self::Vt131,
            12 => Self::Vt125,
            62 => Self::Vt220,
            63 => Self::Vt320,
            64 => Self::Vt420,
            65 => Self::Vt510,
            i => Self::Other(Some(i)),
        }
    }
}

impl TermFeatures {
    /// Get terminal features from their ids.
    pub fn from_ids(ids: &[u32]) -> Self {
        ids.iter().fold(Self::NONE, |r, f| r | Self::from_id(*f))
    }

    /// Get terminal feature from its id.
    pub fn from_id(id: u32) -> Self {
        match id {
            1 => Self::COLUMNS132,
            2 => Self::PRINTER,
            3 => Self::REGIS_GRAPHICS,
            4 => Self::SIXEL_GRAPHICS,
            6 => Self::SELECTIVE_ERASE,
            8 => Self::USER_DEFINED_KEYS,
            9 => Self::NATIONAL_REPLACEMENT_CHARACTER_SETS,
            15 => Self::TECHNICAL_CHARACTERS,
            16 => Self::LOCATOR_PORT,
            17 => Self::TERMINAL_STATE_INTERROGATION,
            18 => Self::USER_WINDOWS,
            21 => Self::HORIZONTAL_SCROLLING,
            22 => Self::ANSI_COLOR,
            28 => Self::RECTANGULAR_EDITING,
            29 => Self::ANSI_TEXT_LOCATOR,
            _ => Self::NONE,
        }
    }
}
