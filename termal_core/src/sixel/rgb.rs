use std::ops::{Add, AddAssign, DivAssign};

/// Single RGB pixel.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgb<T = u8> {
    /// Red component of the pixel.
    pub r: T,
    /// Green component of the pixel.
    pub g: T,
    /// Blue component of the pixel.
    pub b: T,
}

impl<T> Rgb<T> {
    /// Create new rgb pixel.
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl Rgb {
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

impl Rgb<usize> {
    pub fn as_u8(self) -> Rgb<u8> {
        Rgb::new(self.r as u8, self.g as u8, self.b as u8)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl AddAssign<Rgb<u8>> for Rgb<usize> {
    fn add_assign(&mut self, rhs: Rgb<u8>) {
        self.r += rhs.r as usize;
        self.g += rhs.g as usize;
        self.b += rhs.b as usize;
    }
}

impl Add<Rgb<u8>> for Rgb<usize> {
    type Output = Rgb<usize>;

    fn add(mut self, rhs: Rgb<u8>) -> Self::Output {
        self += rhs;
        self
    }
}

impl DivAssign<usize> for Rgb<usize> {
    fn div_assign(&mut self, rhs: usize) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}
