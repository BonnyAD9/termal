use termal::{codes, error::Result};

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
    let mut buf = String::new();

    buf += "1";
    buf.push(codes::HTAB);
    buf += ": number\n";

    buf += "hello";
    buf.push(codes::HTAB);
    buf += ": greeting";

    println!("{buf}");

    Ok(())
}
