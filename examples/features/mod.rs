use termal::{Result, codes as code, gradient, write_gradient};

pub mod codes;
pub mod image;
pub mod raw;

pub fn show_write_gradient() -> Result<()> {
    let mut buf = code::CLEAR.to_string();

    let text = "gradient";
    write_gradient(
        &mut buf,
        text,
        text.len(),
        (0xFD, 0xB9, 0x75),
        (0x57, 0x9B, 0xDF),
    );

    println!("{buf}");

    Ok(())
}

pub fn show_gradient() -> Result<()> {
    let mut buf = code::CLEAR.to_string();
    buf += &gradient("gradient", (0xFD, 0xB9, 0x75), (0x57, 0x9B, 0xDF));
    println!("{buf}");

    Ok(())
}
