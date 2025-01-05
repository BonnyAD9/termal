use termal::{
    error::Result,
    raw::{disable_raw_mode, enable_raw_mode, readers::prompt},
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    start()?;

    disable_raw_mode()?;

    Ok(())
}

fn start() -> Result<()> {
    println!("\n\rread: {}\r", prompt("type\nhere: ")?);
    Ok(())
}
