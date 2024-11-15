use termal::{
    error::Result,
    raw::{disable_raw_mode, enable_raw_mode, Terminal},
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    start()?;

    disable_raw_mode()?;

    Ok(())
}

fn start() -> Result<()> {
    let mut term = Terminal::new();
    println!("\n\rreaded: {}\r", term.read_line()?);
    Ok(())
}
