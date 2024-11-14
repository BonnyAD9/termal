use crate::{
    codes,
    sixel::{Mat, Rgb, SixelImage},
};

use super::Texel;

pub struct TexelState<'a, I>
where
    I: SixelImage,
{
    img: &'a I,
    texw: usize,
    texh: usize,
    res: Mat<Texel>,
}

impl<'a, I> TexelState<'a, I>
where
    I: SixelImage,
{
    pub fn new(img: &'a I, w: usize, h: usize) -> Self {
        let texw = img.sixel_width() / w;
        let texh = img.sixel_height() / h;
        Self {
            img,
            texw,
            texh,
            res: Mat::new(w, h),
        }
    }

    pub fn append(&mut self, res: &mut String, nl: &str) {
        self.process();
        for y in 0..self.res.height() {
            for x in 0..self.res.width() {
                self.res[(x, y)].append_to(res);
            }
            *res += codes::RESET;
            *res += nl;
        }
    }

    fn process(&mut self) {
        for y in 0..self.res.height() {
            for x in 0..self.res.width() {
                self.res[(x, y)] =
                    self.get_texel(x * self.texw, y * self.texh);
            }
        }
    }

    fn get_texel(&self, x: usize, y: usize) -> Texel {
        let half = self.texh / 2;
        let top = self.get_avg(x, y, self.texw, half);
        let bot = self.get_avg(x, y + half, self.texw, self.texh - half);
        Texel { top, bot }
    }

    fn get_avg(&self, x: usize, y: usize, w: usize, h: usize) -> Rgb {
        let mut res: Rgb<usize> = Rgb::default();

        for y in y..(y + h).min(self.img.sixel_height()) {
            for x in x..(x + w).min(self.img.sixel_width()) {
                res += self.img.sixel_get_pixel(x, y);
            }
        }

        res /= w * h;
        res.as_u8()
    }
}
