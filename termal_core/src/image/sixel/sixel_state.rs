use std::collections::BTreeSet;

use crate::{Rgb, image::Image};

use super::Sixel;

/// State when generating sixel image.
pub(super) struct SixelState<'a, I>
where
    I: Image,
{
    line: Vec<Sixel>,
    img: &'a I,
    out: &'a mut String,
}

impl<'a, I> SixelState<'a, I>
where
    I: Image,
{
    /// Create new sixel state. Output will be appended to `out`. To actually
    /// generate the sixel data, call `encode`.
    pub fn new(img: &'a I, out: &'a mut String) -> Self {
        Self {
            line: Vec::with_capacity(img.width()),
            img,
            out,
        }
    }

    /// Generate the sixel data and append it to the output.
    pub fn encode(&mut self) {
        *self.out += "\x1bPq";

        self.define_colors();

        for y in 0..(self.img.height() / 6) {
            self.get_line(y);
            self.draw_line();
        }

        *self.out += "\x1b\\";
    }

    fn get_line(&mut self, y: usize) {
        self.line.clear();
        for x in 0..self.img.width() {
            self.line.push(Sixel::from_img(self.img, (x, y * 6)));
        }
    }

    fn define_colors(&mut self) {
        for i in 1..=255 {
            let (r, g, b) =
                Rgb::<u8>::from_332(i).norm_to_range(0, 100).into();
            *self.out += &format!("#{i};2;{r};{g};{b}");
        }
    }

    fn draw_line(&mut self) {
        let mut line_colors = BTreeSet::new();
        for sx in &self.line {
            line_colors.extend(sx.0);
        }

        for c in line_colors {
            *self.out += &format!("#{c}");
            for sx in &self.line {
                self.out.push(sx.color_char(c));
            }
            self.out.push('$');
        }

        self.out.push('-');
    }
}
