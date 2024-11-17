mod readers;
mod sys;
mod terminal;

pub use readers::*;
pub use sys::*;
pub use terminal::*;

pub mod events;

fn utf8_code_len(first: u8) -> usize {
    if (first & 0x80) == 0 {
        1
    } else if (first & 0xE0) == 0xC0 {
        2
    } else if (first & 0xF0) == 0xE0 {
        3
    } else if (first & 0xF8) == 0xF0 {
        4
    } else {
        0
    }
}
