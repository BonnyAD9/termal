mod mat;
mod raw_img;
mod rgb;
mod sixel;
mod texel;

pub use self::{mat::*, raw_img::*, rgb::*, sixel::*, texel::*};

/// Image data that can be interpreted when generating sixel data.
pub trait Image {
    /// Width of the image in pixels.
    fn width(&self) -> usize;

    /// Height of the image in pixels.
    fn height(&self) -> usize;

    /// Gets pixel at the given coordinates.
    fn get_pixel(&self, x: usize, y: usize) -> Rgb;

    fn get_avg(&self, x: f32, y: f32, w: f32, h: f32) -> Rgb {
        let mut color_sum: Rgb<f32> = Rgb::default();
        let mut val_sum = 0.0_f32;

        let r = x + w;
        let b = y + h;

        let xfr = x.fract();
        let yfr = y.fract();
        let rfr = r.fract();
        let bfr = b.fract();
        let xfl = x.floor();
        let yfl = y.floor();
        let rc = r.ceil();
        let bc = b.ceil();

        let ixfr = 1. - xfr;
        let iyfr = 1. - yfr;

        let mut corner = |x: (f32, f32), y: (f32, f32)| {
            let mul = x.0 * y.0;
            val_sum += mul;
            color_sum +=
                self.get_pixel(x.1 as usize, y.1 as usize).as_f32() * mul;
        };

        let xs = (ixfr, xfl);
        let xe = (rfr, rc);
        let ys = (iyfr, yfl);
        let ye = (bfr, bc);

        corner(xs, ys);
        corner(xe, ys);
        corner(xs, ye);
        corner(xe, ye);

        // TODO

        (color_sum / val_sum).as_u8()
    }
}
