use termal::{codes, error::Result, raw::enable_raw_mode, reset_terminal};

pub fn show_backspace() -> Result<()> {
    let mut buf = String::new();

    buf += "Some test";
    buf.push(codes::BACKSPACE);
    buf.push(codes::BACKSPACE);
    buf += "x";

    println!("{buf}");

    Ok(())
}

pub fn show_htab() -> Result<()> {
    println!("1\t: number");
    println!("hello\t: greeting");
    Ok(())
}

pub fn show_newline() -> Result<()> {
    println!("normal:");
    println!("one\ntwo");

    println!("raw:");
    enable_raw_mode()?;
    println!("one\ntwo\r");

    reset_terminal();

    Ok(())
}
