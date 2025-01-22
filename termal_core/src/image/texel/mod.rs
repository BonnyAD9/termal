use texel_state::TexelState;

use crate::{
    codes::{bg, fg},
    Rgb,
};

use super::Image;

mod texel_state;

#[derive(Debug, Default)]
struct Texel {
    pub fg: Rgb,
    pub bg: Rgb,
    pub chr: char,
}

impl Texel {
    pub fn append_to(&self, r: &mut String) {
        *r += &fg!(self.fg.r, self.fg.g, self.fg.b);
        *r += &bg!(self.bg.r, self.bg.g, self.bg.b);
        r.push(self.chr);
    }
}

/// Append image `img` from half block characters (`▄`) to the buffer `res`.
/// `nl` is used for new lines of the image. `w` and `h` is size of the image
/// in characters. If `w` or `h` is not specified, it is calculated. If neither
/// is specified, it is as if `w` was `80`.
///
/// When calculating `w` or `h` it is expected that each character is twice as
/// tall as wide.
pub fn push_texel_half(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: Option<usize>,
    h: Option<usize>,
) {
    let (w, h) = get_wh(img, w, h);
    let mut state = TexelState::new(img, w, h);
    state.append_half(res, nl);
}

/// Append image `img` from quater block characters (`▄`, `▖`, `▗`, `▘`, `▝`,
/// `▌` and `▚`) to the buffer `res`. `nl` is used for new lines of the image.
/// `w` and `h` is size of the image in characters. If `w` or `h` is not
/// specified, it is calculated. If neither is specified, it is as if `w` was
/// `80`.
///
/// When calculating `w` or `h` it is expected that each character is twice as
/// tall as wide.
pub fn push_texel_quater(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: Option<usize>,
    h: Option<usize>,
) {
    let (w, h) = get_wh(img, w, h);
    let mut state = TexelState::new(img, w, h);
    state.append_quater(res, nl);
}

fn get_wh(
    img: &impl Image,
    w: Option<usize>,
    h: Option<usize>,
) -> (usize, usize) {
    const WMUL: usize = 2;
    match (w, h) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => (w, img.height() * w / img.width() / WMUL),
        (None, Some(h)) => (img.width() * h * WMUL / img.height(), h),
        _ => (80, img.height() * 80 / img.width() / WMUL),
    }
}
