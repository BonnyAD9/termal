//! This module allows generating images that can be displayed in teminal.
//! Images may be displayed either with sixels ([`push_sixel`]) or texels
//! (e.g. [`push_texel_quater`]).
//!
//! Sixels draw the image using special protocol that will draw the image so
//! that one pixel = one pixel on screen. This method is supported only by few
//! terminals. Simple terminals don't support it because it is complicated and
//! new terminals use different protocols for drawing images.
//!
//! Texels use block characters and set the background and foreground color to
//! aproximate the image. They are like low resolution image where single
//! character is ~2-4 pixels. This method is supported in almost every modern
//! terminal, because it is just colored text printing.

mod img_nearest;
mod mat;
mod raw_img;
mod rect;
mod sixel;
mod texel;

use crate::Rgb;

pub use self::{
    img_nearest::*, mat::*, raw_img::*, rect::*, sixel::*, texel::*,
};

/// Image data that can be interpreted when generating sixel data.
pub trait Image {
    /// Width of the image in pixels.
    fn width(&self) -> usize;

    /// Height of the image in pixels.
    fn height(&self) -> usize;

    /// Gets pixel at the given coordinates.
    fn get_pixel(&self, x: usize, y: usize) -> Rgb;

    /// Gets color representing the given rectangle. The implementation may
    /// use any method to get the color.
    ///
    /// The default implementation takes the average over all the pixels in the
    /// given area.
    fn get_avg(&self, rect: Rect) -> Rgb<f32> {
        let mut color_sum: Rgb<usize> = Rgb::default();

        let x = rect.x as usize;
        let y = rect.y as usize;
        let w = (*rect.width() as usize).max(1);
        let h = (*rect.height() as usize).max(1);

        for y in y..y + h {
            for x in x..x + w {
                color_sum += self.get_pixel(x, y).convert::<usize>();
            }
        }

        color_sum.cast::<f32>() / (w * h) as f32
    }
}

#[cfg(feature = "image")]
impl<T: image::GenericImage> Image for T
where
    T::Pixel: image::Pixel<Subpixel = u8>,
{
    fn width(&self) -> usize {
        self.width() as usize
    }

    fn height(&self) -> usize {
        self.height() as usize
    }

    fn get_pixel(&self, x: usize, y: usize) -> Rgb {
        use image::Pixel;

        <Self as image::GenericImageView>::get_pixel(self, x as u32, y as u32)
            .to_rgb()
            .0
            .into()
    }
}
