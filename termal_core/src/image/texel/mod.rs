use std::mem;

use texel_state::TexelState;

use crate::{
    Rgb,
    codes::{bg, fg},
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
        *r += &fg!(self.fg.r(), self.fg.g(), self.fg.b());
        *r += &bg!(self.bg.r(), self.bg.g(), self.bg.b());
        r.push(self.chr);
    }

    pub fn disallowed_bg(&mut self, bg: Rgb) {
        if self.bg == bg {
            if self.bg == self.fg {
                self.chr = '█';
            } else {
                self.swap_char();
            }
        }
    }

    pub fn swap_char(&mut self) {
        self.chr = match self.chr {
            '▄' => '▀',
            '▀' => '▄',
            ' ' => '█',
            '█' => ' ',
            '▐' => '▌',
            '▌' => '▐',
            '▜' => '▖',
            '▖' => '▜',
            '▛' => '▗',
            '▗' => '▛',
            '▟' => '▘',
            '▘' => '▟',
            '▙' => '▝',
            '▝' => '▙',
            '▞' => '▚',
            '▚' => '▞',
            _ => return,
        };
        mem::swap(&mut self.fg, &mut self.bg);
    }
}

/// Append image `img` from half block characters (`▄`) to the buffer `res`.
/// `nl` is used for new lines of the image. `w` and `h` is size of the image
/// in characters. If `w` or `h` is not specified, it is calculated. If neither
/// is specified, it is as if `w` was `80`.
///
/// When calculating `w` or `h` it is expected that each character is twice as
/// tall as wide.
///
/// This method of printing images is supported by any terminal that supports
/// printing with ansi colors. If you would like a little better resolution,
/// you can use [`push_texel_quater`] which uses more types of characters to
/// better approximate the image but may be a little slower to calculate.
///
/// Some terminals (e.g. kitty) will treat background color same as the
/// logical background color as if there was no backgroun color. This may be
/// issue when the terminal background is set to image or transparent. To avoid
/// using some specific color as backgroun color use [`push_texel_half_no_bg`].
///
/// # Example
/// ```no_run
/// use termal_core::{codes, image::{RawImg, push_texel_half}};
///
/// let mut buf = codes::CLEAR.to_string();
///
/// let img_data = include_bytes!("../../../../examples/img2_256.data");
/// let img = RawImg::from_rgb(img_data.into(), 256, 256);
///
/// push_texel_half(&img, &mut buf, "\n", Some(60), None);
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/image/push_texel_half.png)
pub fn push_texel_half(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: Option<usize>,
    h: Option<usize>,
) {
    push_texel_half_inner(img, res, nl, w, h, None);
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
    push_texel_quater_inner(img, res, nl, w, h, None);
}

/// Append image `img` from half block characters (`▄`) to the buffer `res`.
/// `nl` is used for new lines of the image. `w` and `h` is size of the image
/// in characters. If `w` or `h` is not specified, it is calculated. If neither
/// is specified, it is as if `w` was `80`.
///
/// When calculating `w` or `h` it is expected that each character is twice as
/// tall as wide.
///
/// The given color will not be used as background color.
pub fn push_texel_half_no_bg(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: Option<usize>,
    h: Option<usize>,
    bg: impl Into<Rgb>,
) {
    push_texel_quater_inner(img, res, nl, w, h, Some(bg.into()));
}

/// Append image `img` from quater block characters (`▄`, `▖`, `▗`, `▘`, `▝`,
/// `▌` and `▚`) to the buffer `res`. `nl` is used for new lines of the image.
/// `w` and `h` is size of the image in characters. If `w` or `h` is not
/// specified, it is calculated. If neither is specified, it is as if `w` was
/// `80`.
///
/// When calculating `w` or `h` it is expected that each character is twice as
/// tall as wide.
///
/// The given color will not be used as background color.
pub fn push_texel_quater_no_bg(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: Option<usize>,
    h: Option<usize>,
    bg: impl Into<Rgb>,
) {
    push_texel_quater_inner(img, res, nl, w, h, Some(bg.into()));
}

fn push_texel_half_inner(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: Option<usize>,
    h: Option<usize>,
    disallowed_bg: Option<Rgb>,
) {
    let (w, h) = get_wh(img, w, h);
    let mut state = TexelState::new(img, w, h);
    state.disallowed_bg(disallowed_bg);
    state.append_half(res, nl);
}

fn push_texel_quater_inner(
    img: &impl Image,
    res: &mut String,
    nl: &str,
    w: Option<usize>,
    h: Option<usize>,
    disallowed_bg: Option<Rgb>,
) {
    let (w, h) = get_wh(img, w, h);
    let mut state = TexelState::new(img, w, h);
    state.disallowed_bg(disallowed_bg);
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
