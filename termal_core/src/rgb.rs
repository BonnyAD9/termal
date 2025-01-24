use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::{codes::fg, error::Error};

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
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }

    /// Checks if all the components match the given condition.
    pub fn all(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        f(&self.r) && f(&self.g) && f(&self.b)
    }

    /// Create new RGB pixel by transforming its components with `f`.
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Rgb<R> {
        Rgb::new(f(self.r), f(self.g), f(self.b))
    }
}

impl Rgb {
    /// Black color.
    pub const BLACK: Self = Self::new(0, 0, 0);

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

    /// Converts the components to [`f32`]. This doesn't scale them in any way.
    pub fn as_f32(self) -> Rgb<f32> {
        Rgb::new(self.r as f32, self.g as f32, self.b as f32)
    }

    /// Get the foreground code of the rgb.
    pub fn fg(&self) -> String {
        fg!(self.r, self.g, self.b)
    }
}

impl Rgb<usize> {
    /// Converts the components to [`u8`].
    pub fn as_u8(self) -> Rgb<u8> {
        Rgb::new(self.r as u8, self.g as u8, self.b as u8)
    }

    /// Converts the components to [`f32`].
    pub fn as_f32(self) -> Rgb<f32> {
        Rgb::new(self.r as f32, self.g as f32, self.b as f32)
    }
}

impl Rgb<f32> {
    /// Black color.
    pub const BLACK: Self = Self::new(0., 0., 0.);

    /// Converts the components to [`u8`].
    pub fn as_u8(self) -> Rgb<u8> {
        Rgb::new(
            self.r.round() as u8,
            self.g.round() as u8,
            self.b.round() as u8,
        )
    }

    /// Gets the absolute value of each component.
    pub fn abs(self) -> Self {
        Self::new(self.r.abs(), self.g.abs(), self.b.abs())
    }

    /// Sums all the components.
    pub fn sum(self) -> f32 {
        self.r + self.g + self.b
    }
}

impl Rgb<u16> {
    pub fn as_u8(&self) -> Rgb {
        self.map(|a| a as u8)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<(f32, f32, f32)> for Rgb<f32> {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::new(r, g, b)
    }
}

macro_rules! impl_assign_rgb {
    ($trait:ident, $fn:ident, $op:tt, $($typ:ty),+ $(,)?) => {
        $(impl $trait<Rgb<$typ>> for Rgb<$typ> {
            fn $fn(&mut self, rhs: Rgb<$typ>) {
                self.r $op rhs.r;
                self.g $op rhs.g;
                self.b $op rhs.b;
            }
        })+
    };
}

macro_rules! impl_assign {
    ($trait:ident, $fn:ident, $op:tt, $($typ:ty),+ $(,)?) => {
        $(impl $trait<$typ> for Rgb<$typ> {
            fn $fn(&mut self, rhs: $typ) {
                self.r $op rhs;
                self.g $op rhs;
                self.b $op rhs;
            }
        })+
    };
}

macro_rules! impl_op_rgb {
    ($trait:ident, $fn:ident, $op:tt, $($typ:ty),+ $(,)?) => {
        $(impl $trait<Rgb<$typ>> for Rgb<$typ> {
            type Output = Rgb<$typ>;

            fn $fn(mut self, rhs: Rgb<$typ>) -> Self::Output {
                self $op rhs;
                self
            }
        })+
    };
}

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $op:tt, $($typ:ty),+ $(,)?) => {
        $(impl $trait<$typ> for Rgb<$typ> {
            type Output = Rgb<$typ>;

            fn $fn(mut self, rhs: $typ) -> Self::Output {
                self $op rhs;
                self
            }
        })+
    };
}

impl_assign_rgb!(AddAssign, add_assign, +=, f32);
impl_assign_rgb!(DivAssign, div_assign, /=, usize);
impl_assign_rgb!(SubAssign, sub_assign, -=, f32);

impl_assign!(DivAssign, div_assign, /=, f32, usize);
impl_assign!(MulAssign, mul_assign, *=, f32);

impl_op_rgb!(Sub, sub, -=, f32);
impl_op_rgb!(Add, add, +=, f32);

impl_op!(Div, div, /=, f32);
impl_op!(Mul, mul, *=, f32);

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

#[cfg(feature = "image")]
impl<T> From<image::Rgb<T>> for Rgb<T> {
    fn from(value: image::Rgb<T>) -> Self {
        let [r, g, b] = value.0;
        Self::new(r, g, b)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { r, g, b } = self;
        if self.all(|c| c.overflowing_shr(4).0 == (c & 0xf)) {
            write!(
                f,
                "rgb:{:x}/{:x}/{:x}",
                r.overflowing_shr(4).0,
                g.overflowing_shr(4).0,
                b.overflowing_shr(4).0
            )
        } else {
            write!(f, "rgb:{r:02x}/{g:02x}/{b:02x}")
        }
    }
}

impl Display for Rgb<u16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { r, g, b } = self;
        if self.all(|c| c.overflowing_shr(8).0 == (c & 0xff)) {
            self.as_u8().fmt(f)
        } else if self.all(|c| c.overflowing_shr(12).0 == (c & 0xf)) {
            write!(
                f,
                "rgb:{:03x}/{:03x}/{:03x}",
                r.overflowing_shr(4).0,
                g.overflowing_shr(4).0,
                b.overflowing_shr(4).0
            )
        } else {
            write!(f, "rgb:{r:04x}/{g:04x}/{b:04x}")
        }
    }
}

impl FromStr for Rgb<u16> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn interpolate(col: &str) -> Result<u16, Error> {
            let c = u16::from_str_radix(col, 16)?;
            match col.len() {
                1 => {
                    let c = c | (c << 4);
                    Ok(c | (c << 8))
                }
                2 => Ok(c | (c << 8)),
                3 => Ok((c << 4) | (c & 0xF)),
                4 => Ok(c),
                _ => Err(Error::InvalidRgbFormat),
            }
        }

        if let Some(hex) = s.strip_prefix('#') {
            let clen = hex.len() / 3;
            if clen > 4 || clen * 3 != hex.len() {
                return Err(Error::InvalidRgbFormat);
            }
            let r = u16::from_str_radix(&hex[..clen], 16)?;
            let g = u16::from_str_radix(&hex[clen..clen * 2], 16)?;
            let b = u16::from_str_radix(&hex[clen * 2..], 16)?;
            let shift = (4 - clen) * 4;
            Ok(Self::new(r, g, b).map(|a| a << shift))
        } else if let Some(phex) = s.strip_prefix("rgb:") {
            let [r, g, b] = &phex.split('/').collect::<Vec<_>>()[..] else {
                return Err(Error::InvalidRgbFormat);
            };
            let r = interpolate(r)?;
            let g = interpolate(g)?;
            let b = interpolate(b)?;
            Ok(Self::new(r, g, b))
        } else {
            Err(Error::InvalidRgbFormat)
        }
    }
}
