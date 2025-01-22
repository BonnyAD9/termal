use crate::Rgb;

use super::{Image, Rect};

/// Image wrapper that will use the nearest pixel for scaling. This is much
/// faster than the default implementation, but much less precise.
#[derive(Debug, Clone)]
pub struct ImgNearest<I: Image>(pub I);

impl<I: Image> Image for ImgNearest<I> {
    fn width(&self) -> usize {
        self.0.width()
    }

    fn height(&self) -> usize {
        self.0.height()
    }

    fn get_pixel(&self, x: usize, y: usize) -> Rgb {
        self.0.get_pixel(x, y)
    }

    fn get_avg(&self, rect: Rect) -> Rgb<f32> {
        let (x, y) = rect.center();
        self.0.get_pixel(x as usize, y as usize).as_f32()
    }
}
