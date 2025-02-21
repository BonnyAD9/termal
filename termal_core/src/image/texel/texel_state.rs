use crate::{
    Rgb, codes,
    image::{Image, Rect},
};

use super::Texel;

/// State when generating texel image.
pub(super) struct TexelState<'a, I>
where
    I: Image,
{
    img: &'a I,
    texw: f32,
    texh: f32,
    w: usize,
    h: usize,
}

impl<'a, I> TexelState<'a, I>
where
    I: Image,
{
    /// Create new texel image state.
    pub fn new(img: &'a I, w: usize, h: usize) -> Self {
        let texw = img.width() as f32 / w as f32;
        let texh = img.height() as f32 / h as f32;
        Self {
            img,
            texw,
            texh,
            w,
            h,
        }
    }

    /// Append texel image with half chars to the string `res`.
    pub fn append_half(&mut self, res: &mut String, nl: &str) {
        self.append(res, nl, Self::get_half_texel);
    }

    /// Append texel image with quater chars to the string `res`.
    pub fn append_quater(&mut self, res: &mut String, nl: &str) {
        self.append(res, nl, Self::get_quater_texel);
    }

    fn append(
        &mut self,
        res: &mut String,
        nl: &str,
        get_texel: impl Fn(&Self, usize, usize) -> Texel,
    ) {
        for y in 0..self.h - 1 {
            for x in 0..self.w {
                get_texel(self, x, y).append_to(res);
            }
            *res += codes::RESET;
            *res += nl;
        }

        for x in 0..self.w {
            get_texel(self, x, self.h - 1).append_to(res);
        }
    }

    fn get_half_texel(&self, x: usize, y: usize) -> Texel {
        let x = x as f32 * self.texw;
        let y = y as f32 * self.texh;
        let half = self.texh / 2.;
        let top = self.img.get_avg(Rect::new(x, y, self.texw, half));
        let bot = self.img.get_avg(Rect::new(
            x,
            y + half,
            self.texw,
            self.texh - half,
        ));
        Texel {
            bg: top.as_u8(),
            fg: bot.as_u8(),
            chr: '▄',
        }
    }

    fn get_quater_texel(&self, x: usize, y: usize) -> Texel {
        let chrs = [
            ('▄', [0, 0, 1, 1]),
            ('▖', [0, 0, 1, 0]),
            ('▗', [0, 0, 0, 1]),
            ('▘', [1, 0, 0, 0]),
            ('▝', [0, 1, 0, 0]),
            ('▌', [1, 0, 1, 0]),
            ('▚', [1, 0, 0, 1]),
        ];

        chrs.into_iter()
            .map(|(c, d)| self.score_quater_texel(x, y, c, d))
            .min_by_key(|(s, _)| *s as usize)
            .unwrap()
            .1
    }

    fn score_quater_texel(
        &self,
        x: usize,
        y: usize,
        chr: char,
        desc: [usize; 4],
    ) -> (f32, Texel) {
        let x = x as f32 * self.texw;
        let y = y as f32 * self.texh;
        let wh = self.texw / 2.;
        let hh = self.texh / 2.;

        let mut sum = [Rgb::<f32>::default(); 2];
        let mut cnt = [0; 2];

        let vals = [
            self.img.get_avg(Rect::new(x, y, wh, hh)),
            self.img.get_avg(Rect::new(x + wh, y, wh, hh)),
            self.img.get_avg(Rect::new(x, y + hh, wh, hh)),
            self.img.get_avg(Rect::new(x + wh, y + hh, wh, hh)),
        ];

        for (v, d) in vals.iter().zip(&desc) {
            sum[*d] += *v;
            cnt[*d] += 1;
        }

        sum[0] /= cnt[0] as f32;
        sum[1] /= cnt[1] as f32;

        let mut diff = Rgb::<f32>::default();

        for (v, d) in vals.iter().zip(&desc) {
            diff += (*v - sum[*d]).abs();
        }

        (
            diff.sum(),
            Texel {
                bg: sum[0].as_u8(),
                fg: sum[1].as_u8(),
                chr,
            },
        )
    }
}
