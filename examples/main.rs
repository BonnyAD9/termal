use std::io::{self, Read, Write};

use termal::{
    error::Result,
    raw::{
        disable_raw_mode, enable_raw_mode,
        events::{Event, Key, KeyCode, Modifiers},
        term_size, Terminal,
    },
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    start()?;

    disable_raw_mode()?;
    // io::stdout().write(b"\x1b[?1049l")?;
    // io::stdout().flush()?;

    Ok(())
}

fn start() -> Result<()> {
    println!("{:?}\r", term_size()?);
    _reader()
    //_keys()
    //_chars()
    //Ok(())
}

fn _reader() -> Result<()> {
    let mut term = Terminal::new();
    println!("\n\rreaded: {}\r", term.read_line()?);
    Ok(())
}

fn _chars() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    const CNT: usize = 100;
    let mut buf: [u8; CNT] = [0; CNT];
    stdout.write(b"\x1b[c")?;
    stdout.flush()?;

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
            } else if chr.is_ascii_control() {
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

fn _keys() -> Result<()> {
    let mut stdout = io::stdout();
    //stdout.write(b"\x1b]60;?\x9c")?;
    stdout.write(b"\x1b[c")?;
    stdout.flush()?;
    let mut term = Terminal::new();

    loop {
        let key = term.read()?;
        if matches!(
            key,
            Event::KeyPress(Key {
                code: KeyCode::Char('c'),
                modifiers: Modifiers::CONTROL,
                ..
            })
        ) {
            return Ok(());
        }
        print!("{key:?}\n\r");
        stdout.flush()?;
    }
}
