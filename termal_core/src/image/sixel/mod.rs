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
///
/// Sixels are quite complicated and inefficient. This is why many modern
/// terminals choose to not support sixels and use different protocols for
/// showing images.
///
/// If you don't need the resolution, you can try using texels (e.g.
/// [`crate::image::push_texel_quater`]) which are supported on all terminals
/// that support RGB colors.
///
/// # Example
/// ```no_run
/// use termal_core::{codes, image::{RawImg, push_sixel}};
///
/// let mut buf = codes::CLEAR.to_string();
///
/// let img_data = include_bytes!("../../../../examples/img2_256.data");
/// let img = RawImg::from_rgb(img_data.into(), 256, 256);
///
/// push_sixel(&mut buf, &img);
///
/// println!("{buf}");
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/image/push_sixel.png)
pub fn push_sixel(out: &mut String, img: &impl Image) {
    let mut state = SixelState::new(img, out);
    state.encode();
}
