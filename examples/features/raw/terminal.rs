use termal::{
    Result, codes,
    raw::{Terminal, disable_raw_mode, enable_raw_mode},
};

pub fn show_stdio() -> Result<()> {
    let mut term = Terminal::stdio();

    term.flushed(codes::CLEAR)?;
    term.println("This will print to stdout.")?;
    enable_raw_mode()?;
    let data = term.prompt("Enter data to stdin: ")?;
    disable_raw_mode()?;
    term.println(format!("\nData you entered to stdin: {data}"))?;

    Ok(())
}

pub fn show_read_byte() -> Result<()> {
    let mut term = Terminal::stdio();

    term.flushed(codes::CLEAR)?;
    term.flushed("Enter single byte: ")?;
    enable_raw_mode()?;
    let byte = term.read_byte()?;
    disable_raw_mode()?;
    term.println(format!("\nYou entered byte `0x{byte:2x}`."))?;
    term.println(format!(
        "It coresponds to the character `{}`.",
        byte as char
    ))?;

    Ok(())
}
