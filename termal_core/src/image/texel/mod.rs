use texel_state::TexelState;

use crate::codes::{bg, fg};

use super::{Image, Rgb};

mod texel_state;

#[derive(Debug, Default)]
pub struct Texel {
    pub top: Rgb,
    pub bot: Rgb,
}

impl Texel {
    pub fn append_to(&self, r: &mut String) {
        *r += &fg!(self.bot.r, self.bot.g, self.bot.b);
        *r += &bg!(self.top.r, self.top.g, self.top.b);
        r.push('▄');
    }
}

pub fn draw_blocks(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: usize,
    h: usize,
) {
    let mut state = TexelState::new(img, w, h);
    state.append(res, nl);
}
