use super::{Image, Rgb};

/// Image with owned raw RGB data.
pub struct RawImg {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl RawImg {
    /// Create raw image from owned raw rgb data.
    ///
    /// # Panic
    /// - If the data size doesn't match the width and size.
    pub fn from_rgb(data: Vec<u8>, width: usize, height: usize) -> Self {
        if width * height * 3 != data.len() {
            panic!(
                "Invalid raw image data length of {} for \
                [{width}, {height}]({})",
                data.len(),
                width * height
            );
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl Image for RawImg {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_pixel(&self, x: usize, y: usize) -> Rgb {
        let pos = (self.width * y + x) * 3;
        (self.data[pos], self.data[pos + 1], self.data[pos + 2]).into()
    }
}
