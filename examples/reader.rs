use termal::{
    error::Result,
    raw::{
        disable_raw_mode, enable_raw_mode, events::KeyCode, readers::TermRead,
        Terminal,
    },
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    start()?;

    disable_raw_mode()?;

    Ok(())
}

fn start() -> Result<()> {
    let mut term = Terminal::new();
    let mut reader = TermRead::new(&mut term, KeyCode::Enter);
    reader.set_prompt("type here: ");
    println!("\n\rread: {}\r", reader.edit("old text", None)?);
    Ok(())
}
