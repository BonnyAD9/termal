use minlin::Vec3;

use crate::error::Error;

/// Single RGB pixel.
pub type Rgb<T = u8> = Vec3<T>;

pub trait ToColorStr {
    fn to_color_str(&self) -> String;
}

pub trait FromColorStr: Sized {
    fn from_color_str(s: &str) -> Result<Self, Error>;
}

impl ToColorStr for Rgb {
    fn to_color_str(&self) -> String {
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

impl ToColorStr for Rgb<u16> {
    fn to_color_str(&self) -> String {
        let (r, g, b) = (*self).into();
        if self.are_all(|c| c.overflowing_shr(8).0 == (c & 0xff)) {
            self.cast::<u8>().to_color_str()
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

impl FromColorStr for Rgb<u16> {
    fn from_color_str(s: &str) -> Result<Self, Error> {
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
