use minlin::Vec3;

use crate::Error;

/// Single RGB pixel.
pub type Rgb<T = u8> = Vec3<T>;

/// Generate rgb value from hex.
///
/// `rgb` is hex number representing the hex color. Note that it must be all
/// six digits. The MSB is ignored.
///
/// # Examples
/// ```
/// use termal_core::{Rgb, rgb};
///
/// assert_eq!(rgb(0x123456), Rgb::new(0x12, 0x34, 0x56));
/// ```
pub fn rgb(rgb: u32) -> Rgb {
    Rgb::new(
        ((rgb & 0xff0000) >> 16) as u8,
        ((rgb & 0xff00) >> 8) as u8,
        (rgb & 0xff) as u8,
    )
}

/// Extension trait for [`Rgb`] to convert it to ansi color string.
///
/// The ansi color string has the format `rgb:R/G/B` where `R`, `G` and `B`
/// are Red, Green and Blue components in hex with 1 to 4 digits (depending
/// on the size of the type).
pub trait ToAnsiColorStr {
    /// Convert the color to ansi color string.
    ///
    /// The ansi color string has the format `rgb:R/G/B` where `R`, `G` and `B`
    /// are Red, Green and Blue components in hex with 1 to 4 digits (depending
    /// on the size of the type).
    ///
    /// # Example
    ///
    /// ```
    /// use termal_core::{Rgb, ToAnsiColorStr};
    ///
    /// let rgb: Rgb = (0x11, 0x33, 0x55).into();
    /// assert_eq!(rgb.to_ansi_color_str(), "rgb:1/3/5");
    ///
    /// let rgb: Rgb = (0x12, 0x34, 0x56).into();
    /// assert_eq!(rgb.to_ansi_color_str(), "rgb:12/34/56");
    ///
    /// let rgb: Rgb<u16> = (0x1231, 0x3453, 0x5675).into();
    /// assert_eq!(rgb.to_ansi_color_str(), "rgb:123/345/567");
    ///
    /// let rgb: Rgb<u16> = (0x1234, 0x3456, 0x5678).into();
    /// assert_eq!(rgb.to_ansi_color_str(), "rgb:1234/3456/5678");
    /// ```
    fn to_ansi_color_str(&self) -> String;
}

/// Extension trait for [`Rgb`] to parse ansi color string.
///
/// The ansi color string has the format `rgb:R/G/B` or `#RGB` where `R`,
/// `G` and `B` are Red, Green and Blue components in hex with 1 to 4
/// digits (depending on the size of the type).
pub trait FromAnsiColorStr: Sized {
    /// Convert from ansi color string to color.
    ///
    /// The ansi color string has the format `rgb:R/G/B` or `#RGB` where `R`,
    /// `G` and `B` are Red, Green and Blue components in hex with 1 to 4
    /// digits (depending on the size of the type). Colors with fewer digits
    /// are scaled when using `rgb:`, but not when using `#` as per
    /// xparsecolor.
    ///
    /// # Examples
    /// ```
    /// use termal_core::{Rgb, FromAnsiColorStr};
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("rgb:1/3/5").unwrap(),
    ///     Rgb::new(0x1111, 0x3333, 0x5555)
    /// );
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("rgb:12/34/56").unwrap(),
    ///     Rgb::new(0x1212, 0x3434, 0x5656)
    /// );
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("rgb:123/345/567").unwrap(),
    ///     Rgb::new(0x1231, 0x3453, 0x5675)
    /// );
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("rgb:1234/3456/5678").unwrap(),
    ///     Rgb::new(0x1234, 0x3456, 0x5678)
    /// );
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("#135").unwrap(),
    ///     Rgb::new(0x1000, 0x3000, 0x5000)
    /// );
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("#123456").unwrap(),
    ///     Rgb::new(0x1200, 0x3400, 0x5600)
    /// );
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("#123345567").unwrap(),
    ///     Rgb::new(0x1230, 0x3450, 0x5670)
    /// );
    ///
    /// assert_eq!(
    ///     Rgb::from_ansi_color_str("#123434565678").unwrap(),
    ///     Rgb::new(0x1234, 0x3456, 0x5678)
    /// );
    /// ```
    ///
    /// # Errors
    /// - returns [`Error::InvalidRgbFormat`] if the format is invalid.
    fn from_ansi_color_str(s: &str) -> Result<Self, Error>;
}

impl ToAnsiColorStr for Rgb {
    fn to_ansi_color_str(&self) -> String {
        let (r, g, b) = (*self).into();
        if self.are_all(|c| c.overflowing_shr(4).0 == (c & 0xf)) {
            format!(
                "rgb:{:x}/{:x}/{:x}",
                r.overflowing_shr(4).0,
                g.overflowing_shr(4).0,
                b.overflowing_shr(4).0
            )
        } else {
            format!("rgb:{r:02x}/{g:02x}/{b:02x}")
        }
    }
}

impl ToAnsiColorStr for Rgb<u16> {
    fn to_ansi_color_str(&self) -> String {
        let (r, g, b) = (*self).into();
        if self.are_all(|c| c.overflowing_shr(8).0 == (c & 0xff)) {
            self.cast::<u8>().to_ansi_color_str()
        } else if self.are_all(|c| c.overflowing_shr(12).0 == (c & 0xf)) {
            format!(
                "rgb:{:03x}/{:03x}/{:03x}",
                r.overflowing_shr(4).0,
                g.overflowing_shr(4).0,
                b.overflowing_shr(4).0
            )
        } else {
            format!("rgb:{r:04x}/{g:04x}/{b:04x}")
        }
    }
}

impl FromAnsiColorStr for Rgb<u16> {
    fn from_ansi_color_str(s: &str) -> Result<Self, Error> {
        fn interpolate(col: &str) -> Result<u16, Error> {
            let c = u16::from_str_radix(col, 16)?;
            match col.len() {
                1 => {
                    let c = c | (c << 4);
                    Ok(c | (c << 8))
                }
                2 => Ok(c | (c << 8)),
                3 => Ok((c << 4) | c.overflowing_shr(8).0),
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
