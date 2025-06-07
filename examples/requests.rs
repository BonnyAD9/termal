use std::time::Duration;

use termal::{Result, raw::request};

pub fn main() -> Result<()> {
    let c = request::default_bg_color(Duration::from_millis(100))?;

    println!("#{:02X}{:02X}{:02X}", c.r() >> 8, c.g() >> 8, c.b() >> 8);

    Ok(())
}
