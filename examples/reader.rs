use termal::{
    Result, codes,
    raw::{Terminal, enable_raw_mode},
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
    let mut history = vec![];
    let mut term = Terminal::stdio();
    while let Some(s) = term.prompt_to_history(&mut history, "type\nhere: ")? {
        println!("\n\rread: {}\r", s);
    }
    println!("\r");
    Ok(())
}
