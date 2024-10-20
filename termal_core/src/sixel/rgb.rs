/// Single RGB pixel.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgb {
    /// Red component of the pixel.
    pub r: u8,
    /// Green component of the pixel.
    pub g: u8,
    /// Blue component of the pixel.
    pub b: u8,
}

impl Rgb {
    /// Create new rgb pixel.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create new rgb pixel from single byte rgb pixel.
    ///
    /// The single byte has the components (from high bits to low bits):
    /// - Red: 3
    /// - Green: 3
    /// - Blue: 2
    pub fn from_332(c: u8) -> Self {
        let mut r = c >> 5;
        r = (r << 5) | (r << 2) | (r >> 1);

        let mut g = (c >> 2) & 7;
        g = (g << 5) | (g << 2) | (g >> 1);

        let mut b = c & 3;
        b |= b << 2;
        b |= b << 4;

        Self::new(r, g, b)
    }

    /// Convert this pixel to a single byte RGB value.
    ///
    /// The single byte has the components (from high bits to low bits):
    /// - Red: 3
    /// - Green: 3
    /// - Blue: 2
    pub fn to_332(&self) -> u8 {
        (self.r & 0b11100000) | ((self.g >> 3) & 0b11100) | (self.b >> 6)
    }

    /// Get new pixel with the given range of values. (from 0 to `max`).
    pub fn to_range(&self, max: u8) -> Self {
        self.map(|n| (n as usize * max as usize / 255) as u8)
    }

    /// Create new RGB pixel by transforming its components with `f`.
    pub fn map(&self, f: impl Fn(u8) -> u8) -> Self {
        Self::new(f(self.r), f(self.g), f(self.b))
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}
