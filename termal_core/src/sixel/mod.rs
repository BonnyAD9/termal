mod mat;
mod raw_img;
mod rgb;
mod sixel_image;
mod sixel_state;

use sixel_state::SixelState;

pub use self::{mat::*, raw_img::*, rgb::*, sixel_image::*};

#[derive(Default)]
struct Sixel([u8; 6]);

impl Sixel {
    fn from_img(img: &impl SixelImage, (x, y): (usize, usize)) -> Self {
        let mut data = [Default::default(); 6];

        for yo in y..img.sixel_height().min(y + 6) {
            data[yo - y] = img.sixel_get_pixel(x, yo).to_332();
        }

        Self(data)
    }

    fn color_char(&self, rgb: u8) -> char {
        let mut code: u8 = 0;
        for (i, c) in self.0.iter().copied().enumerate() {
            if c == rgb {
                code |= 1 << i;
            }
        }

        (code + 63) as char
    }
}

/// Generate sixel image and append it to the string `out`.
pub fn push_sixel(out: &mut String, img: &impl SixelImage) {
    let mut state = SixelState::new(img, out);
    state.encode();
}
