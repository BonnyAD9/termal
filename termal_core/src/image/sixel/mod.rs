mod sixel_state;

use sixel_state::SixelState;

use super::Image;

#[derive(Default)]
struct Sixel([u8; 6]);

impl Sixel {
    fn from_img(img: &impl Image, (x, y): (usize, usize)) -> Self {
        let mut data = [Default::default(); 6];

        for yo in y..img.height().min(y + 6) {
            data[yo - y] = img.get_pixel(x, yo).to_332();
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
pub fn push_sixel(out: &mut String, img: &impl Image) {
    let mut state = SixelState::new(img, out);
    state.encode();
}
