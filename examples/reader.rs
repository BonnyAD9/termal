use termal::{
    Result, codes,
    raw::{enable_raw_mode, readers::prompt},
    reset_terminal,
};

fn main() -> Result<()> {
    print!("{}", codes::ENABLE_BRACKETED_PASTE_MODE);
    enable_raw_mode()?;

    start()?;

    reset_terminal();

    Ok(())
}

fn start() -> Result<()> {
    println!("\n\rread: {}\r", prompt("type\nhere: ")?);
    Ok(())
}
