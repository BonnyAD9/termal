use std::io::{self, Write};

use termal::{
    codes,
    error::Result,
    raw::{
        disable_raw_mode, enable_raw_mode,
        events::{AmbigousEvent, AnyEvent, Event, Key, KeyCode, Modifiers},
        StdioProvider, Terminal,
    },
};

fn main() -> Result<()> {
    print!("{}", codes::ENABLE_BRACKETED_PASTE_MODE);
    _ = io::stdout().flush();
    enable_raw_mode()?;

    start()?;

    disable_raw_mode()?;
    print!("{}", codes::DISABLE_BRACKETED_PASTE_MODE);
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
