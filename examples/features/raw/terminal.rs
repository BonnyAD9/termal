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
