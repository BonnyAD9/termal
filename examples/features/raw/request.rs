use std::time::Duration;

use termal::{
    Result, codes,
    raw::{Terminal, request},
};

pub fn show_device_attributes() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::device_attributes(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_status_report() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::status_report(Duration::from_millis(100), true)?;
    println!("{res:#?}");
    Ok(())
}
