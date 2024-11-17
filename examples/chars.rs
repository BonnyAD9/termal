use std::io::{self, stdout, Read, Write};

use termal::{
    codes,
    error::Result,
    raw::{disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<()> {
    print!("{}{}\x1b[?1004h", codes::REQUEST_DEVICE_ATTRIBUTES, "");
    _ = stdout().flush();
    enable_raw_mode()?;

    start()?;

    disable_raw_mode()?;
    print!("{}\x1b[?1004l", "");
    _ = stdout().flush();

    Ok(())
}

fn start() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    const CNT: usize = 100;
    let mut buf: [u8; CNT] = [0; CNT];

    loop {
        let cnt = stdin.read(&mut buf)?;
        stdout.flush()?;
        for byte in &buf[..cnt] {
            let chr = *byte as char;
            if chr == '\x03' {
                return Ok(());
            }
            if chr == '\x1b' {
                print!("ESC");
            } else if chr.is_ascii_control() || *byte > 0x7F {
                print!("0x{byte:02X}");
            } else {
                print!("{chr}");
            }
            print!(" ");
        }
        print!("\n\r");
        stdout.flush()?;
    }
}
