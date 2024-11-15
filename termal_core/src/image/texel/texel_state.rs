use crate::{
    codes,
    image::{Image, Mat},
};

use super::Texel;

pub struct TexelState<'a, I>
where
    I: Image,
{
    img: &'a I,
    texw: f32,
    texh: f32,
    res: Mat<Texel>,
}

impl<'a, I> TexelState<'a, I>
where
    I: Image,
{
    pub fn new(img: &'a I, w: usize, h: usize) -> Self {
        let texw = img.width() as f32 / w as f32;
        let texh = img.height() as f32 / h as f32;
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
                self.res[(x, y)] = self.get_texel(x, y);
            }
        }
    }

    fn get_texel(&self, x: usize, y: usize) -> Texel {
        let x = x as f32 * self.texw;
        let y = y as f32 * self.texh;
        let half = self.texh / 2.;
        let top = self.img.get_avg(x, y, self.texw, half);
        let bot = self.img.get_avg(x, y + half, self.texw, self.texh - half);
        Texel {
            top: top.as_u8(),
            bot: bot.as_u8(),
        }
    }
}
