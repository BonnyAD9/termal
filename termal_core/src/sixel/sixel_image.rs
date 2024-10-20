use super::Rgb;

/// Image data that can be interpreted when generating sixel data.
pub trait SixelImage {
    /// Width of the image in pixels.
    fn sixel_width(&self) -> usize;

    /// Height of the image in pixels.
    fn sixel_height(&self) -> usize;

    /// Gets pixel at the given coordinates.
    fn sixel_get_pixel(&self, x: usize, y: usize) -> Rgb;
}
