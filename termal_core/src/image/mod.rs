mod img_nearest;
mod mat;
mod raw_img;
mod rect;
mod rgb;
mod sixel;
mod texel;

pub use self::{
    img_nearest::*, mat::*, raw_img::*, rect::*, rgb::*, sixel::*, texel::*,
};

/// Image data that can be interpreted when generating sixel data.
pub trait Image {
    /// Width of the image in pixels.
    fn width(&self) -> usize;

    /// Height of the image in pixels.
    fn height(&self) -> usize;

    /// Gets pixel at the given coordinates.
    fn get_pixel(&self, x: usize, y: usize) -> Rgb;

    fn get_avg(&self, rect: Rect) -> Rgb<f32> {
        let mut color_sum: Rgb<usize> = Rgb::default();

        let x = rect.x as usize;
        let y = rect.y as usize;
        let w = (rect.w as usize).max(1);
        let h = (rect.h as usize).max(1);

        for y in y..y + h {
            for x in x..x + w {
                color_sum += self.get_pixel(x, y);
            }
        }

        color_sum.as_f32() / (w * h) as f32
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
            .into()
    }
}
