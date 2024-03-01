use std::io::{self, Read, Write};

use termal::raw::{disable_raw_mode, enable_raw_mode};
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    start()?;

    disable_raw_mode()?;

    Ok(())
}

fn start() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    const CNT: usize = 100;
    let mut buf: [u8;CNT] = [0;CNT];
    //stdout.write(b"\x1b]60;?\x9c")?;
    stdout.flush()?;

    loop {
        let cnt = stdin.read(&mut buf)?;
        print!("cnt:{cnt}: ");
        stdout.flush()?;
        for byte in &buf[..cnt] {
            let chr = *byte as char;
            if chr == 'Q' {
                return Ok(());
            }
            if chr == '\x1b' {
                print!("ESC");
            } else {
                print!("{chr}:{byte}");
            }
            print!(" ");
        }
        print!("\n\r");
        stdout.flush()?;
    }
}
