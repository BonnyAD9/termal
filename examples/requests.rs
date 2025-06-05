use std::time::Duration;

use termal::{
    Result, Rgb, codes,
    raw::events::{Event, Status},
};

pub fn main() -> Result<()> {
    let color: Option<Rgb<u16>> = termal::raw::request(
        codes::REQUEST_DEFAULT_BG_COLOR,
        Duration::from_millis(100),
        |e| match e {
            Event::Status(Status::DefaultBgColor(c)) => Some(c),
            _ => None,
        },
    )?;

    if let Some(c) = color {
        println!("#{:02X}{:02X}{:02X}", c.r() >> 8, c.g() >> 8, c.b() >> 8);
    } else {
        println!("No response.");
    }

    Ok(())
}
