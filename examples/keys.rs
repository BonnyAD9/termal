use std::io::{self, Write};

use termal::{
    codes,
    error::Result,
    raw::{
        StdioProvider, Terminal, disable_raw_mode, enable_raw_mode,
        events::{AmbigousEvent, AnyEvent, Event, Key, KeyCode, Modifiers},
    },
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    print!("{}", codes::REQUEST_SELECTION);
    _ = io::stdout().flush();

    start()?;

    print!("");
    disable_raw_mode()?;
    _ = io::stdout().flush();

    Ok(())
}

fn start() -> Result<()> {
    let mut stdout = io::stdout();
    let mut term = Terminal::<StdioProvider>::default();

    loop {
        let key = term.read_ambigous()?;
        if matches!(
            key,
            AmbigousEvent {
                event: AnyEvent::Known(Event::KeyPress(Key {
                    code: KeyCode::Char('c'),
                    modifiers: Modifiers::CONTROL,
                    ..
                })),
                ..
            }
        ) {
            return Ok(());
        }
        print!("{key:?}\n\r");
        stdout.flush()?;
    }
}
